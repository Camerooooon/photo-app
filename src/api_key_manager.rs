use std::time::SystemTime;

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rocket::{State, response::Redirect, form::Form};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{models::{User, Permission, ApiKey}, database};

#[derive(FromForm)]
pub struct CreateApiKeyRequest {
    pub permissions: Vec<Permission>,
    /// Expiration in minutes
    pub expiration: i32,
}

#[post("/api/key/new", data="<request_opt>")]
pub async fn new_key(
    pool: &State<Pool<MySql>>,
    user: User,
    request_opt: Form<Option<CreateApiKeyRequest>>,
) -> Redirect {

    match request_opt.into_inner() {
        Some(request) => {
            for permission in &request.permissions {
                if !user.permissions.contains(&permission) {
                    return Redirect::to("/settings?notice=KEY_CREATION_ERROR");
                }
            }
            let key = generate_api_key(user.username,request.permissions.to_vec());
            println!("{:?}", key);
            match database::write_key(pool, &key).await {
                Ok(_) => { return Redirect::to("/settings?notice=KEY_CREATED") },
                Err(_) => { return Redirect::to("/settings?notice=KEY_CREATION_ERROR") }
            };
        }
        None => Redirect::to("/settings/key/new?error=MISSING_FIELDS"),
    }
}

/// Generates image meta data from the current time
fn generate_api_key(owner: String, permissions: Vec<Permission>) -> ApiKey {
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
        permissions
    }
}