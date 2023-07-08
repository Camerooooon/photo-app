#[macro_use]
extern crate rocket;

pub mod database;
pub mod image_manager;
pub mod models;

use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            name: "test",
        },
    )
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    let database_url = "mysql://cameron:pass@127.0.0.1/photoapp";
    let pool = database::connect_database(database_url)
        .await
        .expect("Unable to connect to the database, the server could not be started!");
    database::initalise_database(&pool)
        .await
        .expect("Failed to initalise database");

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![image_manager::upload_image, image_manager::get_image])
        .attach(Template::fairing())
        .manage(pool)
}
