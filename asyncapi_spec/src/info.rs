use crate::Url;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

use crate::{ExternalDocumentationObject, RefOr, TagsObject};

/// An [Info Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#infoObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub struct InfoObject {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<ContactObject>,
    pub license: Option<LicenseObject>,
    pub tags: Option<TagsObject>,
    pub external_docs: Option<RefOr<ExternalDocumentationObject>>,
}

impl InfoObject {
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            version: version.into(),
            description: None,
            terms_of_service: None,
            contact: None,
            license: None,
            tags: None,
            external_docs: None,
        }
    }
}

/// A [Contact Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#contactObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug, Default)]
pub struct ContactObject {
    pub name: Option<String>,
    pub url: Option<Url>,
    pub email: Option<String>,
}

/// A [License Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#licenseObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug)]
pub struct LicenseObject {
    pub name: String,
    pub url: Option<Url>,
}

impl LicenseObject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
        }
    }
}
