mod common;
use rust_by_example_12;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn larger_can_hold_smaller() {
    common::setup();
    let larger = rust_by_example_12::Rectangle { width: 8, height: 7};
    let smaller = rust_by_example_12::Rectangle { width: 5, height: 1 };
    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = rust_by_example_12::Rectangle { width: 8, height: 7 };
    let smaller = rust_by_example_12::Rectangle { width: 5, height: 1 };
    assert!(!smaller.can_hold(&larger));
}
