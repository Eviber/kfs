#![no_std]
#![no_main]
#![allow(clippy::needless_range_loop)]

use core::arch::asm;
use core::hint::unreachable_unchecked;
use core::{arch::naked_asm, mem::MaybeUninit};

mod multiboot;

#[used]
#[unsafe(link_section = "multiboot")]
static MULTIBOOT2_HEADER: multiboot::Header = multiboot::Header::new();

const KERNEL_STACK_SIZE: usize = 0x1000 * 16;
static mut KERNEL_STACK: MaybeUninit<[u8; KERNEL_STACK_SIZE]> = MaybeUninit::uninit();

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
    const VGA_BUFFER_WIDTH: usize = 80;
    const VGA_BUFFER_HEIGHT: usize = 25;
    const VGA_BUFFER: *mut [u16] = core::ptr::slice_from_raw_parts_mut(
        core::ptr::without_provenance_mut(0xb8000),
        VGA_BUFFER_WIDTH * VGA_BUFFER_HEIGHT,
    );
    const ASCII_42: &str = include_str!("42.txt");

    // SAFETY: We have exclusive access to the VGA buffer.
    let vga_buffer = unsafe { &mut *VGA_BUFFER };

    vga_buffer.fill(0x0F00);
    set_cursor_shape(0, 16);
    let mut d = 0;
    while get_kb_data() != Some(0x0f) {
        let mut row = 8;
        let mut col = 27;
        for c in ASCII_42.trim_end().chars() {
            if c == '\n' {
                row += 1;
                col = 27;
                continue;
            }
            if !c.is_ascii_whitespace() {
                vga_buffer[col + row * VGA_BUFFER_WIDTH] =
                    (((col / 2 + row + d) & 0xF) << 8) as u16 + c as u8 as u16;
            }
            col += 1;
        }
        set_cursor_pos(col, row);
        d = d.wrapping_add(1);
        for _ in 0..1_000_000 {
            core::hint::spin_loop()
        }
    }
    qemu_shutdown()
}

fn qemu_shutdown() -> ! {
    unsafe {
        outw(0x604, 0x2000);
        unreachable_unchecked()
    }
}

fn get_kb_data() -> Option<u8> {
    let status = unsafe { inb(0x64) };
    if status & 0x01 == 0 {
        return None;
    }
    let scancode = unsafe { inb(0x60) };
    Some(scancode)
}

fn set_cursor_shape(cursor_start: u8, cursor_end: u8) {
    unsafe {
        outb(0x3D4, 0x0A);
        outb(0x3D5, (inb(0x3D5) & 0xC0) | cursor_start);

        outb(0x3D4, 0x0B);
        outb(0x3D5, (inb(0x3D5) & 0xE0) | cursor_end);
    }
}

fn set_cursor_pos(x: usize, y: usize) {
    let pos = y * 80 + x;
    unsafe {
        outb(0x3D4, 0x0F);
        outb(0x3D5, (pos & 0xFF) as u8);

        outb(0x3D4, 0x0E);
        outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }
}

/// Read a byte from the specified port.
/// # Safety
/// This function is unsafe because some accesses to certain ports may have
/// side effects that can compromise memory safety.
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    unsafe {
        asm!(
            "in al, dx",
            out("al") ret,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        )
    }
    ret
}

/// Write a byte to the specified port.
/// # Safety
/// This function is unsafe because some accesses to certain ports may have
/// side effects that can compromise memory safety.
unsafe fn outb(port: u16, val: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("al") val,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        )
    }
}

/// Write a word to the specified port.
/// # Safety
/// This function is unsafe because some accesses to certain ports may have
/// side effects that can compromise memory safety.
unsafe fn outw(port: u16, val: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("ax") val,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        )
    }
}

#[panic_handler]
fn crash_and_burn(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
