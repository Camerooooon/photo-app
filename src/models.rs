use rocket::serde::{Deserialize, Serialize};


#[derive(
    Debug, PartialEq, strum_macros::Display, strum_macros::EnumString, Deserialize, Serialize, FromFormField, Clone
)]
#[serde(crate = "rocket::serde")]
pub enum Permission {
    Admin,
    Upload,
    Finance,
    Unknown,
}

