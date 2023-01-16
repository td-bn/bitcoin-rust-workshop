#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-setup.rs");
    t.pass("tests/02-wallet.rs");
}
