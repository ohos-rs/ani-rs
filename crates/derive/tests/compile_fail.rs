#[test]
fn invalid_bindings_fail_with_actionable_diagnostics() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/*.rs");
}
