use stm32_blink::get_the_answer;

#[test]
fn test_true() {
    assert!(true);
}

#[test]
fn test_the_answer() {
    assert_eq!(get_the_answer(), 42);
}
