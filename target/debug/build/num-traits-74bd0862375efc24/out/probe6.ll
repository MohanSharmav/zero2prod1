; ModuleID = 'probe6.df840a91-cgu.0'
source_filename = "probe6.df840a91-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_3974539364cecb332ef81b2ce0c16baa = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/f77bfb7336f21bfe6a5fb5f7358d4406e2597289/library/core/src/num/mod.rs" }>, align 1
@alloc_3cb328c086d61927b60331b1120f47b5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_3974539364cecb332ef81b2ce0c16baa, [16 x i8] c"K\00\00\00\00\00\00\00/\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe6::probe
; Function Attrs: uwtable
define void @_ZN6probe65probe17haa8e4e4cd4ce8d67E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17he9a6953ef7837251E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hc2ef20a1778c568dE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_3cb328c086d61927b60331b1120f47b5) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17he9a6953ef7837251E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hc2ef20a1778c568dE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}