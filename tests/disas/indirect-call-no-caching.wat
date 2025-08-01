;;! target = "x86_64"

;; This test checks that we do *not* get the indirect-call caching optimization
;; when it is not enabled, because it is off by default.
;;
;; The key bit in the expectation below is that the call sequence in
;; `u0:3` below goes straight to the bounds-check (v5), lazy-table
;; init (masking of bits with v13), and loading of the funcref fields
;; in block3, with no caching fastpath.

(module
 (table 10 10 funcref)

 (func $f1 (result i32) i32.const 1)
 (func $f2 (result i32) i32.const 2)
 (func $f3 (result i32) i32.const 3)

 (func (export "call_it") (param i32) (result i32)
  local.get 0
  call_indirect (result i32))

 (elem (i32.const 1) func $f1 $f2 $f3))
;; function u0:0(i64 vmctx, i64) -> i32 tail {
;;     gv0 = vmctx
;;     gv1 = load.i64 notrap aligned readonly gv0+8
;;     gv2 = load.i64 notrap aligned gv1+16
;;     stack_limit = gv2
;;
;;                                 block0(v0: i64, v1: i64):
;; @003f                               v3 = iconst.i32 1
;; @0041                               jump block1
;;
;;                                 block1:
;; @0041                               return v3  ; v3 = 1
;; }
;;
;; function u0:1(i64 vmctx, i64) -> i32 tail {
;;     gv0 = vmctx
;;     gv1 = load.i64 notrap aligned readonly gv0+8
;;     gv2 = load.i64 notrap aligned gv1+16
;;     stack_limit = gv2
;;
;;                                 block0(v0: i64, v1: i64):
;; @0044                               v3 = iconst.i32 2
;; @0046                               jump block1
;;
;;                                 block1:
;; @0046                               return v3  ; v3 = 2
;; }
;;
;; function u0:2(i64 vmctx, i64) -> i32 tail {
;;     gv0 = vmctx
;;     gv1 = load.i64 notrap aligned readonly gv0+8
;;     gv2 = load.i64 notrap aligned gv1+16
;;     stack_limit = gv2
;;
;;                                 block0(v0: i64, v1: i64):
;; @0049                               v3 = iconst.i32 3
;; @004b                               jump block1
;;
;;                                 block1:
;; @004b                               return v3  ; v3 = 3
;; }
;;
;; function u0:3(i64 vmctx, i64, i32) -> i32 tail {
;;     gv0 = vmctx
;;     gv1 = load.i64 notrap aligned readonly gv0+8
;;     gv2 = load.i64 notrap aligned gv1+16
;;     gv3 = vmctx
;;     gv4 = load.i64 notrap aligned readonly can_move gv3+48
;;     sig0 = (i64 vmctx, i64) -> i32 tail
;;     sig1 = (i64 vmctx, i32, i64) -> i64 tail
;;     fn0 = colocated u1:9 sig1
;;     stack_limit = gv2
;;
;;                                 block0(v0: i64, v1: i64, v2: i32):
;; @0050                               v4 = iconst.i32 10
;; @0050                               v5 = icmp uge v2, v4  ; v4 = 10
;; @0050                               v6 = uextend.i64 v2
;; @0050                               v7 = load.i64 notrap aligned readonly can_move v0+48
;;                                     v28 = iconst.i64 3
;; @0050                               v8 = ishl v6, v28  ; v28 = 3
;; @0050                               v9 = iadd v7, v8
;; @0050                               v10 = iconst.i64 0
;; @0050                               v11 = select_spectre_guard v5, v10, v9  ; v10 = 0
;; @0050                               v12 = load.i64 user5 aligned table v11
;;                                     v27 = iconst.i64 -2
;; @0050                               v13 = band v12, v27  ; v27 = -2
;; @0050                               brif v12, block3(v13), block2
;;
;;                                 block2 cold:
;; @0050                               v15 = iconst.i32 0
;; @0050                               v17 = uextend.i64 v2
;; @0050                               v18 = call fn0(v0, v15, v17)  ; v15 = 0
;; @0050                               jump block3(v18)
;;
;;                                 block3(v14: i64):
;; @0050                               v20 = load.i64 notrap aligned readonly can_move v0+40
;; @0050                               v21 = load.i32 notrap aligned readonly can_move v20
;; @0050                               v22 = load.i32 user6 aligned readonly v14+16
;; @0050                               v23 = icmp eq v22, v21
;; @0050                               trapz v23, user7
;; @0050                               v24 = load.i64 notrap aligned readonly v14+8
;; @0050                               v25 = load.i64 notrap aligned readonly v14+24
;; @0050                               v26 = call_indirect sig0, v24(v25, v0)
;; @0053                               jump block1
;;
;;                                 block1:
;; @0053                               return v26
;; }
