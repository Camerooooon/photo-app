use std::time::SystemTime;

use rocket::{
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request, State,
};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{keys::{key_repository::fetch_key_by_secret, key::ApiKey}, models::Permission};

use super::user_repository::fetch_user;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub created: SystemTime,
    pub permissions: Vec<Permission>,
    pub id: Option<u32>,
}

pub struct AuthenticatedUser {
    pub user: User,
    pub key: Option<ApiKey>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, ()> {
        let pool = request.guard::<&State<Pool<MySql>>>().await.unwrap();
        match request.cookies().get_private("username") {
            Some(username) => match fetch_user(pool, &username.value().to_string()).await {
                Ok(user) => Outcome::Success(user),
                Err(_) => Outcome::Forward(()),
            },
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
            Some(username) => match fetch_user(pool, &username.value().to_string()).await {
                Ok(user) => {
                    if user.permissions.is_empty() {
                        Outcome::Forward(())
                    } else {
                        Outcome::Success(AuthenticatedUser { user, key: None })
                    }
                }
                Err(_) => Outcome::Forward(()),
            },
            None => {
                // Do ApiKey authentication
                match fetch_key_by_secret(
                    pool,
                    &request
                        .headers()
                        .get_one("authorization")
                        .unwrap_or("")
                        .to_string(),
                )
                .await
                {
                    Ok(key) => match fetch_user(pool, &key.owner.to_string()).await {
                        Ok(user) => {
                            if user.permissions.is_empty() {
                                Outcome::Forward(())
                            } else {
                                Outcome::Success(AuthenticatedUser { user, key: Some(key) })
                            }
                        }
                        Err(_) => Outcome::Forward(()),
                    },
                    Err(_) => Outcome::Forward(()),
                }
            }
        }
    }
}
