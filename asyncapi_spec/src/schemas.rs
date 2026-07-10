use crate::{ExternalDocumentationObject, RefOr, ReferenceObject};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

/// A [Multi Format Schema Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#multiFormatSchemaObject)
// TODO: This should probably be an enum that has different types for the `schema` field
// based on the string value of `schema_format`
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub struct MultiFormatSchemaObject {
    /// A string containing the name of the schema format that is used to define
    /// the information. If schemaFormat is missing, it MUST default to
    /// `application/vnd.aai.asyncapi+json;version={{asyncapi}} where {{asyncapi}}`
    /// matches the AsyncAPI Version String. In such a case, this would make the
    /// Multi Format Schema Object equivalent to the Schema Object. When using
    /// Reference Object within the schema, the schemaFormat of the resource
    /// being referenced MUST match the schemaFormat of the schema that contains
    /// the initial reference. For example, if you reference Avro schema, then
    /// schemaFormat of referencing resource and the resource being reference
    /// MUST match.
    #[cfg_attr(feature = "serde", serde(default = "default_schema_format"))]
    pub schema_format: String,
    /// Required. Definition of the message payload. It can be of any type but
    /// defaults to Schema Object. It MUST match the schema format defined in
    /// schemaFormat, including the encoding type. E.g., Avro should be inlined as
    /// either a YAML or JSON object instead of as a string to be parsed as YAML or
    /// JSON. Non-JSON-based schemas (e.g., Protobuf or XSD) MUST be inlined as a string.
    pub schema: serde_json::Value,
}

#[cfg(feature = "serde")]
fn default_schema_format() -> String {
    "application/vnd.aai.asyncapi+json;version=3.1.0".into()
}

/// A [Schema Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#schemaObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(untagged)
)]
#[derive(Clone, Debug)]
pub enum SchemaObject {
    /// A schema is considered valid if it's just a bool
    Bool(bool),
    Schema(AsyncApiSchema),
}

impl SchemaObject {
    /// Returns a reference to the schema variant if the SchemaObject is a schema
    pub fn as_schema(&self) -> Option<&AsyncApiSchema> {
        if let Self::Schema(schema) = self {
            Some(schema)
        } else {
            None
        }
    }
    /// Returns a reference to the bool value if the schema is a bool value
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    /// Takes ownership of the schema value if the SchemaObject is a schema
    pub fn into_schema(self) -> Option<AsyncApiSchema> {
        if let Self::Schema(schema) = self {
            Some(schema)
        } else {
            None
        }
    }
}

#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize)
)]
#[derive(Clone, Debug)]
pub struct AsyncApiSchema {
    /// Adds support for polymorphism. The discriminator is the schema property name that
    /// is used to differentiate between other schema that inherit this schema. The property
    /// name used MUST be defined at this schema and it MUST be in the `required` property
    /// list. When used, the value MUST be the name of this schema or any schema that inherits
    /// it
    ///
    /// This is _not_ the same as a discriminator in OpenAPI, so anything that produces
    /// a schema meant for an OpenAPI context needs to be transformed first before being
    /// deserialized into this struct (as of asyncapi 3.1.0)
    pub discriminator: Option<String>,
    #[cfg_attr(feature = "serde", serde(rename = "externalDocs"))]
    pub external_docs: Option<RefOr<ExternalDocumentationObject>>,
    pub deprecated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    /// The remaining JSON Schema definition
    pub __schema: serde_json::Value,
}

/// Wrapper enum for each variant a schema can be in an asyncapi document
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(untagged)
)]
#[derive(Clone, Debug)]
pub enum SchemaDefinition {
    Ref(ReferenceObject),
    MultiFormat(MultiFormatSchemaObject),
    Schema(SchemaObject),
}
