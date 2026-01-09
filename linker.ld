ENTRY(_start)

PHDRS
{
    text    PT_LOAD FLAGS(5);
    rodata  PT_LOAD FLAGS(4);
    data    PT_LOAD FLAGS(6);
}

SECTIONS
{
	. = 2M;

	.text : ALIGN(4K)
	{
		KEEP(*(.multiboot))
		*(.text)
	} : text

	/* Read-only data. */
	.rodata : ALIGN(4K)
	{
		*(.rodata)
	} : rodata

	/* Read-write data (initialized) */
	.data : ALIGN(4K)
	{
		*(.data)
	} : data

	/* Read-write data (uninitialized) and stack */
	.bss : ALIGN(4K)
	{
		*(COMMON)
		*(.bss)
	} : data
}
