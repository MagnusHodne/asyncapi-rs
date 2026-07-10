#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::{ExternalDocumentationObject, RefOr, schemas::SchemaDefinition, tags::TagsObject};

/// A [Messages Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#messagesObject)
pub type MessagesObject = HashMap<String, RefOr<MessageObject>>;

/// A [Message Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#messageObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, Default)]
pub struct MessageObject {
    pub headers: Option<SchemaDefinition>,
    pub payload: Option<SchemaDefinition>,
    pub correlation_id: Option<RefOr<CorrelationIdObject>>,
    pub content_type: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<TagsObject>,
    pub external_docs: Option<RefOr<ExternalDocumentationObject>>,
    pub bindings: Option<RefOr<MessageBindingsObject>>,
    pub examples: Option<Vec<MessageExampleObject>>,
    pub traits: Option<Vec<RefOr<MessageTraitObject>>>,
}

/// A [Message Example Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#messageExampleObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug, Default)]
pub struct MessageExampleObject {
    pub headers: Option<HashMap<String, Value>>,
    pub payload: Option<Value>,
    pub name: Option<String>,
    pub summary: Option<String>,
}

/// A [Message Trait Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#messageTraitObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, Default)]
pub struct MessageTraitObject {
    pub headers: Option<SchemaDefinition>,
    pub correlation_id: Option<RefOr<CorrelationIdObject>>,
    pub content_type: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<TagsObject>,
    pub external_docs: Option<RefOr<ExternalDocumentationObject>>,
    pub bindings: Option<RefOr<MessageBindingsObject>>,
    pub examples: Option<Vec<MessageExampleObject>>,
}

/// A [Correlation ID Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#correlationIdObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug)]
pub struct CorrelationIdObject {
    pub description: Option<String>,
    pub location: String,
}

impl CorrelationIdObject {
    pub fn new(location: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            description: None,
        }
    }
}

/// A [Message Bindings Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#messageBindingsObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug, Default)]
pub struct MessageBindingsObject {
    pub http: Option<Value>,
    pub ws: Option<Value>,
    pub kafka: Option<Value>,
    pub anypointmq: Option<Value>,
    pub amqp: Option<Value>,
    pub amqp1: Option<Value>,
    pub mqtt: Option<Value>,
    pub mqtt5: Option<Value>,
    pub nats: Option<Value>,
    pub jms: Option<Value>,
    pub sns: Option<Value>,
    pub solace: Option<Value>,
    pub sqs: Option<Value>,
    pub stomp: Option<Value>,
    pub redis: Option<Value>,
    pub mercure: Option<Value>,
    pub ibmmq: Option<Value>,
    pub googlepubsub: Option<Value>,
    pub pulsar: Option<Value>,
    pub ros2: Option<Value>,
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::MessageObject;
    use crate::{AsyncApiSchema, SchemaDefinition, SchemaObject};
    use crate::{RefOr, ReferenceObject};

    use serde_json::{Value, json};

    #[test]
    fn schema_object_known_fields() -> anyhow::Result<()> {
        let schema = serde_json::from_value::<SchemaObject>(json!({
            "type": "object",
            "discriminator": "kind",
            "externalDocs": {
                "$ref": "#/some/ref"
            }
        }))?;
        let schema = schema
            .as_schema()
            .ok_or_else(|| anyhow::anyhow!("{schema:?} should be a SchemaObject::Schema"))?;
        assert!(matches!(&schema.discriminator, Some(d) if d == "kind"));
        assert!(
            matches!(&schema.external_docs, Some(RefOr::Ref(ReferenceObject { _ref})) if _ref == "#/some/ref")
        );
        assert!(matches!(schema.__schema.get("type"), Some(Value::String(t)) if t == "object"));
        Ok(())
    }

    #[test]
    fn message_object() -> anyhow::Result<()> {
        let schema = serde_json::from_value::<MessageObject>(json!({
            "payload": { "type": "string" },
            "examples": [{ "payload": "foo" }]
        }))?;
        assert!(matches!(
            schema.payload,
            Some(SchemaDefinition::Schema(SchemaObject::Schema(AsyncApiSchema { __schema, .. })))
                if __schema.get("type").is_some_and(|t| t == "string")
        ));
        Ok(())
    }
}
