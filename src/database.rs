use sqlx::{Error, Pool};
use sqlx_mysql::{MySql, MySqlPool};

pub async fn connect_database(database_url: &str) -> Result<Pool<MySql>, Error> {
    MySqlPool::connect(database_url).await
}

pub async fn initalise_database(pool: &Pool<MySql>) -> Result<(), Error> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
