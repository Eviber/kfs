use core::arch::asm;
use core::hint::unreachable_unchecked;

mod vga_chars;

const VGA_BUFFER_ADDRESS: usize = 0xb8000;
const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;

const TAB_SIZE: usize = 4;

pub struct VgaBuffer {
    buffer: &'static mut [u16],
    cursor_x: usize,
    cursor_y: usize,
    current_color: u8,
}

impl VgaBuffer {
    /// Creates the VGA buffer interface.
    ///
    /// # Safety
    /// This function is unsafe because it allows mutable access to the VGA buffer, and Text Mode cursor,
    /// and keyboard controller ports, which may lead to data races if multiple mutable references exist.
    /// As such, the caller must ensure that they have exclusive access to these resources.
    pub unsafe fn new() -> Self {
        const VGA_BUFFER: *mut [u16] = core::ptr::slice_from_raw_parts_mut(
            core::ptr::without_provenance_mut(VGA_BUFFER_ADDRESS),
            VGA_BUFFER_WIDTH * VGA_BUFFER_HEIGHT,
        );

        // SAFETY: The caller must ensure that they have exclusive access to the VGA buffer.
        let buffer = unsafe { &mut *VGA_BUFFER };
        // SAFETY: The caller must ensure that they have exclusive access to the Text Mode cursor.
        let (cursor_x, cursor_y) = unsafe { get_cursor_pos() };
        let current_color = 0x0F; // White on black

        VgaBuffer {
            buffer,
            cursor_x,
            cursor_y,
            current_color,
        }
    }

    /// Clears the VGA buffer by filling it with spaces and default colors.
    pub fn clear(&mut self) {
        self.buffer
            .fill((self.current_color as u16) << 8 | (b' ' as u16));
    }

    /// Writes a byte to the VGA buffer at the specified coordinates with the given color.
    #[inline]
    pub fn write_byte(&mut self, x: usize, y: usize, byte: u8, color: u8) {
        self.buffer[x + y * VGA_BUFFER_WIDTH] = (color as u16) << 8 | (byte as u16);
    }

    /// Writes a byte to the VGA buffer at the specified coordinates using the current color.
    pub fn write_at(&mut self, x: usize, y: usize, byte: u8) {
        self.write_byte(x, y, byte, self.current_color);
    }

    fn newline(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        if self.cursor_y == VGA_BUFFER_HEIGHT {
            self.buffer.copy_within(VGA_BUFFER_WIDTH.., 0);
            self.buffer[VGA_BUFFER_WIDTH * (VGA_BUFFER_HEIGHT - 1)..]
                .fill((self.current_color as u16) << 8);
            self.cursor_y -= 1;
        } else if self.cursor_y > VGA_BUFFER_HEIGHT {
            panic!("NTM");
        }
    }

    pub fn putchar(&mut self, c: char) {
        match c {
            '\n' => {
                self.newline();
            }
            '\r' => {
                self.cursor_x = 0;
            }
            '\t' => {
                self.cursor_x = (self.cursor_x + 1).next_multiple_of(TAB_SIZE);
            }
            _ => {
                const REPLACEMENT_CHARACTER: u8 = vga_chars::from_char('■').unwrap();
                let b = vga_chars::from_char(c).unwrap_or(REPLACEMENT_CHARACTER);
                self.write_at(self.cursor_x, self.cursor_y, b);
                self.cursor_x += 1;
            }
        }
        if self.cursor_x >= VGA_BUFFER_WIDTH {
            self.newline();
        }
    }

    #[inline]
    pub fn set_color(&mut self, color: u8) {
        self.current_color = color;
    }

    pub fn set_cursor_pos(&mut self, x: usize, y: usize) {
        let pos = y * 80 + x;
        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);

            outb(0x3D4, 0x0E);
            outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
        }
        self.cursor_x = x;
        self.cursor_y = y;
    }

    pub fn set_cursor_shape(&mut self, cursor_start: u8, cursor_end: u8) {
        unsafe {
            outb(0x3D4, 0x0A);
            outb(0x3D5, (inb(0x3D5) & 0xC0) | cursor_start);

            outb(0x3D4, 0x0B);
            outb(0x3D5, (inb(0x3D5) & 0xE0) | cursor_end);
        }
    }

    pub fn get_kb_data(&mut self) -> Option<u8> {
        let status = unsafe { inb(0x64) };
        if status & 0x01 == 0 {
            return None;
        }
        let scancode = unsafe { inb(0x60) };
        Some(scancode)
    }
}

unsafe fn get_cursor_pos() -> (usize, usize) {
    let mut pos: usize;
    unsafe {
        outb(0x3D4, 0x0F);
        pos = inb(0x3D5) as usize;

        outb(0x3D4, 0x0E);
        pos |= (inb(0x3D5) as usize) << 8;
    }
    (pos % VGA_BUFFER_WIDTH, pos / VGA_BUFFER_WIDTH)
}

pub fn qemu_shutdown() -> ! {
    unsafe {
        outw(0x604, 0x2000);
        unreachable_unchecked()
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
