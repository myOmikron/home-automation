use chrono::NaiveDateTime;
use rorm::{Model, Patch};
use uuid::Uuid;

/// The database model of an account
#[derive(Model)]
pub struct User {
    /// The primary key of an account
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The username of an account
    #[rorm(max_length = 255, unique)]
    pub username: String,

    /// The display_name of an account
    #[rorm(max_length = 255, unique)]
    pub display_name: String,

    /// The display_name of an account
    #[rorm(max_length = 4096, unique)]
    pub password_hash: String,

    /// The most recent point in time when the client has logged in
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Patch)]
#[rorm(model = "User")]
pub(crate) struct UserInsert {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
    pub(crate) display_name: String,
    pub(crate) password_hash: String,
}
