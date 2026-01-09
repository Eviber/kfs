#![no_std]
#![no_main]
#![allow(clippy::needless_range_loop)]

use core::{arch::naked_asm, mem::MaybeUninit};
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
        call {main}
        ",
        main = sym main,
        stack_base = sym KERNEL_STACK,
        stack_size = const KERNEL_STACK_SIZE,
    )
}

extern "C" fn main() -> ! {
    const ASCII_42: &str = include_str!("42.txt");

    // Initialize the VGA buffer.
    {
        let mut lock = VGA_BUFFER.lock();
        lock.clear();
        lock.set_cursor_shape(0, 16);
        lock.set_visual_cursor_pos(0, 0);
    }

    let mut d = 0;
    while VGA_BUFFER.lock().get_kb_data() != Some(0x0f) {
        let mut row = 8;
        let mut col = 27;
        for c in ASCII_42.trim_ascii_end().bytes() {
            if c == b'\n' {
                row += 1;
                col = 27;
                continue;
            }
            if !c.is_ascii_whitespace() {
                let color = ((col / 2 + row + d) & 0xF) as u8;
                VGA_BUFFER.lock().set_color(color);
                VGA_BUFFER.lock().write_at(col, row, c);
            }
            col += 1;
        }
        d = d.wrapping_add(1);
        printk!("Hello {}!\n", d);
        for _ in 0..1_000_000 {
            core::hint::spin_loop()
        }
    }
    io::qemu_shutdown()
}

#[panic_handler]
fn crash_and_burn(_: &core::panic::PanicInfo) -> ! {
    io::qemu_shutdown()
}
