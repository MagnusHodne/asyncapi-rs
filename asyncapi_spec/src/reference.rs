#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A [Reference Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#referenceObject)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceObject {
    #[cfg_attr(feature = "serde", serde(rename = "$ref"))]
    pub _ref: String,
}

impl ReferenceObject {
    /// Creates a reference object from a reference string
    ///
    /// No parsing is performed on the input
    pub fn new(reference: impl Into<String>) -> Self {
        Self {
            _ref: reference.into(),
        }
    }
    /// Construct a reference to a schema name within the components object
    pub fn schema(schema_name: impl std::fmt::Display) -> Self {
        Self::new(format!("#/components/schemas/{schema_name}"))
    }
}

/// Wrapper enum for items that can be either a reference to something else in the doc, or a value
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
#[derive(Clone, Debug, PartialEq)]
pub enum RefOr<T> {
    Ref(ReferenceObject),
    T(T),
}
