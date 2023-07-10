use bcrypt::DEFAULT_COST;
use regex::Regex;
use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    response::Redirect,
    State,
};
use sqlx::Pool;
use sqlx_mysql::MySql;
use std::time::SystemTime;

use crate::{database, models::User};

#[derive(FromForm)]
pub struct UserCredentials {
    username: String,
    password: String,
}

#[post("/api/user/register", data = "<credentials>")]
pub async fn signup(
    credentials: Form<UserCredentials>,
    pool: &State<Pool<MySql>>,
) -> Result<Redirect, Redirect> {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    // Make sure that the username is alphanumerical
    let regex = Regex::new(r"^[a-zA-Z0-9]+$").expect("Invalid regular expression");
    if !regex.is_match(username.as_str()) {
        return Err(Redirect::to("/register?error=INVALID_USERNAME"));
    }

    // Check that the username is not already taken
    let user = database::fetch_user(pool, &username)
        .await.ok();

    if user.is_some() {
        return Err(Redirect::to("/register?error=DUPLICATE_USERNAME"));
    }

    let hashed_password =
        bcrypt::hash(password, DEFAULT_COST).expect("Could not hash password");

    let user = User {
        username,
        created: SystemTime::now(),
        permissions: vec![],
    };

    database::write_user(&pool, &user, hashed_password)
        .await
        .map_err(|_| Redirect::to("/register?error=REGISTRATION_FAILED"))?;

    Ok(Redirect::to("/login?notice=ACCOUNT_CREATED"))
}

#[post("/api/user/login", data = "<credentials>")]
pub async fn login(
    credentials: Form<UserCredentials>,
    pool: &State<Pool<MySql>>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Redirect> {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    let verified = database::verify_hash(pool, &username, password)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Redirect::to("/login?error=INVALID_USER_PASS"),
            _ => Redirect::to("/login?error=VERIFACTION_FAILED"),
        })?;

    if !verified {
        return Ok(Redirect::to("/login?error=INVALID_USER_PASS"));
    }

    let user = database::fetch_user(pool, &username)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Redirect::to("/login?error=INVALID_USER_PASS"),
            _ => Redirect::to("/login?error=VERIFACTION_FAILED"),
        })?;
    cookies.add_private(Cookie::new("username", user.username));

    Ok(Redirect::to("/api/user/status"))
}

#[get("/api/user/status")]
pub async fn status(cookies: &CookieJar<'_>) -> Result<String, String> {
    let session_cookie = cookies.get_private("username");
    match session_cookie {
        Some(c) => return Ok(format!("Logged into: {}", c)),
        None => return Ok("not authenticated".to_string()),
    }
}
