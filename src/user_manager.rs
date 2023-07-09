use bcrypt::DEFAULT_COST;
use rocket::{form::Form, State};
use sqlx::Pool;
use sqlx_mysql::MySql;
use std::time::SystemTime;

use crate::{models::User, database};

#[derive(FromForm)]
pub struct UserCredentials {
    username: String,
    password: String,
}

#[post("/api/register", data = "<credentials>")]
pub async fn signup(credentials: Form<UserCredentials>, pool: &State<Pool<MySql>>) -> Result<String, String> {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    let hashed_password = bcrypt::hash(password, DEFAULT_COST).expect("Could not hash password?!?!?!");

    let user = User {
        username,
        created: SystemTime::now(),
        permissions: vec![],
    };

    database::write_user(&pool, &user, hashed_password).await.map_err(|e| format!("Could not save the user to the database: {}", e))?;

    Ok("OK".to_string())

}

#[post("/api/login", data = "<credentials>")]
pub async fn login(credentials: Form<UserCredentials>, pool: &State<Pool<MySql>>) -> Result<String, String> {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    let verified = database::verify_hash(pool, &username, password).await.map_err(|e| format!("Unable to verify your password: {}", e))?;

    if !verified {
        return Ok("Invalid".to_string());
    }

    let user = database::fetch_user(pool, &username);

    Ok("OK".to_string())
    

}
