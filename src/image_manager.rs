use std::{fs::File, time::SystemTime};

use models::ImageMeta;
use rocket::serde::json::Json;
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{models, database};
use crate::rocket::tokio::io::AsyncReadExt;

use image::DynamicImage;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::data::ToByteUnit;
use rocket::{Data, State};

/// Route for handling image loading
#[get("/api/image/<url>")]
pub async fn get_image(url: String, pool: &State<Pool<MySql>>) -> Result<Json<ImageMeta>, String> {

    let meta = database::read_image_metadata(pool, url).await.map_err(|e| format!("Failed to fetch image: {}", e))?;
    Ok(Json(meta))

}

/// Route for handling business logic for uploading of an image
#[post("/upload", data = "<image>")]
pub async fn upload_image(image: Data<'_>, pool: &State<Pool<MySql>>) -> Result<String, String> {
    // Read the image data from the request
    let mut buf = Vec::new();
    if let Err(e) = image.open(2.megabytes()).read_to_end(&mut buf).await {
        return Err(format!("Failed to read image data: {}", e));
    }

    let img = image::load_from_memory(&buf).map_err(|e| format!("Failed to load image: {}", e))?;

    let meta_data = generate_metadata()?;

    save_image(&img, &meta_data)?;

    database::write_image(pool, &meta_data).await.map_err(|_| "Failed to save image")?;

    Ok("Image uploaded successfully".into())
}

/// Saves an image to the file system given a list bytes
/// Returns a human readable error if unsuccessful
/// Does **not** save image meta data to database
fn save_image(img: &DynamicImage, meta_data: &ImageMeta) -> Result<(), String> {
    // Save the image to a file
    let mut file = File::create(format!("./images/full/{}.jpg", meta_data.url))
        .map_err(|e| format!("Failed to create file: {}", e))?;
    img.write_to(&mut file, image::ImageOutputFormat::Jpeg(100))
        .map_err(|e| format!("Failed to write image data: {}", e))?;

    let thumbnail = img.thumbnail(500, 500);

    let mut file = File::create(format!("./images/thumbnail/{}.jpg", meta_data.url))
        .map_err(|e| format!("Failed to create file: {}", e))?;
    thumbnail
        .write_to(&mut file, image::ImageOutputFormat::Jpeg(100))
        .map_err(|e| format!("Failed to write image data: {}", e))?;

    Ok(())
}

/// Generates image meta data from the current time
fn generate_metadata() -> Result<ImageMeta, String> {
    // Generate a random name for the image file
    let filename: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>();
    Ok(ImageMeta {
        privacy: models::Privacy::Unspecified,
        uploaded: SystemTime::now(),
        print_available: false,
        url: filename,
        name: "Unnamed".to_string(),
        categories: vec![],
    })
}
