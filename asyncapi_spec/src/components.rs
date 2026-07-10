use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

use crate::{
    ChannelBindingsObject, CorrelationIdObject, ExternalDocumentationObject, MessageBindingsObject,
    MessageObject, MessageTraitObject, OperationBindingsObject, OperationObject,
    OperationReplyAddressObject, OperationReplyObject, OperationTraitObject, ParameterObject,
    RefOr, SchemaDefinition, SecuritySchemeObject, ServerBindingsObject, ServerObject,
    ServerVariableObject, TagObject,
};

type Component<T> = Option<HashMap<String, T>>;

/// A [Components Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#componentsObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug, Default)]
pub struct ComponentsObject {
    pub schemas: Component<SchemaDefinition>,
    pub servers: Component<RefOr<ServerObject>>,
    pub operations: Component<RefOr<OperationObject>>,
    pub messages: Component<RefOr<MessageObject>>,
    pub security_schemes: Component<RefOr<SecuritySchemeObject>>,
    pub server_variables: Component<RefOr<ServerVariableObject>>,
    pub parameters: Component<RefOr<ParameterObject>>,
    pub correlationids: Component<RefOr<CorrelationIdObject>>,
    pub replies: Component<RefOr<OperationReplyObject>>,
    pub reply_addresses: Component<RefOr<OperationReplyAddressObject>>,
    pub external_docs: Component<RefOr<ExternalDocumentationObject>>,
    pub tags: Component<RefOr<TagObject>>,
    pub operation_traits: Component<RefOr<OperationTraitObject>>,
    pub message_traits: Component<RefOr<MessageTraitObject>>,
    pub server_bindings: Component<RefOr<ServerBindingsObject>>,
    pub channel_bindings: Component<RefOr<ChannelBindingsObject>>,
    pub operation_bindings: Component<RefOr<OperationBindingsObject>>,
    pub message_bindings: Component<RefOr<MessageBindingsObject>>,
}
