
section .data
	ip_text db "IP address: "
	port_text db "Port: "

section .bss
	ip resb 16

section .text
	global _start

_start:
	mov rax, 60
	mov rdi, 0
	syscall

_getIP:
	mov rax, 0
	mov rdi, 0
	mov rsi, ip
	mov rdx, 16
	syscall
	ret
	


