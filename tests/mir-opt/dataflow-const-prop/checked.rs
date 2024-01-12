// unit-test: DataflowConstProp
// compile-flags: -Coverflow-checks=on
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY

// EMIT_MIR checked.main.DataflowConstProp.diff
#[allow(arithmetic_overflow)]

// CHECK-LABEL: fn main(
fn main() {
    // CHECK: debug a => [[a:_.*]];
    // CHECK: debug b => [[b:_.*]];
    // CHECK: debug c => [[c:_.*]];
    // CHECK: debug d => [[d:_.*]];
    // CHECK: debug e => [[e:_.*]];

    // CHECK: [[a]] = const 1_i32;
    let a = 1;

    // CHECK: [[b]] = const 2_i32;
    let b = 2;

    // CHECK-LABEL: assert(!const false,
    // CHECK: [[c]] = const 3_i32;
    let c = a + b;

    // CHECK: [[d]] = const _;
    let d = i32::MAX;

    // CHECK-LABEL: assert(!const true,
    // CHECK: [[e]] = const i32::MIN;
    let e = d + 1;
}
