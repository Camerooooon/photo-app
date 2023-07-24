use std::time::{SystemTime, Duration};

use rocket::serde::{Deserialize, Serialize};

/// Metadata souring a certain uploaded image
#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ImageMeta {
    pub file_extension: String,
    pub privacy: Privacy,
    pub uploaded: SystemTime,
    pub print_available: bool,
    pub id: String,
    pub name: String,
    pub categories: Vec<Category>,
}

/// A group of images, can be created by an authenticated user
pub struct ImageGroup {
    pub created: SystemTime,
    pub name: String,
    pub privacy: Privacy,
    pub id: String,
}

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

/// The privacy level of a group of images
#[derive(PartialEq, Eq, strum_macros::Display, Serialize, Deserialize, sqlx::Type, Debug)]
#[serde(crate = "rocket::serde")]
pub enum Privacy {
    /// Image will appear on front page, group will appear on front page
    Listed,
    /// Image will not appear on front page, group will not appear on front page
    Unlisted,
    /// Image will follow same privacy as group
    Unspecified,
}

/// Contains a category for certain images, will appear on front end
#[derive(
    Debug, PartialEq, strum_macros::Display, strum_macros::EnumString, Deserialize, Serialize,
)]
#[serde(crate = "rocket::serde")]
pub enum Category {
    Landscape,
    Macro,
    Animals,
    Street,
    Documentation,
    Night,
    Candid,
    Sports,
    Unknown,
}
