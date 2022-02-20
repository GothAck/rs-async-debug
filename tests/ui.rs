use trybuild::TestCases;

#[test]
fn ui() {
    let t = TestCases::new();
    t.pass("tests/ui/pass/*.rs");
    t.compile_fail("tests/ui/fail/*.rs");
}
