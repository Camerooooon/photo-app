use std::time::Duration;

use rocket::{form::Form, response::Redirect, State};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::users::user::User;
use crate::{
    keys::{key_generator::generate_api_key, key_repository::write_key},
    models::Permission,
};

#[derive(FromForm)]
pub struct CreateApiKeyRequest {
    pub permissions: Vec<Permission>,
    /// Expiration in minutes
    pub expiration: i32,
}

#[post("/api/key/new", data = "<request_opt>")]
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
            let key = generate_api_key(
                user.username,
                Duration::from_secs_f32((request.expiration * 60) as f32),
                request.permissions.to_vec(),
            );
            println!("{:?}", key);
            match write_key(pool, &key).await {
                Ok(_) => return Redirect::to("/settings?notice=KEY_CREATED"),
                Err(_) => return Redirect::to("/settings?notice=KEY_CREATION_ERROR"),
            };
        }
        None => Redirect::to("/settings/key/new?error=MISSING_FIELDS"),
    }
}
