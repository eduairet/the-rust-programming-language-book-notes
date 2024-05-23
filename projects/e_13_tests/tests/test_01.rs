use e_13_tests::Guess;

#[test]
fn greater_than_100() {
    let guess = Guess::new(200);
    assert_eq!(guess.value(), 200);
}
