use rocket::State;
use rocket_dyn_templates::{context, Template};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{database, models::User};

#[get("/")]
pub async fn index(pool: &State<Pool<MySql>>) -> Result<Template, String> {
    Ok(Template::render(
        "index",
        context! {
            name: database::get_recent_images(&pool).await.map_err(|_| "Could not fetch recent images from database")?,
        },
    ))
}

#[get("/login?<error>&<notice>")]
pub async fn login(notice: Option<String>, error: Option<String>) -> Result<Template, String> {
    let notice_message = match notice.unwrap_or_default().as_str() {
        "ACCOUNT_CREATED" => "Your account has been created, please log in with your username and password",
        _ => "",
    };
    let error_message = match error.unwrap_or_default().as_str() {
        "INVALID_USER_PASS" => "Invalid username or password",
        "VERIFICATION_FAILED" => "We were unable to log you in, please try again later",
        _ => "",
    };
    Ok(Template::render(
        "login",
        context! {
            notice: notice_message,
            error: error_message
        },
    ))
}

#[get("/register?<error>")]
pub async fn register(error: Option<String>) -> Result<Template, String> {
    let error_message = match error.unwrap_or_default().as_str() {
        "DUPLICATE_USERNAME" => "That username is already taken, please try another!",
        "INVALID_USERNAME" => "Usernames must only contain letters and numbers",
        "REGISTRATION_FAILED" => "We were unable to add you to our systems, please try again later",
        _ => "",
    };
    Ok(Template::render(
        "register",
        context! {
            error: error_message
        },
    ))
}

#[get("/dashboard")]
pub async fn dashboard(user: User) -> Result <Template, String> {
    let mut notice_message = "";
    if user.permissions.is_empty() {
        notice_message = "You currently do not have permission to access the dashboard, please wait for your account to be approved!"
    }
    Ok(Template::render(
        "dashboard",
        context! {
            notice: notice_message,
            permissions: user.permissions,
            username: user.username
        },
    ))
}
