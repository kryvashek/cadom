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

#[test]
fn chain_display() {
    let chain = CodePlaceChain::from(TEST_PLACE.clone()).prepend(place!());
    assert_eq!(
        &format!("{}", chain),
        "[src/place/tests.rs:41:66, src/place/tests.rs:3:71]"
    );
    let chain = CodePlaceChain::from(TEST_PLACE_MACRO.clone()).prepend(place!());
    assert_eq!(
        &format!("{}", chain),
        "[src/place/tests.rs:46:72, src/place/tests.rs:4:37]"
    );
}

#[test]
fn chain_debug() {
    let chain = CodePlaceChain::from(TEST_PLACE.clone()).prepend(place!());
    assert_eq!(
        &format!("{:?}", chain),
        "[src/place/tests.rs:55:66, src/place/tests.rs:3:71]"
    );
    let chain = CodePlaceChain::from(TEST_PLACE_MACRO.clone()).prepend(place!());
    assert_eq!(
        &format!("{:?}", chain),
        "[src/place/tests.rs:60:72, src/place/tests.rs:4:37]"
    );
}
