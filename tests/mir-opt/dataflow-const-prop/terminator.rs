// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// unit-test: DataflowConstProp

fn foo(n: i32) {}

// EMIT_MIR terminator.main.DataflowConstProp.diff

// CHECK-LABEL: fn main(
fn main() {
    let a = 1;
    // Checks that we propagate into terminators.
    // CHECK: {{_.*}} = foo(const 2_i32) -> [return: {{bb.*}}, unwind continue];
    foo(a + 1);
}
