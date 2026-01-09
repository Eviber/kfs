#![no_std]
#![no_main]
#![allow(clippy::needless_range_loop)]

use core::{
    arch::{asm, naked_asm},
    mem::MaybeUninit,
};
use mutex::Mutex;

mod io;
mod multiboot;
mod mutex;

#[used]
#[unsafe(link_section = ".multiboot")]
static MULTIBOOT2_HEADER: multiboot::Header = multiboot::Header::new();

const KERNEL_STACK_SIZE: usize = 0x1000 * 32;
static mut KERNEL_STACK: MaybeUninit<[u8; KERNEL_STACK_SIZE]> = MaybeUninit::uninit();

static VGA_BUFFER: Mutex<io::VgaBuffer> = unsafe { Mutex::new(io::VgaBuffer::new()) };

macro_rules! printk {
    ($($arg:tt)*) => {
        _ = core::fmt::Write::write_fmt(&mut *VGA_BUFFER.lock(), core::format_args!($($arg)*))
    };
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
extern "C" fn _start() {
    naked_asm!(
        "
        lea esp, [{stack_base} + {stack_size}]
        and esp, 0xfffffff0
        call {main}
        ",
        main = sym main,
        stack_base = sym KERNEL_STACK,
        stack_size = const KERNEL_STACK_SIZE,
    )
}

extern "C" fn main() -> ! {
    init_gdt();
    funny_42();
    print_stack();
    io::qemu_shutdown()
}

fn print_stack() {
    let esp: usize;
    // Safety: nothing is touched, we only get the value of ESP
    unsafe {
        asm!("mov {}, esp", out (reg) esp, options(nostack, nomem, preserves_flags));
    }
    let mut esp = esp as *const u8;
    printk!("Stack dump from {:p}:\n", esp);
    const STACK_END: *const u8 = unsafe { core::ptr::addr_of!(KERNEL_STACK).add(1).cast() };
    if !esp.addr().is_multiple_of(16) {
        printk!("{:p}:", esp);
        if !esp.addr().is_multiple_of(4) {
            printk!(" ");
        }
    }
    while esp < STACK_END {
        let byte = unsafe { esp.read_volatile() };
        if esp.addr().is_multiple_of(16) {
            printk!("{:p}: ", esp);
        } else if esp.addr().is_multiple_of(4) {
            printk!(" ");
        }
        printk!("{:02x}", byte);
        esp = unsafe { esp.add(1) };
        if esp.addr().is_multiple_of(16) {
            printk!("\n");
        }
    }
    // printk!("{:p}\n", esp);
}

fn init_gdt() {
    // https://docs.rs/x86_64/latest/src/x86_64/structures/gdt.rs.html#543
    const GDT: [u64; 7] = [
        0,                  // https://wiki.osdev.org/GDT_Tutorial#Basics
        0x00cf9b000000ffff, // KERNEL_CODE  - DPL 0 + executable + readable
        0x00cf93000000ffff, // KERNEL_DATA  - DPL 0 + readable   + writable
        0x00cf93000000ffff, // KERNEL_STACK - DPL 0 + readable   + writable
        0x00cffb000000ffff, // USER_CODE    - DPL 3 + executable + readable
        0x00cff3000000ffff, // USER_DATA    - DPL 3 + readable   + writable
        0x00cff3000000ffff, // USER_STACK   - DPL 3 + readable   + writable
    ];
    #[repr(C, packed)]
    struct Gdtr {
        size: u16,
        address: usize,
    }
    const ADDRESS: usize = 0x00000800;
    unsafe {
        core::ptr::without_provenance_mut::<[u64; 7]>(ADDRESS).write_volatile(GDT);
        let gdtr = Gdtr {
            size: size_of::<[u64; 7]>() as u16 - 1,
            address: ADDRESS,
        };
        const KERNEL_CODE_SELECTOR: u16 = 8;
        const KERNEL_DATA_SELECTOR: u16 = 8 * 2;
        const KERNEL_STACK_SELECTOR: u16 = 8 * 3;
        asm!("lgdt [{gdtr}]", gdtr = in (reg) &gdtr, options(readonly, nostack, preserves_flags));
        asm!(
            "mov {tmp:x}, {kernel_data}
            mov ds, {tmp:x}
            mov es, {tmp:x}
            mov fs, {tmp:x}
            mov gs, {tmp:x}
            mov {tmp:x}, {kernel_stack}
            mov ss, {tmp:x}
            ",
            tmp = lateout(reg) _,
            kernel_data = const KERNEL_DATA_SELECTOR,
            kernel_stack = const KERNEL_STACK_SELECTOR,
            options(nostack, preserves_flags)
        );
        asm!(
            "jmp ${kernel_code}, $2f;
            2:",
            kernel_code = const KERNEL_CODE_SELECTOR,
            options(att_syntax)
        );
    }
}

fn funny_42() {
    const ASCII_42: &str = include_str!("42.txt");

    // Initialize the VGA buffer.
    {
        let mut lock = VGA_BUFFER.lock();
        lock.clear();
        lock.set_cursor_shape(0, 16);
        lock.set_visual_cursor_pos(0, 0);
    }

    let mut d = 0;
    'a: loop {
        for _ in 0..5_000 {
            let mut row = 0;
            let mut col = 27;
            for c in ASCII_42.trim_ascii_end().bytes() {
                if c == b'\n' {
                    row += 1;
                    col = 27;
                    continue;
                }
                let color = ((col / 2 + row + d) & 0xF) as u8;
                VGA_BUFFER.lock().set_color(color);
                VGA_BUFFER.lock().write_at(col, row, c);
                col += 1;
            }
            if VGA_BUFFER.lock().get_char().is_some() {
                break 'a;
            }
        }
        d = d.wrapping_add(1);
    }
}

#[panic_handler]
fn crash_and_burn(info: &core::panic::PanicInfo) -> ! {
    // Safety: At this point we're crashing down anyways.
    // Might as well try to get some insights.
    let mut lock = unsafe { VGA_BUFFER.lock_unchecked() };
    _ = core::fmt::Write::write_fmt(
        &mut *lock,
        core::format_args!("{info}\nPress ESC to shutdown"),
    );
    while lock.get_kb_data() != Some(0x01) {
        core::hint::spin_loop();
    }
    io::qemu_shutdown()
}
