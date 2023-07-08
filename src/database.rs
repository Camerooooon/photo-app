use std::time::UNIX_EPOCH;

use sqlx::{Error, Pool};
use sqlx_mysql::{MySql, MySqlPool};

use crate::models::{ImageMeta, ImageGroup};

pub async fn connect_database(database_url: &str) -> Result<Pool<MySql>, Error> {
    MySqlPool::connect(database_url).await
}

/// Initialise database structure
/// Creates the following tables:
///     - Images
///     - Imagegroups
pub async fn initalise_database(pool: &Pool<MySql>) -> Result<(), Error> {
    sqlx::query!("CREATE TABLE IF NOT EXISTS images (uploaded BIGINT, print_available BOOLEAN, url TEXT, name TEXT, categories TEXT)").execute(pool).await?;
    sqlx::query!("CREATE TABLE IF NOT EXISTS imagegroups (created BIGINT, name TEXT, privacy ENUM('Listed', 'Unlisted', 'Unspecified'), url TEXT)").execute(pool).await?;
    Ok(())
}

/// Writes some image metadata to the database
pub async fn write_image(pool: &Pool<MySql>, metadata: &ImageMeta) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO images VALUES(?, ?, ?, ?, ?)",
        metadata
            .uploaded
            .duration_since(UNIX_EPOCH)
            .expect("Unexpected duration")
            .as_millis() as u64,
        metadata.print_available,
        metadata.url,
        metadata.name,
        metadata
            .categories
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn write_group(pool: &Pool<MySql>, group: &ImageGroup) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO imagegroups VALUES(?, ?, ?, ?)",
        group 
            .created
            .duration_since(UNIX_EPOCH)
            .expect("Unexpected duration")
            .as_millis() as u64,
        group.name,
        group.privacy.to_string(),
        group.url
    )
    .execute(pool)
    .await?;
    Ok(())
}
