use chrono_humanize::{Accuracy, HumanTime, Tense};
use rocket::http::ContentType;
use rocket::{fs::NamedFile, State};
use rocket_dyn_templates::{context, Template};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::images::image_repository::get_recent_images;
use crate::keys::key_repository::get_recent_api_keys;
use crate::users::user::User;

#[get("/")]
pub async fn index(pool: &State<Pool<MySql>>) -> Result<Template, String> {
    Ok(Template::render(
        "index",
        context! {
            name: get_recent_images(&pool).await.map_err(|_| "Could not fetch recent images from database")?,
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
            apikeys: get_recent_api_keys(&pool, &user).await.unwrap_or(vec![]),
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
        "VERIFICATION_FAILED" => {
            "Could not verify your account, please contact the server administrator"
        }
        "DELETION_FAILED" => {
            "Could not delete your account, please contact the server administrator"
        }
        _ => "",
    };
    Ok(Template::render(
        "delete",
        context! {
            error: error_message
        },
    ))
}
