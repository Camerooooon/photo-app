use rocket::serde::{Deserialize, Serialize};

#[derive(
    Debug,
    PartialEq,
    strum_macros::Display,
    strum_macros::EnumString,
    Deserialize,
    Serialize,
    FromFormField,
    Clone,
)]
#[serde(crate = "rocket::serde")]
pub enum Permission {
    Admin,
    Upload,
    Finance,
    Unknown,
}

pub fn from_comma_seperated_string(value: String) -> Vec<Permission> {
    value
        .split(",")
        .filter(|s| !s.is_empty())
        .into_iter()
        .map(|s| Permission::try_from(s).unwrap_or(Permission::Unknown))
        .collect()
}
