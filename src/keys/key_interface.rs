use rocket_dyn_templates::{Template, context};

use crate::users::user::User;

#[get("/settings/key/new")]
pub async fn new_api_key(user: User) -> Result<Template, String> {
    Ok(Template::render(
        "newapikey",
        context! {
            permissions: user.permissions,
        },
    ))
}