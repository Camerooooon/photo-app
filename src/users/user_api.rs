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

use super::{
    user::User,
    user_repository::{delete_user, fetch_user, verify_hash, write_user},
};

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

    if username.len() < 3 {
        return Err(Redirect::to("/register?error=SHORT_USERNAME"));
    }

    // Make sure that the username is alphanumerical
    let regex = Regex::new(r"^[a-zA-Z0-9]+$").expect("Invalid regular expression");
    if !regex.is_match(username.as_str()) {
        return Err(Redirect::to("/register?error=INVALID_USERNAME"));
    }

    if password.len() < 3 {
        return Err(Redirect::to("/register?error=SHORT_PASSWORD"));
    }

    // Check that the username is not already taken
    let user = fetch_user(pool, &username).await.ok();

    if user.is_some() {
        return Err(Redirect::to("/register?error=DUPLICATE_USERNAME"));
    }

    let hashed_password = bcrypt::hash(password, DEFAULT_COST).expect("Could not hash password");

    let user = User {
        username,
        created: SystemTime::now(),
        permissions: vec![],
        id: None,
    };

    write_user(&pool, &user, hashed_password)
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

    let verified = verify_hash(pool, &username, password)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Redirect::to("/login?error=INVALID_USER_PASS"),
            _ => Redirect::to("/login?error=VERIFACTION_FAILED"),
        })?;

    if !verified {
        return Ok(Redirect::to("/login?error=INVALID_USER_PASS"));
    }

    let user = fetch_user(pool, &username).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => Redirect::to("/login?error=INVALID_USER_PASS"),
        _ => Redirect::to("/login?error=VERIFACTION_FAILED"),
    })?;
    cookies.add_private(Cookie::new("username", user.username));

    Ok(Redirect::to("/dashboard"))
}

#[post("/api/user/delete", data = "<password>")]
pub async fn delete(
    user: User,
    password: Form<String>,
    pool: &State<Pool<MySql>>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Redirect> {
    let verified = verify_hash(pool, &user.username, password.clone())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Redirect::to("/settings/delete?error=VERIFICATION_FAILED"),
            _ => Redirect::to("/settings/delete?error=VERIFICATION_FAILED"),
        })?;

    if !verified {
        return Ok(Redirect::to("/settings/delete?error=INVALID_PASS"));
    }

    match delete_user(pool, &user.username).await {
        Ok(_) => {
            cookies.remove_private(
                cookies
                    .get_private("username")
                    .expect("The user should have cookies at this point"),
            );
            Ok(Redirect::to("/"))
        }
        Err(_) => Err(Redirect::to("/settings/delete?error=DELETION_FAILED")),
    }
}
