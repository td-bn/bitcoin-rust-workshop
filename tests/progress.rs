// Uncomment the tests one by one to progress with the exercises. 
// It is recommened that you do the exercises in order, as they build on each
// other.
// If you are an advanced user however, feel free to break the rules!

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-setup.rs");
    t.pass("tests/02-wallet.rs");
    t.pass("tests/03-balance.rs");
    t.pass("tests/04-simple-transaction.rs");
    t.pass("tests/05-raw-tx-transmit.rs");
    t.pass("tests/06-multisig.rs");
}
