#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RefOr;

/// A [Tags Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#tagsObject)
pub type TagsObject = Vec<RefOr<TagObject>>;

/// A [Tag Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#tagObject)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct TagObject {
    pub name: String,
    pub description: Option<String>,
}

impl TagObject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }
}
