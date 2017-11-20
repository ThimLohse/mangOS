global long_mode_start

section .text
bits 64
long_mode_start:
	;;  load 0 into all data segment registers
	mov ax, 0
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	extern rust_main	;new
	call rust_main		;new

	;;  print `OKAY` to screen
	mov rax, 0x5f595f415f4b5f4f
	mov qword [0xb8000], rax
	hlt
