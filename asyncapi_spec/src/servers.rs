use std::collections::HashMap;

use crate::{ExternalDocumentationObject, RefOr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

use crate::SecuritySchemeObject;
use crate::TagsObject;

/// A [Servers Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#serversObject)
pub type ServersObject = HashMap<String, RefOr<ServerObject>>;

/// A [Server Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#serverObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub struct ServerObject {
    pub host: String,
    pub protocol: String,
    pub protocol_version: Option<String>,
    pub pathname: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub variables: Option<HashMap<String, RefOr<ServerVariableObject>>>,
    pub security: Option<Vec<RefOr<SecuritySchemeObject>>>,
    pub tags: Option<TagsObject>,
    pub external_docs: Option<RefOr<ExternalDocumentationObject>>,
    pub bindings: Option<RefOr<ServerBindingsObject>>,
}

impl ServerObject {
    pub fn new(host: impl Into<String>, protocol: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            protocol: protocol.into(),
            protocol_version: None,
            pathname: None,
            description: None,
            title: None,
            summary: None,
            variables: None,
            security: None,
            tags: None,
            external_docs: None,
            bindings: None,
        }
    }
}

/// A [Server Variable Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#serverVariableObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, Default)]
pub struct ServerVariableObject {
    #[cfg_attr(feature = "serde", serde(rename = "enum"))]
    pub _enum: Option<Vec<String>>,
    pub default: Option<String>,
    pub description: Option<String>,
    pub examples: Option<Vec<String>>,
}

/// A [Server Bindings Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#serverBindingsObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug, Default)]
pub struct ServerBindingsObject {
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
