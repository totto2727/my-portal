use std::error::Error;

use sea_orm::{Database, DatabaseConnection};

use crate::env;

pub async fn get_connection() -> Result<DatabaseConnection, Box<dyn Error>> {
    let username = env::env_postgres_user()?;
    let password = env::env_postgres_password()?;
    let database = env::env_postgres_db()?;
    let domain = env::env_postgres_domain()?;

    Ok(Database::connect(format!("postgres://{}:{}@{}/{}", username, password, domain, database)).await?)
}
