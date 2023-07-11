use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sqlx::{Error, Pool};
use sqlx_mysql::{MySql, MySqlPool};

use crate::models::{Category, ImageGroup, ImageMeta, Permission, Privacy, User};

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

/// Writes a new user to the database
pub async fn write_user(
    pool: &Pool<MySql>,
    user: &User,
    hashed_password: String,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO users VALUES(?, ?, ?, ?)",
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
            .join(",")
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
    })
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

/// Writes some image metadata to the specified database pool
pub async fn write_image(pool: &Pool<MySql>, metadata: &ImageMeta) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO images VALUES(?, ?, ?, ?, ?, ?, ?)",
        metadata.file_extension,
        metadata
            .uploaded
            .duration_since(UNIX_EPOCH)
            .expect("Unexpected duration")
            .as_millis() as u64,
        metadata.print_available,
        metadata.url,
        metadata.name,
        metadata.privacy,
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

/// Get the recent uploaded images from the sql pool
/// Will not show images marked as `Unlisted`
pub async fn get_recent_images(pool: &Pool<MySql>) -> Result<Vec<ImageMeta>, Error> {
    let images = sqlx::query!("SELECT * FROM images ORDER BY uploaded LIMIT 50")
        .fetch_all(pool)
        .await?;
    let mut to_return: Vec<ImageMeta> = vec![];
    for image in images {
        let meta = ImageMeta {
            file_extension: image.file_extension,
            privacy: crate::models::Privacy::Unspecified,
            uploaded: SystemTime::UNIX_EPOCH + Duration::from_millis(image.uploaded as u64),
            print_available: {
                if image.print_available == 0 {
                    true
                } else {
                    false
                }
            },
            url: image.url,
            name: image.name,
            categories: image
                .categories
                .split(",")
                .into_iter()
                .map(|s| Category::try_from(s).unwrap_or(Category::Unknown))
                .collect(),
        };
        if meta.privacy.ne(&Privacy::Unlisted) {
            to_return.push(meta);
        }
    }
    Ok(to_return)
}

pub async fn read_image_metadata(pool: &Pool<MySql>, url: String) -> Result<ImageMeta, Error> {
    let response = sqlx::query!("SELECT * FROM images WHERE url = ?", url)
        .fetch_one(pool)
        .await?;
    Ok(ImageMeta {
        file_extension: response.file_extension,
        privacy: crate::models::Privacy::Unspecified,
        uploaded: SystemTime::UNIX_EPOCH + Duration::from_millis(response.uploaded as u64),
        print_available: {
            if response.print_available == 0 {
                true
            } else {
                false
            }
        },
        url,
        name: response.name,
        categories: response
            .categories
            .split(",")
            .into_iter()
            .map(|s| Category::try_from(s).unwrap_or(Category::Unknown))
            .collect(),
    })
}
