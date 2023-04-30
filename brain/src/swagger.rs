//! The swagger documentation is located in this module

use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::handler;

struct CookieSecurity;

impl Modify for CookieSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("id"))),
            )
        }
    }
}

/// Helper struct for the openapi definitions.
#[derive(OpenApi)]
#[openapi(
    paths(
        handler::login,
        handler::logout
    ),
    components(schemas(
        handler::ApiStatusCode,
        handler::ApiErrorResponse,
        handler::LoginRequest
    )),
    modifiers(&CookieSecurity)
)]
pub struct ApiDoc;
