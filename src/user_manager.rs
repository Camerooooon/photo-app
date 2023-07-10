use bcrypt::DEFAULT_COST;
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
) -> Result<String, String> {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    let hashed_password =
        bcrypt::hash(password, DEFAULT_COST).expect("Could not hash password?!?!?!");

    let user = User {
        username,
        created: SystemTime::now(),
        permissions: vec![],
    };

    database::write_user(&pool, &user, hashed_password)
        .await
        .map_err(|e| format!("Could not save the user to the database: {}", e))?;

    Ok("OK".to_string())
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
            e => Redirect::to("/login?error=VERIFACTION_FAILED"),
        })?;

    if !verified {
        return Ok(Redirect::to("/login?error=INVALID_USER_PASS"));
    }

    let user = database::fetch_user(pool, &username)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Redirect::to("/login?error=INVALID_USER_PASS"),
            e => Redirect::to("/login?error=VERIFACTION_FAILED"),
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
