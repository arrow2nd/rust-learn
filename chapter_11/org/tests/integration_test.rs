extern crate org;

mod common;

#[test]
fn it_add_two() {
    common::setup();
    assert_eq!(4, org::add_two(2));
}
