use rocket::http::CookieJar;
use rocket_dyn_templates::{context, Template};

#[get("/login?<error>&<notice>")]
pub fn login(
    cookies: &CookieJar<'_>,
    notice: Option<String>,
    error: Option<String>,
) -> Result<Template, String> {
    let notice_message = match notice.unwrap_or_default().as_str() {
        "ACCOUNT_CREATED" => {
            "Your account has been created, please log in with your username and password"
        }
        _ => "",
    };
    let error_message = match error.unwrap_or_default().as_str() {
        "INVALID_USER_PASS" => "Invalid username or password",
        "VERIFICATION_FAILED" => "We were unable to log you in, please try again later",
        _ => "",
    };
    Ok(Template::render(
        "login",
        context! {
            notice: notice_message,
            error: error_message,
            signed_in: cookies.get_private("username").is_some(),
        },
    ))
}

#[get("/register?<error>")]
pub async fn register(cookies: &CookieJar<'_>, error: Option<String>) -> Result<Template, String> {
    let error_message = match error.unwrap_or_default().as_str() {
        "DUPLICATE_USERNAME" => "That username is already taken, please try another!",
        "INVALID_USERNAME" => {
            "Username is invalid, usernames must only contain letters and numbers"
        }
        "SHORT_USERNAME" => {
            "That username is too short. Usernames must be at least 3 characters long"
        }
        "SHORT_PASSWORD" => "Please enter a longer password",
        "REGISTRATION_FAILED" => "We were unable to add you to our systems, please try again later",
        _ => "",
    };
    Ok(Template::render(
        "register",
        context! {
            error: error_message,
            signed_in: cookies.get_private("username").is_some(),
        },
    ))
}
