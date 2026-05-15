.global _add
_add:
    push rbp
    mov rbp, rsp
    sub rsp, 256
    mov [rbp - 8], rdi
    mov [rbp - 16], rsi
    mov rax, [rbp - 8]
    mov rbx, [rbp - 16]
    add rax, rbx
    mov [rbp - 72], rax
    mov rax, [rbp - 72]
    mov rsp, rbp
    pop rbp
    ret

.global _main
_main:
    push rbp
    mov rbp, rsp
    sub rsp, 256
    mov rdi, 3
    mov rsi, 5
    call _add
    mov [rbp - 72], rax
    mov rax, [rbp - 72]
    mov rbx, 10
    imul rax, rbx
    mov [rbp - 80], rax
    mov rax, [rbp - 80]
    mov rsp, rbp
    pop rbp
    ret

