#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/events/ui/01-content-sanity-check.rs");
    t.compile_fail("tests/events/ui/02-no-event-type.rs");
    t.compile_fail("tests/events/ui/03-invalid-event-type.rs");
    t.pass("tests/events/ui/10-content-wildcard.rs");
    t.pass("tests/events/ui/11-content-without-relation-sanity-check.rs");
    t.compile_fail("tests/events/ui/12-no-relates_to.rs");
    t.pass("tests/events/ui/13-private-event-content-type.rs");
}
