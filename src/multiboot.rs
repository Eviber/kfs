// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Header-layout
// 0	u32	magic	required
// 4	u32	flags	required
// 8	u32	checksum	required
// 12	u32	header_addr	if flags[16] is set
// 16	u32	load_addr	if flags[16] is set
// 20	u32	load_end_addr	if flags[16] is set
// 24	u32	bss_end_addr	if flags[16] is set
// 28	u32	entry_addr	if flags[16] is set
// 32	u32	mode_type	if flags[2] is set
// 36	u32	width	if flags[2] is set
// 40	u32	height	if flags[2] is set
// 44	u32	depth	if flags[2] is set
#[repr(C, align(8))]
pub struct Header {
    magic: u32,
    flags: u32,
    checksum: u32,
}

impl Header {
    pub const fn new() -> Self {
        let magic: u32 = 0x1BADB002;
        let flags: u32 = 1;
        let checksum: u32 = magic.wrapping_add(flags).wrapping_neg();

        Header {
            magic,
            flags,
            checksum,
        }
    }
}
