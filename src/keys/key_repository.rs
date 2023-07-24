use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sqlx::{Error, Pool};
use sqlx_mysql::MySql;

use crate::{models::Permission, users::user::User};

use super::key::ApiKey;

/// Writes a `ApiKey` to a specified database pool
pub async fn write_key(pool: &Pool<MySql>, key: &ApiKey) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO apikeys VALUES(?, ?, ?, ?, ?, ?)",
        key.created
            .duration_since(UNIX_EPOCH)
            .expect("Unexpected duration")
            .as_millis() as u64,
        key.owner,
        key.secret,
        key.permissions
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(","),
        key.expires.as_millis() as i32,
        0
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn fetch_key(pool: &Pool<MySql>, secret: &String) -> Result<ApiKey, Error> {
    let response = sqlx::query!("SELECT * FROM apikeys WHERE secret = ?", secret)
        .fetch_one(pool)
        .await?;
    Ok(ApiKey {
        created: SystemTime::UNIX_EPOCH + Duration::from_millis(response.created as u64),
        owner: response.owner,
        secret: response.secret,
        permissions: response
            .permissions
            .split(",")
            .filter(|s| !s.is_empty())
            .into_iter()
            .map(|s| Permission::try_from(s).unwrap_or(Permission::Unknown))
            .collect(),
        expires: Duration::from_millis(response.expires as u64),
        id: Some(response.id),
    })
}

pub async fn get_recent_api_keys(pool: &Pool<MySql>, user: &User) -> Result<Vec<ApiKey>, Error> {
    let keys = sqlx::query!("SELECT * FROM apikeys WHERE owner = ? ORDER BY created LIMIT 50", user.username)
        .fetch_all(pool)
        .await?;
    let mut to_return: Vec<ApiKey> = vec![];
    for key in keys {
        let apikey = ApiKey {
            created: SystemTime::UNIX_EPOCH + Duration::from_millis(key.created as u64),
            owner: key.owner,
            secret: key.secret,
            permissions: key
                .permissions
                .split(",")
                .filter(|s| !s.is_empty())
                .into_iter()
                .map(|s| Permission::try_from(s).unwrap_or(Permission::Unknown))
                .collect(),
            expires: Duration::from_millis(key.expires as u64),
            id: Some(key.id)
        };
        to_return.push(apikey);
    }
    Ok(to_return)
}
