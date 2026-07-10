use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

use crate::{
    ExternalDocumentationObject, MessagesObject, RefOr, ReferenceObject, tags::TagsObject,
};

/// A [Channels Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#channelsObject)
pub type ChannelsObject = HashMap<String, RefOr<ChannelObject>>;

/// A [Channel Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#channelObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub struct ChannelObject {
    pub address: Option<String>,
    pub messages: Option<MessagesObject>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub servers: Option<Vec<ReferenceObject>>,
    pub parameters: Option<ParametersObject>,
    pub tags: Option<TagsObject>,
    pub external_docs: Option<RefOr<ExternalDocumentationObject>>,
    pub bindings: Option<RefOr<ChannelBindingsObject>>,
}

/// A [Parameters Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#parametersObject)
pub type ParametersObject = HashMap<String, RefOr<ParameterObject>>;

/// A [Parameter Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#parameterObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub struct ParameterObject {
    #[cfg_attr(feature = "serde", serde(rename = "enum"))]
    pub _enum: Option<Vec<String>>,
    pub default: Option<String>,
    pub description: Option<String>,
    pub examples: Option<Vec<String>>,
    pub location: Option<String>,
}

/// An AsyncAPI [Channel Bindings Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#channelBindingsObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug)]
pub struct ChannelBindingsObject {
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
