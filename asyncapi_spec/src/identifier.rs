#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// An [Identifier](https://www.asyncapi.com/docs/reference/specification/v3.1.0#A2SIdString)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
#[derive(Clone, Debug, PartialEq)]
pub enum Identifier {
    #[cfg(feature = "url")]
    Url(url::Url),
    // TODO: Find an actual urn type to use for this
    Urn(String),
}
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "url")]
            Self::Url(u) => write!(f, "{u}"),
            Self::Urn(u) => write!(f, "{u}"),
        }
    }
}

#[cfg(feature = "url")]
impl From<url::Url> for Identifier {
    fn from(value: url::Url) -> Self {
        Self::Url(value)
    }
}
impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self::Urn(value)
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        match self {
            #[cfg(feature = "url")]
            Self::Url(url) => url.as_str() == other,
            Self::Urn(string) => string == other,
        }
    }
}
