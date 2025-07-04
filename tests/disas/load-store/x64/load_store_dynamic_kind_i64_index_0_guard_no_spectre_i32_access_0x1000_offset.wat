;;! target = "x86_64"
;;! test = "compile"
;;! flags = " -C cranelift-enable-heap-access-spectre-mitigation=false -W memory64 -O static-memory-maximum-size=0 -O static-memory-guard-size=0 -O dynamic-memory-guard-size=0"

;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
;; !!! GENERATED BY 'make-load-store-tests.sh' DO NOT EDIT !!!
;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

(module
  (memory i64 1)

  (func (export "do_store") (param i64 i32)
    local.get 0
    local.get 1
    i32.store offset=0x1000)

  (func (export "do_load") (param i64) (result i32)
    local.get 0
    i32.load offset=0x1000))

;; wasm[0]::function[0]:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    0x40(%rdi), %r8
;;       subq    $0x1004, %r8
;;       cmpq    %r8, %rdx
;;       ja      0x29
;;   18: movq    0x38(%rdi), %r11
;;       movl    %ecx, 0x1000(%r11, %rdx)
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;   29: ud2
;;
;; wasm[0]::function[1]:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    0x40(%rdi), %r8
;;       subq    $0x1004, %r8
;;       cmpq    %r8, %rdx
;;       ja      0x69
;;   58: movq    0x38(%rdi), %r11
;;       movl    0x1000(%r11, %rdx), %eax
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;   69: ud2
