use std::collections::HashMap;

use crate::Url;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

/// A [Security Scheme Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#securitySchemeObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub struct SecuritySchemeObject {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub scheme_properties: SecuritySchemeType,
    pub description: Option<String>,
}

/// Valid sources for the `apiKey.in` field of SecuritySchemeObject
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Clone, Debug)]
pub enum ApiKeyIn {
    User,
    Password,
}

/// Valid sources for the `httpApiKey.in` field of SecuritySchemeObject
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Clone, Debug)]
pub enum HttpApiKeyIn {
    Query,
    Header,
    Cookie,
}

/// Enum representation of the differing fields within a [Security Scheme Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#securitySchemeObject)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(tag = "type"))]
#[derive(Clone, Debug)]
pub enum SecuritySchemeType {
    #[cfg_attr(feature = "serde", serde(rename = "userPassword"))]
    UserPassword,
    #[cfg_attr(feature = "serde", serde(rename = "apiKey"))]
    ApiKey {
        #[cfg_attr(feature = "serde", serde(rename = "in"))]
        _in: ApiKeyIn,
    },
    #[cfg_attr(feature = "serde", serde(rename = "X509"))]
    X509,
    #[cfg_attr(feature = "serde", serde(rename = "symmetricEncryption"))]
    SymmetricEncryption,
    #[cfg_attr(feature = "serde", serde(rename = "asymmetricEncryption"))]
    AsymmetricEncryption,
    #[cfg_attr(feature = "serde", serde(rename = "httpApiKey"))]
    HttpApiKey {
        name: String,
        #[cfg_attr(feature = "serde", serde(rename = "in"))]
        _in: HttpApiKeyIn,
    },
    #[cfg_attr(feature = "serde", serde(rename = "http"))]
    Http {
        scheme: String,
        #[cfg_attr(feature = "serde", serde(rename = "bearerFormat"))]
        bearer_format: Option<String>,
    },
    #[cfg_attr(feature = "serde", serde(rename = "oauth2"))]
    OAuth2 {
        flows: OauthFlowsObject,
        scopes: Scopes,
    },
    #[cfg_attr(feature = "serde", serde(rename = "openIdConnect"))]
    OIDC {
        #[cfg_attr(feature = "serde", serde(rename = "openIdConnectUrl"))]
        oidc_connect_url: Url,
        scopes: Scopes,
    },
    #[cfg_attr(feature = "serde", serde(rename = "plain"))]
    Plain,
    #[cfg_attr(feature = "serde", serde(rename = "scramSha256"))]
    ScramSha256,
    #[cfg_attr(feature = "serde", serde(rename = "scramSha512"))]
    ScramSha512,
    #[cfg_attr(feature = "serde", serde(rename = "gssapi"))]
    GssApi,
}

pub type Scopes = Option<Vec<String>>;

#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub struct CommonVariantFields {
    pub refresh_url: Option<Url>,
    pub available_scopes: HashMap<String, String>,
}

/// An [OAuth Flows Object](https://www.asyncapi.com/docs/reference/specification/v3.1.0#oauthFlowsObject)
#[cfg_attr(
    feature = "serde",
    skip_serializing_none,
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[derive(Clone, Debug)]
pub enum OauthFlowsObject {
    Implicit {
        #[cfg_attr(feature = "serde", serde(rename = "authorizationUrl"))]
        authorization_url: String,
        #[cfg_attr(feature = "serde", serde(flatten))]
        common: CommonVariantFields,
    },
    Password {
        #[cfg_attr(feature = "serde", serde(rename = "tokenUrl"))]
        token_url: Url,
        #[cfg_attr(feature = "serde", serde(flatten))]
        common: CommonVariantFields,
    },
    ClientCredentials {
        #[cfg_attr(feature = "serde", serde(rename = "tokenUrl"))]
        token_url: Url,
        #[cfg_attr(feature = "serde", serde(flatten))]
        common: CommonVariantFields,
    },
    AuthorizationCode {
        #[cfg_attr(feature = "serde", serde(rename = "authorizationUrl"))]
        authorization_url: String,
        #[cfg_attr(feature = "serde", serde(rename = "tokenUrl"))]
        token_url: Url,
        #[cfg_attr(feature = "serde", serde(flatten))]
        common: CommonVariantFields,
    },
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use rstest::rstest;
    use serde_json::{Value, json};

    use super::*;

    #[rstest]
    #[case::user_password(
        json!({ "type": "userPassword" }),
        |scheme| {
            matches!(scheme, SecuritySchemeObject {
                scheme_properties: SecuritySchemeType::UserPassword,
                ..
            })
        }
    )]
    #[case::api_key_auth(
        json!({ "type": "apiKey", "in": "user" }),
        |scheme| {
            matches!(scheme, SecuritySchemeObject {
                scheme_properties: SecuritySchemeType::ApiKey { _in: ApiKeyIn::User },
                ..
            })
        }
    )]
    #[case::x_509(
        json!({ "type": "X509" }),
        |scheme| {
            matches!(scheme, SecuritySchemeObject {
                scheme_properties: SecuritySchemeType::X509,
                ..
            })
        }
    )]
    #[case::symmetric(
        json!({ "type": "symmetricEncryption" }),
        |scheme| {
            matches!(scheme, SecuritySchemeObject {
                scheme_properties: SecuritySchemeType::SymmetricEncryption,
                ..
            })
        }
    )]
    #[case::basic_auth(
        json!({ "type": "http", "scheme": "basic" }),
        |scheme| {
            matches!(scheme, SecuritySchemeObject {
                scheme_properties: SecuritySchemeType::Http { scheme, .. },
                ..
            } if scheme == "basic")
        }
    )]
    #[case::http_api_key(
        json!({ "type": "httpApiKey", "name": "api_key", "in": "header" }),
        |scheme| {
            matches!(scheme, SecuritySchemeObject {
                scheme_properties: SecuritySchemeType::HttpApiKey { name, _in: HttpApiKeyIn::Header },
                ..
            } if name == "api_key")
        }
    )]
    #[case::jwt_bearer(
        json!({ "type": "http", "scheme": "bearer", "bearerFormat": "JWT" }),
        |scheme| {
            matches!(scheme, SecuritySchemeObject {
                scheme_properties: SecuritySchemeType::Http { scheme, bearer_format: Some(format) },
                ..
            } if scheme == "bearer" && format == "JWT")
        }
    )]
    #[case::implicit_oauth(
        json!({
            "type": "oauth2",
            "flows": {
                "implicit": {
                    "authorizationUrl": "https://example.com/api/oauth/dialog",
                    "availableScopes": {
                        "write:pets": "modify pets in your account",
                        "read:pets": "read your pets"
                    }
                }
            },
            "scopes": [
                "write:pets"
            ]
        }),
        |scheme| {
            matches!(scheme, SecuritySchemeObject {
                scheme_properties: SecuritySchemeType::OAuth2 {
                    flows: OauthFlowsObject::Implicit {
                        authorization_url,
                        common: CommonVariantFields {
                            available_scopes,
                            ..
                        },
                        ..
                    },
                    scopes: Some(scopes)
                },
                ..
            } if scopes.contains(&String::from("write:pets"))
                && available_scopes.contains_key("write:pets")
                && available_scopes.contains_key("read:pets")
                && authorization_url == "https://example.com/api/oauth/dialog"
            )
        }
    )]
    fn deserializing<F>(#[case] json: Value, #[case] matches: F)
    where
        F: FnOnce(SecuritySchemeObject) -> bool,
    {
        let result = serde_json::from_value::<SecuritySchemeObject>(json);
        assert!(result.is_ok());
        let scheme = result.expect("Handled by assert above");
        assert!(matches(scheme));
    }
}
