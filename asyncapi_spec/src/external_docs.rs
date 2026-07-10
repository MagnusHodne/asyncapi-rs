use crate::Url;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An [External Documentation Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#externalDocumentationObject)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ExternalDocumentationObject {
    pub description: Option<String>,
    pub url: Url,
}
