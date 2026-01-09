#![no_std]
#![no_main]
#![allow(clippy::needless_range_loop)]

use core::{arch::naked_asm, mem::MaybeUninit};

mod io;
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
    const ASCII_42: &str = include_str!("42.txt");

    // SAFETY: We have exclusive access to the VGA buffer.
    let mut vga_buffer = unsafe { io::VgaBuffer::new() };

    vga_buffer.clear();
    io::set_cursor_shape(0, 16);
    vga_buffer.set_cursor_pos(0, 0);
    let mut d = 0;
    while io::get_kb_data() != Some(0x0f) {
        let mut row = 8;
        let mut col = 27;
        for c in ASCII_42.trim_end().bytes() {
            if c == b'\n' {
                row += 1;
                col = 27;
                continue;
            }
            if !c.is_ascii_whitespace() {
                let color = ((col / 2 + row + d) & 0xF) as u8;
                vga_buffer.write_byte(col, row, c, color);
            }
            col += 1;
        }
        d = d.wrapping_add(1);
        for _ in 0..1_000_000 {
            core::hint::spin_loop()
        }
    }
    io::qemu_shutdown()
}

#[panic_handler]
fn crash_and_burn(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
