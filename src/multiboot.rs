/// The Multiboot header structure.
///
/// [https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Header-layout]
/// ```markdown
/// | Offset  | Type  | Field Name    | Note                |
/// |---------|-------|---------------|---------------------|
/// | 0       | u32   | magic         | required            |
/// | 4       | u32   | flags         | required            |
/// | 8       | u32   | checksum      | required            |
/// | 12      | u32   | header_addr   | if flags:16 is set  |
/// | 16      | u32   | load_addr     | if flags:16 is set  |
/// | 20      | u32   | load_end_addr | if flags:16 is set  |
/// | 24      | u32   | bss_end_addr  | if flags:16 is set  |
/// | 28      | u32   | entry_addr    | if flags:16 is set  |
/// | 32      | u32   | mode_type     | if flags:2 is set   |
/// | 36      | u32   | width         | if flags:2 is set   |
/// | 40      | u32   | height        | if flags:2 is set   |
/// | 44      | u32   | depth         | if flags:2 is set   |
/// ```
#[repr(C, align(8))]
pub struct Header {
    /// The field `magic` is the magic number identifying the header, which must be the hexadecimal value 0x1BADB002.
    magic: u32,
    /// The field `flags` specifies features that the OS image requests or requires of an boot loader. Bits 0-15 indicate requirements; if the boot loader sees any of these bits set but doesn’t understand the flag or can’t fulfill the requirements it indicates for some reason, it must notify the user and fail to load the OS image. Bits 16-31 indicate optional features; if any bits in this range are set but the boot loader doesn’t understand them, it may simply ignore them and proceed as usual. Naturally, all as-yet-undefined bits in the `flags` word must be set to zero in OS images. This way, the `flags` fields serves for version control as well as simple feature selection.
    /// * If bit 0 in the `flags` word is set, then all boot modules loaded along with the operating system must be aligned on page (4KB) boundaries. Some operating systems expect to be able to map the pages containing boot modules directly into a paged address space during startup, and thus need the boot modules to be page-aligned.
    /// * If bit 1 in the `flags` word is set, then information on available memory via at least the `mem_*` fields of the Multiboot information structure (see Boot information format) must be included. If the boot loader is capable of passing a memory map (the `mmap_*` fields) and one exists, then it may be included as well.
    /// * If bit 2 in the `flags` word is set, information about the video mode table (see Boot information format) must be available to the kernel.
    /// * If bit 16 in the `flags` word is set, then the fields at offsets 12-28 in the Multiboot header are valid, and the boot loader should use them instead of the fields in the actual executable header to calculate where to load the OS image. This information does not need to be provided if the kernel image is in ELF format, but it must be provided if the images is in a.out format or in some other format. Compliant boot loaders must be able to load images that either are in ELF format or contain the load address information embedded in the Multiboot header; they may also directly support other executable formats, such as particular a.out variants, but are not required to.
    flags: u32,
    /// The field `checksum` is a 32-bit unsigned value which, when added to the other magic fields (i.e. `magic` and `flags`), must have a 32-bit unsigned sum of zero.
    checksum: u32,
}

impl Header {
    /// Creates a new Multiboot header requesting page-aligned modules.
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
