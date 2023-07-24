
use chrono_humanize::{HumanTime, Accuracy, Tense};
use rocket::http::{ContentType, CookieJar};
use rocket::{State, fs::NamedFile};
use rocket_dyn_templates::{context, Template};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::database;
use crate::users::user::User;

#[get("/")]
pub async fn index(pool: &State<Pool<MySql>>) -> Result<Template, String> {
    Ok(Template::render(
        "index",
        context! {
            name: database::get_recent_images(&pool).await.map_err(|_| "Could not fetch recent images from database")?,
        },
    ))
}

#[get("/semantic/dist/semantic.min.js")]
pub async fn semantic_js() -> (ContentType, NamedFile) {
    let content_type = ContentType::new("text", "javascript");
    let file = NamedFile::open("./static/semantic.min.js").await.unwrap();

    (content_type, file)
}

#[get("/semantic/dist/semantic.min.css")]
pub async fn semantic_css() -> (ContentType, NamedFile) {
    let content_type = ContentType::new("text", "css");
    let file = NamedFile::open("./static/semantic.min.css").await.unwrap();

    (content_type, file)
}

#[get("/semantic/dist/icon.min.css")]
pub async fn semantic_icon_css() -> (ContentType, NamedFile) {
    let content_type = ContentType::new("text", "css");
    let file = NamedFile::open("./static/icon.min.css").await.unwrap();

    (content_type, file)
}

#[get("/semantic/themes/default/assets/fonts/icons.woff2")]
pub async fn semantic_icon_woff2() -> (ContentType, NamedFile) {
    let content_type = ContentType::new("text", "css");
    let file = NamedFile::open("./static/icons.woff2").await.unwrap();

    (content_type, file)
}

#[get("/login?<error>&<notice>")]
pub async fn login(cookies: &CookieJar<'_>, notice: Option<String>, error: Option<String>) -> Result<Template, String> {
    let notice_message = match notice.unwrap_or_default().as_str() {
        "ACCOUNT_CREATED" => {
            "Your account has been created, please log in with your username and password"
        }
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
            error: error_message,
            signed_in: cookies.get_private("username").is_some(),
        },
    ))
}

#[get("/register?<error>")]
pub async fn register(cookies: &CookieJar<'_>, error: Option<String>) -> Result<Template, String> {
    let error_message = match error.unwrap_or_default().as_str() {
        "DUPLICATE_USERNAME" => "That username is already taken, please try another!",
        "INVALID_USERNAME" => "Username is invalid, usernames must only contain letters and numbers",
        "SHORT_USERNAME" => "That username is too short. Usernames must be at least 3 characters long",
        "SHORT_PASSWORD" => "Please enter a longer password",
        "REGISTRATION_FAILED" => "We were unable to add you to our systems, please try again later",
        _ => "",
    };
    Ok(Template::render(
        "register",
        context! {
            error: error_message,
            signed_in: cookies.get_private("username").is_some(),
        },
    ))
}

#[get("/dashboard")]
pub async fn dashboard(user: User) -> Result<Template, String> {
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

#[get("/settings")]
pub async fn settings(user: User, pool: &State<Pool<MySql>>) -> Result<Template, String> {
    Ok(Template::render(
        "settings",
        context! {
            apikeys: database::get_recent_api_keys(&pool).await.unwrap_or(vec![]),
            permissions: user.permissions,
            created: HumanTime::from(user.created).to_text_en(Accuracy::Rough, Tense::Past),
            username: user.username
        },
    ))
}

#[get("/settings/delete?<error>")]
pub async fn delete(_user: User, error: Option<String>) -> Result<Template, String> {
    let error_message = match error.unwrap_or_default().as_str() {
        "INVALID_PASS" => "That password was incorrect",
        "VERIFICATION_FAILED" => "Could not verify your account, please contact the server administrator",
        "DELETION_FAILED" => "Could not delete your account, please contact the server administrator",
        _ => "",
    };
    Ok(Template::render(
        "delete",
        context! {
            error: error_message
        },
    ))
}

#[get("/settings/key/new")]
pub async fn new_api_key(user: User) -> Result<Template, String> {
    Ok(Template::render(
        "newapikey",
        context! {
            permissions: user.permissions,
        },
    ))
}
