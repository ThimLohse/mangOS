ENTRY(start)

SECTIONS {
	 . = 1M;

	 /*This is different from the tutorial part 4, and what we were supposed to do before?? WHY?  merge it with .rodata to not waste a whole page for a few bytes of multiboot header?*/
	 /*
	 .boot :
	 {
		KEEP(*(.multiboot_header))
		.=ALIGN(4K);
	 }
	 */

	 .rodata :
	  {
	    /* ensure that the multiboot header is at the beginning */
	    KEEP(*(.multiboot_header))
	    *(.rodata .rodata.*)
	    . = ALIGN(4K);
	  }

	  .text :
	  {
	    *(.text .text.*)
	    . = ALIGN(4K);
	  }

	  .data :
	  {
	    *(.data .data.*)
	    . = ALIGN(4K);
	  }

	  .bss :
	  {
	    *(.bss .bss.*)
	    . = ALIGN(4K);
	  }

	  .got :
	  {
	    *(.got)
	    . = ALIGN(4K);
	  }

	  .got.plt :
	  {
	    *(.got.plt)
	    . = ALIGN(4K);
	  }

	  .data.rel.ro : ALIGN(4K) {
	    *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
	    . = ALIGN(4K);
	  }

	  .gcc_except_table : ALIGN(4K) {
	    *(.gcc_except_table)
	    . = ALIGN(4K);
      	  }
}
	
