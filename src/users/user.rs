use std::time::SystemTime;

use rocket::{serde::{Serialize, Deserialize}, request::{FromRequest, Outcome}, Request, State};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{models::Permission, keys::key_repository::fetch_key};

use super::user_repository::fetch_user;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub created: SystemTime,
    pub permissions: Vec<Permission>,
}

pub struct AuthenticatedUser {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, ()> {
        let pool = request.guard::<&State<Pool<MySql>>>().await.unwrap();
        match request.cookies().get_private("username") {
            Some(username) => {
                match fetch_user(pool, &username.value().to_string()).await {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Forward(()),
                }
            }
            None => Outcome::Forward(()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, ()> {
        let pool = request.guard::<&State<Pool<MySql>>>().await.unwrap();
        match request.cookies().get_private("username") {
            Some(username) => {
                match fetch_user(pool, &username.value().to_string()).await {
                    Ok(user) => {
                        if user.permissions.is_empty() {
                            Outcome::Forward(())
                        } else {
                            Outcome::Success(AuthenticatedUser { user })
                        }
                    }
                    Err(_) => Outcome::Forward(()),
                }
            }
            None => {
                // Do ApiKey authentication
                match fetch_key(pool, &request.headers().get_one("authorization").unwrap_or("").to_string()).await {
                    Ok(key) => {
                        match fetch_user(pool, &key.owner.to_string()).await {
                            Ok(user) => {
                                if user.permissions.is_empty() {
                                    Outcome::Forward(())
                                } else {
                                    Outcome::Success(AuthenticatedUser { user })
                                }
                            }
                            Err(_) => Outcome::Forward(()),
                        }
                    },
                    Err(_) => Outcome::Forward(()),
                }
            },
        }
    }
}
