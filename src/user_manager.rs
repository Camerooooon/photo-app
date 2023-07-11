use bcrypt::DEFAULT_COST;
use regex::Regex;
use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    request::{FromRequest, Outcome},
    response::Redirect,
    Request, State,
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

pub struct AuthenticatedUser {
    user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, ()> {
        let pool = request.guard::<&State<Pool<MySql>>>().await.unwrap();
        match request.cookies().get_private("username") {
            Some(username) => {
                match database::fetch_user(pool, &username.value().to_string()).await {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Forward(()),
                }
            }
            None => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, ()> {
        let pool = request.guard::<&State<Pool<MySql>>>().await.unwrap();
        match request.cookies().get_private("username") {
            Some(username) => {
                match database::fetch_user(pool, &username.value().to_string()).await {
                    Ok(user) => {
                        if user.permissions.is_empty() {
                            Outcome::Forward(())
                        } else {
                            Outcome::Success(AuthenticatedUser { user })
                        }
                    }
                    Err(_) => Outcome::Forward(()),
                }
            }
            None => Outcome::Forward(()),
        }
    }
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
    let user = database::fetch_user(pool, &username).await.ok();

    if user.is_some() {
        return Err(Redirect::to("/register?error=DUPLICATE_USERNAME"));
    }

    let hashed_password = bcrypt::hash(password, DEFAULT_COST).expect("Could not hash password");

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

    Ok(Redirect::to("/dashboard"))
}

#[get("/api/user/status")]
pub async fn status(user: AuthenticatedUser) -> Result<String, String> {
    return Ok(format!("Logged in to: {}", user.user.username));
}
