use std::path::PathBuf;
use std::{fs::File, time::SystemTime};

use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::images::image::ImageMeta;
use crate::rocket::tokio::io::AsyncReadExt;

use image::DynamicImage;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::data::ToByteUnit;
use rocket::{Data, State};

use super::image::Privacy;
use super::image_repository::{read_image_metadata, write_image};

/// Route for handling image loading
#[get("/api/image/<url>")]
pub async fn get_image_meta(
    url: String,
    pool: &State<Pool<MySql>>,
) -> Result<Json<ImageMeta>, String> {
    let meta = read_image_metadata(pool, url)
        .await
        .map_err(|e| format!("Failed to fetch image: {}", e))?;
    Ok(Json(meta))
}

/// Route for getting image
#[get("/i/<url>")]
pub async fn get_image(url: String, pool: &State<Pool<MySql>>) -> Result<NamedFile, String> {
    let meta = read_image_metadata(pool, url)
        .await
        .map_err(|e| format!("Failed to fetch image: {}", e))?;
    let image_path = PathBuf::from(format!("./images/full/{}.{}", meta.id, meta.file_extension));
    NamedFile::open(image_path)
        .await
        .map_err(|e| format!("The image could not be read for you: {}", e))
}

/// Route for getting thumbnail
#[get("/thumb/<url>")]
pub async fn get_thumbnails(url: String, pool: &State<Pool<MySql>>) -> Result<NamedFile, String> {
    let meta = read_image_metadata(pool, url)
        .await
        .map_err(|e| format!("Failed to fetch image: {}", e))?;
    let image_path = PathBuf::from(format!("./images/thumbnail/{}.jpg", meta.id));
    NamedFile::open(image_path)
        .await
        .map_err(|e| format!("The image could not be read for you: {}", e))
}

/// Route for handling business logic for uploading of an image
#[post("/upload", data = "<image>")]
pub async fn upload_image(image: Data<'_>, pool: &State<Pool<MySql>>) -> Result<String, String> {
    // Read the image data from the request
    let mut buf = Vec::new();
    if let Err(e) = image.open(50.megabytes()).read_to_end(&mut buf).await {
        return Err(format!("Failed to read image data: {}", e));
    }

    let img = image::load_from_memory(&buf).map_err(|e| format!("Failed to load image: {}", e))?;

    // Extracts the image format from the image
    let extension = match image::guess_format(&buf) {
        Ok(format) => format.extensions_str()[0],
        Err(_) => {
            return Err("Image format was not recognized!".to_string());
        }
    };

    let meta_data = generate_metadata(extension.to_string())?;

    save_image(&img, &meta_data)?;

    write_image(pool, &meta_data)
        .await
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok("Image uploaded successfully".into())
}

/// Saves an image to the file system given an `DynamicImage`
/// Returns a human readable error if unsuccessful
///
/// Does **not** save image meta data to database
///
/// The image will save in the `./images/full/` directory with the URL specified in meta_data as
/// the name and with the extension of the uploaded image
/// The image will also save a 500x500 thumbnail in the `./images/thumbnail/` directory with the URL
/// specified as the name and the jpg extension
fn save_image(img: &DynamicImage, meta_data: &ImageMeta) -> Result<(), String> {
    // Save the image to a file
    let mut file = File::create(format!(
        "./images/full/{}.{}",
        meta_data.id, meta_data.file_extension
    ))
    .map_err(|e| format!("Failed to create file: {}", e))?;
    img.write_to(&mut file, image::ImageOutputFormat::Jpeg(100))
        .map_err(|e| format!("Failed to write image data: {}", e))?;

    let thumbnail = img.thumbnail(500, 500);

    let mut file = File::create(format!("./images/thumbnail/{}.jpg", meta_data.id))
        .map_err(|e| format!("Failed to create file: {}", e))?;
    thumbnail
        .write_to(&mut file, image::ImageOutputFormat::Jpeg(100))
        .map_err(|e| format!("Failed to write image data: {}", e))?;

    Ok(())
}

/// Generates image meta data from the current time
fn generate_metadata(file_extension: String) -> Result<ImageMeta, String> {
    // Generate a random name for the image file
    let filename: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>();
    Ok(ImageMeta {
        file_extension,
        privacy: Privacy::Unspecified,
        uploaded: SystemTime::now(),
        print_available: false,
        id: filename,
        name: "Unnamed".to_string(),
        categories: vec![],
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_metadata_test() {
        let meta = generate_metadata("jpg".to_string()).unwrap();
        assert_eq!(meta.id.len(), 10);
        assert_eq!(meta.file_extension, "jpg");
    }
}
