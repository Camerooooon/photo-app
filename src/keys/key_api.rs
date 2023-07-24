use std::time::Duration;

use rocket::{form::Form, response::Redirect, State};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::users::user::User;
use crate::{
    keys::{key_generator::generate_api_key, key_repository::write_key},
    models::Permission,
};

static MINUTES_TO_SECONDS: i32 = 60;

#[derive(FromForm)]
pub struct CreateApiKeyRequest {
    pub permissions: Vec<Permission>,
    pub expiration_minutes: i32,
}

#[post("/api/key/new", data = "<request_opt>")]
pub async fn new_key(
    pool: &State<Pool<MySql>>,
    user: User,
    request_opt: Form<Option<CreateApiKeyRequest>>,
) -> Redirect {
    if let Some(request) = request_opt.into_inner() {
        let missing_permissions = request
            .permissions
            .iter()
            .filter(|permission| !user.permissions.contains(permission))
            .collect::<Vec<_>>();

        if !missing_permissions.is_empty() {
            return Redirect::to("/settings?notice=KEY_CREATION_ERROR");
        }

        let key = generate_api_key(
            user.username,
            Duration::from_secs_f32((request.expiration_minutes * MINUTES_TO_SECONDS) as f32),
            request.permissions.to_vec(),
        );

        match write_key(pool, &key).await {
            Ok(_) => return Redirect::to("/settings?notice=KEY_CREATED"),
            Err(_) => return Redirect::to("/settings?notice=KEY_CREATION_ERROR"),
        };
    }
    Redirect::to("/settings/key/new?error=MISSING_FIELDS")
}
