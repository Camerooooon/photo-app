use std::time::Duration;

use rocket::{State, response::Redirect, form::Form};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{models::Permission, keys::{key_repository::write_key, key_generator::generate_api_key}};
use crate::users::user::User;

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
            let key = generate_api_key(user.username,Duration::from_secs_f32((request.expiration*60) as f32), request.permissions.to_vec());
            println!("{:?}", key);
            match write_key(pool, &key).await {
                Ok(_) => { return Redirect::to("/settings?notice=KEY_CREATED") },
                Err(_) => { return Redirect::to("/settings?notice=KEY_CREATION_ERROR") }
            };
        }
        None => Redirect::to("/settings/key/new?error=MISSING_FIELDS"),
    }
}

