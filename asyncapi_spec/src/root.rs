use crate::{
    ChannelsObject, ComponentsObject, Identifier, InfoObject, OperationsObject, ServersObject,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An [AsyncAPI Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#A2SObject)
///
/// Represented as an enum to ensure nested schema definition matches asyncapi spec version
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(tag = "asyncapi")
)]
#[derive(Clone, Debug)]
pub enum AsyncAPIObject {
    /// The 3.1.0 version of an asyncapi spec
    #[cfg_attr(feature = "serde", serde(rename = "3.1.0"))]
    V3_1_0 {
        id: Option<Identifier>,
        info: InfoObject,
        servers: Option<ServersObject>,
        default_content_type: Option<String>,
        channels: Option<ChannelsObject>,
        operations: Option<OperationsObject>,
        components: Option<ComponentsObject>,
    },
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use crate::AsyncAPIObject;
    use crate::Identifier;
    use serde_json::json;

    #[test]
    fn deserialize_root_spec() -> anyhow::Result<()> {
        let spec = json!({
            "asyncapi": "3.1.0",
            "id": "com.example.asyncapi",
            "info": { "title": "My AsyncAPI", "version": "0.1.0" }
        });
        let de: AsyncAPIObject = serde_json::from_value(spec.clone())?;
        assert!(
            matches!(de, AsyncAPIObject::V3_1_0 { id: Some(ref id), .. } if id == "com.example.asyncapi" && matches!(id, Identifier::Urn(_)))
        );
        assert!(
            matches!(de, AsyncAPIObject::V3_1_0 { ref info, .. } if info.title == "My AsyncAPI")
        );
        Ok(())
    }
}
