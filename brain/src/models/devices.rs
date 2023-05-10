use rorm::fields::ForeignModel;
use rorm::{DbEnum, Model, Patch};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Type of a device
#[derive(DbEnum, Serialize, Deserialize, ToSchema)]
pub enum DeviceType {
    /// A simple light that has only two states
    SimpleLight,
    /// A dimmable light
    DimLight,
    /// A light with r, g, b values
    RgbLight,
    /// A light with r, g, b values that are dimmable
    DimRgbLight,
}

/// Representation of arbitrary devices
#[derive(Model)]
pub struct Device {
    /// Primary key of a device
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Name of the device
    #[rorm(max_length = 255)]
    pub name: String,

    /// Description of the device
    #[rorm(max_length = 255)]
    pub description: String,

    /// Type of the device
    pub device_type: DeviceType,
}

#[derive(Patch)]
#[rorm(model = "Device")]
pub(crate) struct DeviceInsert {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) device_type: DeviceType,
}

/// Representation of groups of devices
#[derive(Model)]
pub struct DeviceGroup {
    /// Primary key of a device
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Name of the group
    #[rorm(max_length = 255)]
    pub name: String,
}

/// M2M relation for DeviceGroup <-> Device
#[derive(Model)]
pub struct DeviceGroupMember {
    /// Primary key of the m2m
    #[rorm(id)]
    pub id: i64,

    /// The device group
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub device_group: ForeignModel<DeviceGroup>,

    /// The device
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub device: ForeignModel<Device>,
}

#[derive(Patch)]
#[rorm(model = "DeviceGroupMember")]
pub(crate) struct DeviceGroupMemberInsert {
    pub(crate) device_group: ForeignModel<DeviceGroup>,
    pub(crate) device: ForeignModel<Device>,
}
