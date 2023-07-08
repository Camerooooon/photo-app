use std::time::SystemTime;

use rocket::serde::{Deserialize, Serialize};

/// Metadata souring a certain uploaded image
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ImageMeta {
    pub file_extension: String,
    pub privacy: Privacy,
    pub uploaded: SystemTime,
    pub print_available: bool,
    pub url: String,
    pub name: String,
    pub categories: Vec<Category>,
}

/// A group of images, can be created by an authenticated user
pub struct ImageGroup {
    pub created: SystemTime,
    pub name: String,
    pub privacy: Privacy,
    pub url: String,
}

/// The privacy level of a group of images
#[derive(PartialEq, Eq, strum_macros::Display, Serialize, Deserialize, sqlx::Type)]
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
#[derive(Debug, PartialEq, strum_macros::Display, strum_macros::EnumString)]
#[derive(Deserialize, Serialize)]
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
