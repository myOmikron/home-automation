//! The swagger documentation is located in this module

use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::{handler, models};

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
        handler::logout,
        handler::test,
        handler::get_me,
        handler::get_devices,
    ),
    components(schemas(
        handler::ApiStatusCode,
        handler::ApiErrorResponse,
        handler::LoginRequest,
        handler::GetMeResponse,
        models::DeviceType,
        handler::GetDevicesResponse,
        handler::DeviceResponse,
    )),
    modifiers(&CookieSecurity)
)]
pub struct ApiDoc;
