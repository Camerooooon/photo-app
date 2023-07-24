use std::time::{Duration, SystemTime};

use rocket::serde::{Deserialize, Serialize};

use crate::models::Permission;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiKey {
    pub owner: String,
    pub created: SystemTime,
    pub expires: Duration,
    pub secret: String,
    pub permissions: Vec<Permission>,
}
