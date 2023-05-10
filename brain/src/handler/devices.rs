use actix_web::get;
use actix_web::web::{Data, Json};
use rorm::{query, Database};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::handler::ApiResult;
use crate::models::{Device, DeviceType};

/// The representation of a single device
#[derive(Serialize, ToSchema)]
pub struct DeviceResponse {
    uuid: Uuid,
    name: String,
    description: String,
    device_type: DeviceType,
}

/// The list of all devices
#[derive(Serialize, ToSchema)]
pub struct GetDevicesResponse {
    devices: Vec<DeviceResponse>,
}

/// Retrieve all devices
#[utoipa::path(
    tag = "Device management",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Retrieve all devices", body = GetDevicesResponse),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("session_cookie" = []))
)]
#[get("/devices")]
pub async fn get_devices(db: Data<Database>) -> ApiResult<Json<GetDevicesResponse>> {
    let devices = query!(db.as_ref(), Device).all().await?;

    Ok(Json(GetDevicesResponse {
        devices: devices
            .into_iter()
            .map(|x| DeviceResponse {
                uuid: x.uuid,
                name: x.name,
                description: x.description,
                device_type: x.device_type,
            })
            .collect(),
    }))
}
