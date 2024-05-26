
section .data
	ip_input db "IP address: "
	port_input db "Port: "

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
	


