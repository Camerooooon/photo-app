#[macro_use]
extern crate rocket;

pub mod user_manager;
pub mod database;
pub mod image_manager;
pub mod models;
pub mod interface;

use rocket_dyn_templates::Template;

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    let database_url = "mysql://cameron:pass@127.0.0.1/photoapp";
    let pool = database::connect_database(database_url)
        .await
        .expect("Unable to connect to the database");
    database::initalise_database(&pool)
        .await
        .expect("Failed to initalise database");

    rocket::build()
        .mount("/", routes![interface::index])
        .mount("/", routes![image_manager::upload_image, image_manager::get_image, image_manager::get_image_meta, image_manager::get_thumbnails])
        .mount("/", routes![user_manager::signup, user_manager::login, user_manager::status])
        .attach(Template::fairing())
        .manage(pool)
}
