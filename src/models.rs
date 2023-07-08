use std::time::SystemTime;

/// Metadata souring a certain uploaded image
pub struct ImageMeta {
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
#[derive(strum_macros::Display)]
pub enum Privacy {
    /// Image will appear on front page, group will appear on front page
    Listed,
    /// Image will not appear on front page, group will not appear on front page
    Unlisted,
    /// Image will follow same privacy as group
    Unspecified,
}

/// Contains a category for certain images, will appear on front end
#[derive(strum_macros::Display)]
pub enum Category {
    Landscape,
    Macro,
    Animals,
    Street,
    Documentation,
    Night,
    Candid,
    Sports,
}
