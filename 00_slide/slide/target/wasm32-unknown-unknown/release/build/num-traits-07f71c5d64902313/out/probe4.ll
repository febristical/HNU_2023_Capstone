; ModuleID = 'probe4.52addaeb12b1d165-cgu.0'
source_filename = "probe4.52addaeb12b1d165-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_e8be82769d7fb4ec4a8566f665d8b09e = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/ba7c7a301984967c8c13adb580ef9b86ba706a83/library/core/src/num/mod.rs" }>, align 1
@alloc_a63ccb2c7f9355e34ae88ab7d72e542d = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_e8be82769d7fb4ec4a8566f665d8b09e, [12 x i8] c"K\00\00\00y\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe45probe17h8f033f38b476b6ecE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7e662820a9f415d2E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h9cfd10e081b94431E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_a63ccb2c7f9355e34ae88ab7d72e542d) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7e662820a9f415d2E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h9cfd10e081b94431E(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #3 = { noreturn nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.76.0-nightly (ba7c7a301 2023-11-13)"}
