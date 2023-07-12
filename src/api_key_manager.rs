use std::time::SystemTime;

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rocket::{State, response::Redirect, form::Form};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::models::{User, Permission, ApiKey};

#[post("/api/key/new", data="<permissions>")]
pub async fn new_key(
    pool: &State<Pool<MySql>>,
    user: User,
    permissions: Form<Vec<Permission>>,
) -> Result<Redirect, Redirect> {

    let key = generate_api_key(user.username,permissions.to_vec());
    println!("{:?}", key);
    todo!()
}

/// Generates image meta data from the current time
fn generate_api_key(owner: String, permissions: Vec<Permission>) -> Result<ApiKey, String> {
    // Generate a random name for the image file
    let secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect::<String>();
    Ok(ApiKey {
        owner,
        created: SystemTime::now(),
        secret,
        permissions
    })
}
