use super::*;

const TEST_PLACE: CodePlace = CodePlace::new("src/place/tests.rs", 3, 71);
const TEST_PLACE_MACRO: CodePlace = place!();

#[test]
fn place_display() {
    assert_eq!(&format!("{}", TEST_PLACE), "src/place/tests.rs:3:71");
    assert_eq!(&format!("{}", TEST_PLACE_MACRO), "src/place/tests.rs:4:37");
}

#[test]
fn place_display_pretty() {
    assert_eq!(&format!("{:#}", TEST_PLACE), "src/place/tests.rs:3:71");
    assert_eq!(
        &format!("{:#}", TEST_PLACE_MACRO),
        "src/place/tests.rs:4:37"
    );
}

#[test]
fn place_debug() {
    assert_eq!(&format!("{:?}", TEST_PLACE), "src/place/tests.rs:3:71");
    assert_eq!(
        &format!("{:?}", TEST_PLACE_MACRO),
        "src/place/tests.rs:4:37"
    );
}

#[test]
fn place_debug_pretty() {
    assert_eq!(&format!("{:#?}", TEST_PLACE), "src/place/tests.rs:3:71");
    assert_eq!(
        &format!("{:#?}", TEST_PLACE_MACRO),
        "src/place/tests.rs:4:37"
    );
}
