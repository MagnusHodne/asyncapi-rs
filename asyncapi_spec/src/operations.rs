use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

use crate::{
    ExternalDocumentationObject, RefOr, ReferenceObject, SecuritySchemeObject, TagsObject,
};

/// An [Operations Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#operationsObject)
pub type OperationsObject = HashMap<String, RefOr<OperationObject>>;

/// An [Operation Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#operationObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub struct OperationObject {
    pub action: Action,
    pub channel: ReferenceObject,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub security: Option<Vec<RefOr<SecuritySchemeObject>>>,
    pub tags: Option<TagsObject>,
    pub external_docs: Option<RefOr<ExternalDocumentationObject>>,
    pub bindings: Option<RefOr<OperationBindingsObject>>,
    pub traits: Option<Vec<RefOr<OperationTraitObject>>>,
    pub messages: Option<Vec<ReferenceObject>>,
    pub reply: Option<RefOr<OperationReplyObject>>,
}

/// An operation action
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Clone, Debug)]
pub enum Action {
    Send,
    Receive,
}

/// An [Operation Reply Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#operationReplyObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug)]
pub struct OperationReplyObject {
    pub address: Option<RefOr<OperationReplyAddressObject>>,
    pub channel: Option<ReferenceObject>,
    pub messages: Option<Vec<ReferenceObject>>,
}

/// An [Operation Trait Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#operationTraitObject)
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Clone, Debug)]
pub struct OperationTraitObject {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub security: Option<Vec<RefOr<SecuritySchemeObject>>>,
    pub tags: Option<TagsObject>,
    pub external_docs: Option<RefOr<ExternalDocumentationObject>>,
    pub bindings: Option<RefOr<OperationBindingsObject>>,
}

/// An [Operation Reply Address Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#operationReplyAddressObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug)]
pub struct OperationReplyAddressObject {
    pub description: Option<String>,
    pub location: String,
}

/// An [Operation Bindings Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#operationBindingsObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug)]
pub struct OperationBindingsObject {
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
