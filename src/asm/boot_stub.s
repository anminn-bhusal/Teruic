; src/asm/boot_stub.s - Low-Level CPU Control for Teruic OS
.global asm_cpu_halt
.global asm_read_tsc
.global asm_sys_entry

.section .text

; Halts the CPU directly using Assembly
asm_cpu_halt:
    cli
1:  hlt
    jmp 1b

; Reads CPU Time Stamp Counter into rax
asm_read_tsc:
    rdtsc
    shl $32, %rdx
    or %rdx, %rax
    ret

; Minimal Syscall Entry point for Native Executables
asm_sys_entry:
    ; RDI = Syscall Number, RSI = Arg1 (Buffer pointer), RDX = Arg2 (Length)
    cmp $1, %rdi            ; Syscall 1: Print String
    je sys_print
    ret

sys_print:
    ; Transfers control back to C/Rust print routine
    ret