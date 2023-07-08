use std::time::{UNIX_EPOCH, SystemTime, Duration};

use sqlx::{Error, Pool};
use sqlx_mysql::{MySql, MySqlPool};

use crate::models::{ImageMeta, ImageGroup, Category};

pub async fn connect_database(database_url: &str) -> Result<Pool<MySql>, Error> {
    MySqlPool::connect(database_url).await
}

/// Initialise database structure
/// Creates the following tables:
///     - Images
///     - Imagegroups
pub async fn initalise_database(pool: &Pool<MySql>) -> Result<(), Error> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}

/// Writes some image metadata to the specified database pool
pub async fn write_image(pool: &Pool<MySql>, metadata: &ImageMeta) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO images VALUES(?, ?, ?, ?, ?, ?)",
        metadata.privacy,
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

/// Writes a `ImageGroup` to a specified database pool
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

pub async fn read_image_metadata(pool: &Pool<MySql>, url: String) -> Result<ImageMeta, Error> {
    let response = sqlx::query!("SELECT * FROM images WHERE url = ?", url).fetch_one(pool).await?;
    Ok(ImageMeta {
        privacy: crate::models::Privacy::Unspecified,
        uploaded: SystemTime::UNIX_EPOCH + Duration::from_millis(response.uploaded as u64),
        print_available: { if response.print_available == 0 { true } else { false} },
        url,
        name: response.name,
        categories: response.categories.split(",").into_iter().map(|s| Category::try_from(s).unwrap_or(Category::Unknown)).collect(),
    })
}
