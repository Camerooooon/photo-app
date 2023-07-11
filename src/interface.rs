use rocket::http::ContentType;
use rocket::{State, fs::NamedFile};
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

#[get("/semantic/dist/semantic.min.js")]
pub async fn semantic_js() -> (ContentType, NamedFile) {
    let content_type = ContentType::new("application", "javascript");
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
pub async fn login(notice: Option<String>, error: Option<String>) -> Result<Template, String> {
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
