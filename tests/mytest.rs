mod common;
use rust_by_example_12;
#[test]
fn larger_can_hold_small() {
    // use super::*;
    common::setup();
    let larger = rust_by_example_12::Rectangle {
        width: 8,
        height: 7,
    };
    let small = rust_by_example_12::Rectangle {
        width: 5,
        height: 1,
    };
    assert!(larger.can_hold(&small));
}
