;;! target = "x86_64"
;;! test = "winch"

(module
    (func (param i64) (param i64) (result i32)
        (local.get 0)
        (local.get 1)
        (i64.gt_s)
    )
)
;; wasm[0]::function[0]:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    8(%rdi), %r11
;;       movq    0x10(%r11), %r11
;;       addq    $0x20, %r11
;;       cmpq    %rsp, %r11
;;       ja      0x58
;;   1c: movq    %rdi, %r14
;;       subq    $0x20, %rsp
;;       movq    %rdi, 0x18(%rsp)
;;       movq    %rsi, 0x10(%rsp)
;;       movq    %rdx, 8(%rsp)
;;       movq    %rcx, (%rsp)
;;       movq    (%rsp), %rax
;;       movq    8(%rsp), %rcx
;;       cmpq    %rax, %rcx
;;       movl    $0, %ecx
;;       setg    %cl
;;       movl    %ecx, %eax
;;       addq    $0x20, %rsp
;;       popq    %rbp
;;       retq
;;   58: ud2
