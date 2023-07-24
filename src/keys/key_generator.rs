use std::time::{Duration, SystemTime};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::models::Permission;

use super::key::ApiKey;

/// Generates image meta data from the current time
pub fn generate_api_key(owner: String, expires: Duration, permissions: Vec<Permission>) -> ApiKey {
    // Generate a random name for the image file
    let secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(25)
        .map(char::from)
        .collect::<String>();
    ApiKey {
        owner,
        created: SystemTime::now(),
        secret,
        permissions,
        expires,
    }
}
