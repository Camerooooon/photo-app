use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sqlx::{Error, Pool};
use sqlx_mysql::MySql;

use crate::models::Permission;

use super::user::User;

/// Writes a new user to the database
pub async fn write_user(
    pool: &Pool<MySql>,
    user: &User,
    hashed_password: String,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO users VALUES(?, ?, ?, ?, ?)",
        user.created
            .duration_since(UNIX_EPOCH)
            .expect("Unexpected duration")
            .as_millis() as u64,
        user.username,
        hashed_password,
        user.permissions
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(","),
        0
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn fetch_user(pool: &Pool<MySql>, username: &String) -> Result<User, Error> {
    let response = sqlx::query!("SELECT * FROM users WHERE username = ?", username)
        .fetch_one(pool)
        .await?;
    Ok(User {
        created: SystemTime::UNIX_EPOCH + Duration::from_millis(response.created as u64),
        username: response.username,
        permissions: response
            .permissions
            .split(",")
            .filter(|s| !s.is_empty())
            .into_iter()
            .map(|s| Permission::try_from(s).unwrap_or(Permission::Unknown))
            .collect(),
        id: Some(response.id)
    })
}

pub async fn delete_user(pool: &Pool<MySql>, username: &String) -> Result<(), Error> {
    sqlx::query!("DELETE FROM users WHERE username = ?", username)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn verify_hash(
    pool: &Pool<MySql>,
    username: &String,
    password: String,
) -> Result<bool, Error> {
    let response = sqlx::query!("SELECT * FROM users WHERE username = ?", username)
        .fetch_one(pool)
        .await?;
    Ok(bcrypt::verify(password, &response.hashed_password).expect("Unable to compare the hashes"))
}
