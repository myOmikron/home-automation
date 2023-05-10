use actix_toolbox::tb_middleware::Session;
use actix_web::get;
use actix_web::web::{Data, Json};
use chrono::{DateTime, Utc};
use rorm::{query, Database, Model};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::handler::{ApiError, ApiResult};
use crate::models::User;

/// The response to a get me call
#[derive(ToSchema, Serialize)]
pub struct GetMeResponse {
    uuid: Uuid,
    username: String,
    display_name: String,
    last_login: Option<DateTime<Utc>>,
}

/// Retrieve the data about the currently logged in user
#[utoipa::path(
    tag = "User management",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "User details", body = GetMeResponse),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("session_cookie" = []))
)]
#[get("/users/me")]
pub async fn get_me(session: Session, db: Data<Database>) -> ApiResult<Json<GetMeResponse>> {
    let uuid: Uuid = session.get("uuid")?.ok_or(ApiError::SessionCorrupt)?;

    let me = query!(db.as_ref(), User)
        .condition(User::F.uuid.equals(uuid.as_ref()))
        .optional()
        .await?
        .ok_or(ApiError::SessionCorrupt)?;

    Ok(Json(GetMeResponse {
        uuid: me.uuid,
        username: me.username,
        display_name: me.display_name,
        last_login: me.last_login.map(|x| DateTime::from_utc(x, Utc)),
    }))
}
