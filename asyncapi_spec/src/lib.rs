//! AsyncAPI specification expressed via the type system
//!
//! # Crate features
//! * **serde** - Enables serde derives for all the structs
//! * **url** - Uses the [`url`] crate to parse URL fields

mod channels;
mod components;
mod external_docs;
mod identifier;
mod info;
mod messages;
mod operations;
mod reference;
mod root;
mod schemas;
mod security_schemes;
mod servers;
mod tags;

pub use channels::{
    ChannelBindingsObject, ChannelObject, ChannelsObject, ParameterObject, ParametersObject,
};
pub use components::ComponentsObject;
pub use external_docs::ExternalDocumentationObject;
pub use identifier::Identifier;
pub use info::{ContactObject, InfoObject, LicenseObject};
pub use messages::{
    CorrelationIdObject, MessageBindingsObject, MessageExampleObject, MessageObject,
    MessageTraitObject, MessagesObject,
};
pub use operations::{
    Action, OperationBindingsObject, OperationObject, OperationReplyAddressObject,
    OperationReplyObject, OperationTraitObject, OperationsObject,
};
pub use reference::{RefOr, ReferenceObject};
pub use root::AsyncAPIObject;
pub use schemas::{AsyncApiSchema, MultiFormatSchemaObject, SchemaDefinition, SchemaObject};
pub use security_schemes::{
    ApiKeyIn, CommonVariantFields, HttpApiKeyIn, OauthFlowsObject, Scopes, SecuritySchemeObject,
    SecuritySchemeType,
};
pub use servers::{ServerBindingsObject, ServerObject, ServerVariableObject, ServersObject};
pub use tags::{TagObject, TagsObject};

#[cfg(not(feature = "url"))]
pub(crate) type Url = String;
#[cfg(feature = "url")]
pub(crate) type Url = url::Url;
