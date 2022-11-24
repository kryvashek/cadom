use std::ops::Deref;

use super::*;

#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
enum FailKind {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Custom(String),
}

impl From<String> for FailKind {
    fn from(src: String) -> Self {
        FailKind::Custom(src)
    }
}

type Fail = Decay<FailKind>;

#[test]
fn external_parse_error_all_steps() {
    let parse_u8_error = "definetely not a number"
        .parse::<u8>()
        .expect_err("Prasing attempt of presented text should cause an error");
    let fail_kind = FailKind::from(parse_u8_error.clone());
    let fail = Fail::from(fail_kind);

    match fail {
        Decay::External { error } => assert_eq!(error, FailKind::ParseInt(parse_u8_error)),
        _ => panic!("Fail variant should be Decay::External"),
    }
}

#[test]
fn external_custom_error_all_steps() {
    let custom_error = "Text representing some error".to_owned();
    let fail_kind = FailKind::from(custom_error.clone());
    let fail = Fail::from(fail_kind);

    match fail {
        Decay::External { error } => assert_eq!(error, FailKind::Custom(custom_error)),
        _ => panic!("Fail variant should be Decay::External"),
    }
}

#[test]
fn external_parse_error_morph_unnoted_once() {
    let parse_u8_error = "definetely not a number"
        .parse::<u8>()
        .expect_err("Prasing attempt of presented text should cause an error");
    let fail: Fail = rot!()(parse_u8_error.clone());

    match fail {
        Decay::Further { error, note, place } => {
            assert_eq!(
                error.deref(),
                &Decay::External {
                    error: FailKind::ParseInt(parse_u8_error)
                }
            );
            assert_eq!(note, Note::NONE);
            let mut places_iter = place.into_iter();
            match places_iter.next() {
                None => panic!("Exactly one place should be added"),
                Some(cp) => {
                    assert_eq!(cp.file, "src/decay/tests.rs");
                    assert_eq!(cp.line, 52);
                    assert_eq!(cp.column, 22);
                }
            }
            assert_eq!(places_iter.next(), None);
        }
        _ => panic!("Fail variant should be Decay::Further"),
    }
}

#[test]
fn external_custom_error_morph_once_str() {
    let custom_error = "Text representing some error".to_owned();
    let fail: Fail = rot!("Some note")(custom_error.clone());

    match fail {
        Decay::Further { error, note, place } => {
            assert_eq!(
                error.deref(),
                &Decay::External {
                    error: FailKind::Custom(custom_error)
                }
            );
            assert_eq!(note.text(), Some("Some note"));
            let mut places_iter = place.into_iter();
            match places_iter.next() {
                None => panic!("Exactly one place should be added"),
                Some(cp) => {
                    assert_eq!(cp.file, "src/decay/tests.rs");
                    assert_eq!(cp.line, 81);
                    assert_eq!(cp.column, 22);
                }
            }
            assert_eq!(places_iter.next(), None);
        }
        _ => panic!("Fail variant should be Decay::Further"),
    }
}

#[test]
fn new_error_morph_thrice_string() {
    let start: Fail = decay!("Dumb sample error with text '{}'", "text of error");
    let fail: Fail = rot!()(start);
    let fail: Fail = rot!("Some note #{}, formatted", 2)(fail);

    let next_level = match fail {
        Decay::Further { error, note, place } => {
            assert_eq!(note.text(), Some("Some note #2, formatted"));
            let mut places_iter = place.into_iter();
            match places_iter.next() {
                None => panic!("Exactly one place should be added"),
                Some(cp) => {
                    assert_eq!(cp.file, "src/decay/tests.rs");
                    assert_eq!(cp.line, 111);
                    assert_eq!(cp.column, 22);
                }
            }
            assert_eq!(places_iter.next(), None);
            error
        }
        _ => panic!("Fail variant should be Decay::Further"),
    };

    match next_level.deref() {
        Decay::Internal { note, place } => {
            assert_eq!(
                note.text(),
                Some("Dumb sample error with text 'text of error'")
            );
            let mut places_iter = place.into_iter();
            match places_iter.next() {
                None => panic!("Exactly two places should be added (this should be the last one)"),
                Some(cp) => {
                    assert_eq!(cp.file, "src/decay/tests.rs");
                    assert_eq!(cp.line, 110);
                    assert_eq!(cp.column, 22);
                }
            }
            match places_iter.next() {
                None => panic!("Exactly two places should be added (this should be the first one)"),
                Some(cp) => {
                    assert_eq!(cp.file, "src/decay/tests.rs");
                    assert_eq!(cp.line, 109);
                    assert_eq!(cp.column, 23);
                }
            }
            assert_eq!(places_iter.next(), None);
        }
        _ => panic!("Fail variant should be Decay::Internal"),
    }
}

#[test]
fn external_custom_error_morph_once_from_o() {
    let custom_error = FailKind::Custom("Text representing some error".into());
    let fail: Fail = rot!("Some note")(custom_error.clone());

    match fail {
        Decay::Further { error, note, place } => {
            assert_eq!(
                error.deref(),
                &Decay::External {
                    error: custom_error
                }
            );
            assert_eq!(note.text(), Some("Some note"));
            let mut places_iter = place.into_iter();
            match places_iter.next() {
                None => panic!("Exactly one place should be added"),
                Some(cp) => {
                    assert_eq!(cp.file, "src/decay/tests.rs");
                    assert_eq!(cp.line, 163);
                    assert_eq!(cp.column, 22);
                }
            }
            assert_eq!(places_iter.next(), None);
        }
        _ => panic!("Fail variant should be Decay::Further"),
    }
}

#[test]
fn decay_root_internal() {
    let start: Fail = decay!("Dumb sample error with text '{}'", "text of error");
    let fail: Fail = rot!()(start);
    let fail: Fail = rot!("Some note #{}, formatted", 2)(fail);

    match fail.root() {
        DecayRoot::External { .. } => panic!("Expected DecayRoot::Internal"),
        DecayRoot::Internal { note, place } => {
            assert_eq!(
                note.text(),
                Some("Dumb sample error with text 'text of error'")
            );
            let mut places_iter = place.into_iter();
            match places_iter.next() {
                None => panic!("Exactly two places should be added (no second found)"),
                Some(cp) => {
                    assert_eq!(cp.file, "src/decay/tests.rs");
                    assert_eq!(cp.line, 192);
                    assert_eq!(cp.column, 22);
                }
            }
            match places_iter.next() {
                None => panic!("Exactly two places should be added (no first found)"),
                Some(cp) => {
                    assert_eq!(cp.file, "src/decay/tests.rs");
                    assert_eq!(cp.line, 191);
                    assert_eq!(cp.column, 23);
                }
            }
            assert_eq!(places_iter.next(), None);
        }
    }
}

#[test]
fn decay_root_external() {
    let custom_error = FailKind::Custom("Text representing some error".into());
    let fail: Fail = rot!("Some note")(custom_error.clone());

    match fail.root() {
        DecayRoot::Internal { .. } => panic!("Expected DecayRoot::External"),
        DecayRoot::External { error } => assert_eq!(error, &custom_error),
    }
}
