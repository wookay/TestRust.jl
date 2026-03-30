use my_rustcall;

#[test]
fn add_two_numbers() {
    assert_eq!(my_rustcall::add(1, 2), 3);
}
