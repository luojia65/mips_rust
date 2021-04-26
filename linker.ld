OUTPUT_ARCH(mips)

ENTRY(entry) 

MEMORY {
	axi_bram_ctrl_1 (rw) 	: ORIGIN = 0, LENGTH = 64K
	axi_uart16550_0 (rw)	: ORIGIN = 0x10400000, LENGTH = 64K
	axi_gpio_0 (rw) 		: ORIGIN = 0x10600000, LENGTH = 64K
	axi_bram_ctrl_0 (rx) 	: ORIGIN = 0x1fc00000, LENGTH = 8K
}

BASE_ADDRESS = 0x1fc00000;

SECTIONS {
	. = 0x00000000; 

	data_start = .;
	.data : {
		* (.data, .data.*)
	} > axi_bram_ctrl_1 

	.bss (NOLOAD) : {
		* (.bss .bss.*)
		* (.sbss .sbss.*)
	} > axi_bram_ctrl_1 

	. = BASE_ADDRESS;
	text_start = .;
	.text : AT(BASE_ADDRESS) {
		/* put `entry` first */
		* (.text.entry)
		* (.text .text.*)
	} > axi_bram_ctrl_0 

	.rodata : {
		* (.rodata .rodata.*)
	} > axi_bram_ctrl_0 

	/DISCARD/ : {
		* (COMMON)
	}
}
