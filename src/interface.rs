use rocket::State;
use rocket_dyn_templates::{Template, context};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::database;


#[get("/")]
pub async fn index(pool: &State<Pool<MySql>>) -> Result<Template, String> {
    Ok(Template::render(
        "index",
        context! {
            name: database::get_recent_images(&pool).await.map_err(|_| "Could not fetch recent images from database")?,
        },
    ))
}
