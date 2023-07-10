use rocket::State;
use rocket_dyn_templates::{context, Template};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::database;

#[get("/")]
pub async fn index(pool: &State<Pool<MySql>>) -> Result<Template, String> {
    Ok(Template::render(
        "index",
        context! {
            name: database::get_recent_images(&pool).await.map_err(|_| "Could not fetch recent images from database")?,
        },
    ))
}

#[get("/login?<error>")]
pub async fn login(pool: &State<Pool<MySql>>, error: Option<String>) -> Result<Template, String> {
    let error_message = match error.unwrap_or_default().as_str() {
        "INVALID_USER_PASS" => "Invalid username or password",
        "VERIFICATION_FAILED" => "We were unable to log you in, please try again later",
        _ => "",
    };
    Ok(Template::render(
        "login",
        context! {
            error: error_message
        },
    ))
}
