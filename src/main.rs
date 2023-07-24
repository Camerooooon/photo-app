#[macro_use]
extern crate rocket;

pub mod users;
pub mod keys;

pub mod database;
pub mod image_manager;
pub mod interface;
pub mod models;
pub mod filters;

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
        .mount(
            "/",
            routes![
                interface::index,
                interface::login,
                interface::register,
                interface::dashboard,
                interface::settings,
                interface::delete,
                interface::new_api_key,
                interface::semantic_js,
                interface::semantic_css,
                interface::semantic_icon_css,
                interface::semantic_icon_woff2,
            ],
        )
        .mount(
            "/",
            routes![
                image_manager::upload_image,
                image_manager::get_image,
                image_manager::get_image_meta,
                image_manager::get_thumbnails
            ],
        )
        .mount(
            "/",
            routes![
                users::user_api::signup,
                users::user_api::delete,
                users::user_api::login,
                users::user_api::status
            ],
        )
        .mount(
            "/",
            routes![
                keys::key_api::new_key,
            ]
        )
        .attach(Template::custom(|engines| {
            engines.tera.register_filter("format_time_ago", filters::format_time_ago);
            engines.tera.register_filter("format_time_future", filters::format_duration);
        }))
        .manage(pool)
}
