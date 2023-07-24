use rocket::State;
use rocket_dyn_templates::{context, Template};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::users::user::User;

use super::key_repository::fetch_key_by_id;

#[get("/settings/key/new")]
pub async fn new_api_key(user: User) -> Result<Template, String> {
    Ok(Template::render(
        "newapikey",
        context! {
            permissions: user.permissions,
        },
    ))
}

#[get("/settings/key/delete/<id>")]
pub async fn delete_api_key(pool: &State<Pool<MySql>>, user: User, id: u32) -> Result<Template, String> {

    let key = fetch_key_by_id(pool, id).await.map_err(|_| "Could not find api key")?;

    if key.owner != user.username {
        return Err("Could not find api key".to_string());
    }

    Ok(Template::render(
        "deleteapikey",
        context! {
            key,
            permissions: user.permissions,
        },
    ))
}
