#[cfg(feature = "schema")]
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize, Serializer};
use std::{
    borrow::{Borrow, BorrowMut},
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};

use crate::Decay;

/// Serialization implementation for [Decay] through serde.
/// Since some information is assumed redundant for inter-service communication,
/// it is ommited in serialized version. Thus deserialization into the strictly
/// same type becomes actually impossible. Deserialization into [DecayDeser] is
/// provided as a replacement.
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<O: StdError + Serialize> Serialize for Decay<O> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;
        for decay in self {
            match decay {
                Decay::External { error } => seq.serialize_element(error)?,
                Decay::Internal { note, .. } | Decay::Further { note, .. } => {
                    if let Some(text) = note.text() {
                        seq.serialize_element(text)?
                    }
                }
            }
        }
        seq.end()
    }
}

#[cfg(feature = "schema")]
#[cfg_attr(docsrs, doc(cfg(all(feature = "serde", feature = "schema"))))]
impl<O: StdError + Serialize + JsonSchema> JsonSchema for Decay<O> {
    /// Always returns true since [Decay] is complex enough to be referenced when possible.
    #[inline]
    fn is_referenceable() -> bool {
        true
    }

    /// Will return name like "Decay(<smth>)", where <smth> is the type O schema name.
    /// Avoids redundant memory allocation by writing into the same [String],
    /// but can't avoid copying bytes of the type O schema name (shifts them).
    fn schema_name() -> String {
        let mut name = <O as JsonSchema>::schema_name();
        name.insert_str(0, "Decay(");
        name.push(')');
        name
    }

    /// Since [Decay] serializes just the same way as [DecayDeser],
    /// they obviously share the same schema, which is returned here.
    #[inline]
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        <DecayDeser<O> as JsonSchema>::json_schema(gen)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecayDeserItem<O: StdError> {
    Internal(String),
    External(O),
}

impl<O: StdError + Display> Display for DecayDeserItem<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            DecayDeserItem::Internal(x) => f.write_str(x),
            DecayDeserItem::External(o) => Display::fmt(o, f),
        }
    }
}

pub type DecayDeserInner<O> = Vec<DecayDeserItem<O>>;

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecayDeser<O: StdError>(DecayDeserInner<O>);

impl<O: StdError> Deref for DecayDeser<O> {
    type Target = DecayDeserInner<O>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<O: StdError> DerefMut for DecayDeser<O> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl<O: StdError> Borrow<DecayDeserInner<O>> for DecayDeser<O> {
    #[inline]
    fn borrow(&self) -> &DecayDeserInner<O> {
        self.inner()
    }
}

impl<O: StdError> BorrowMut<DecayDeserInner<O>> for DecayDeser<O> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut DecayDeserInner<O> {
        self.inner_mut()
    }
}

impl<O: StdError> AsRef<DecayDeserInner<O>> for DecayDeser<O> {
    #[inline]
    fn as_ref(&self) -> &DecayDeserInner<O> {
        self.inner()
    }
}

impl<O: StdError> AsMut<DecayDeserInner<O>> for DecayDeser<O> {
    #[inline]
    fn as_mut(&mut self) -> &mut DecayDeserInner<O> {
        self.inner_mut()
    }
}

impl<O: StdError> From<DecayDeser<O>> for DecayDeserInner<O> {
    #[inline]
    fn from(src: DecayDeser<O>) -> Self {
        src.into_inner()
    }
}

impl<O: StdError> From<DecayDeserInner<O>> for DecayDeser<O> {
    #[inline]
    fn from(src: DecayDeserInner<O>) -> Self {
        Self::new(src)
    }
}

impl<O: StdError> DecayDeser<O> {
    #[inline]
    pub fn new(src: DecayDeserInner<O>) -> Self {
        Self(src)
    }

    #[inline]
    pub fn into_inner(self) -> DecayDeserInner<O> {
        self.0
    }

    #[inline]
    pub fn inner(&self) -> &DecayDeserInner<O> {
        &self.0
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut DecayDeserInner<O> {
        &mut self.0
    }
}

impl<O: StdError + Display> Display for DecayDeser<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        cubob::display_list_from_iter(f, self.0.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
    #[error("Level {level} test error: {note}")]
    struct TestInnErr {
        level: u8,
        note: String,
    }

    type TestFail = Decay<TestInnErr>;
    type TestDeser = DecayDeser<TestInnErr>;

    const EXAMPLE_TEXT_EXTERNAL: &str =
        r#"["Some note #2, formatted",{"level":1,"note":"Any text"}]"#;
    const EXAMPLE_TEXT_INTERNAL: &str =
        r#"["Some note #2, formatted","Some 0-level note (formatted) about error"]"#;

    #[test]
    fn serialize_decay_external() {
        let start = TestInnErr {
            level: 1,
            note: "Any text".into(),
        };
        let fail: TestFail = rot!()(start);
        let fail: TestFail = rot!("Some note #{}, formatted", 2)(fail);

        let actual_text =
            serde_json::to_string(&fail).expect("Serialization should complete successfully");

        assert_eq!(actual_text, EXAMPLE_TEXT_EXTERNAL);
    }

    #[test]
    fn serialize_decay_internal() {
        let start: TestFail = decay!("Some {}-level note (formatted) about error", 0);
        let fail: TestFail = rot!()(start);
        let fail: TestFail = rot!("Some note #{}, formatted", 2)(fail);

        let actual_text =
            serde_json::to_string(&fail).expect("Serialization should complete successfully");

        assert_eq!(actual_text, EXAMPLE_TEXT_INTERNAL);
    }

    #[test]
    fn deserialize_decay_deser_external() {
        let decay_deser: TestDeser = serde_json::from_str(EXAMPLE_TEXT_EXTERNAL)
            .expect("Deserialization should complete successfully");

        assert_eq!(decay_deser.len(), 2);
        assert_eq!(
            decay_deser[0],
            DecayDeserItem::Internal("Some note #2, formatted".into())
        );
        assert_eq!(
            decay_deser[1],
            DecayDeserItem::External(TestInnErr {
                level: 1,
                note: "Any text".into()
            })
        );
    }

    #[test]
    fn deserialize_decay_deser_internal() {
        let decay_deser: TestDeser = serde_json::from_str(EXAMPLE_TEXT_INTERNAL)
            .expect("Deserialization should complete successfully");

        assert_eq!(decay_deser.len(), 2);
        assert_eq!(
            decay_deser[0],
            DecayDeserItem::Internal("Some note #2, formatted".into())
        );
        assert_eq!(
            decay_deser[1],
            DecayDeserItem::Internal("Some 0-level note (formatted) about error".into())
        );
    }
}
