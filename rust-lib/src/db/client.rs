use std::error::Error;

use crate::env::{env_surrealdb_password, env_surrealdb_user};
use surrealdb_rs::{net::WsClient, param::Root, protocol::Ws, Surreal};

pub async fn get_client<S: Into<String>>(
    ns: S,
    db: S,
) -> Result<Surreal<WsClient>, Box<dyn Error>> {
    let username = env_surrealdb_user()?;
    let password = env_surrealdb_password()?;

    let client = Surreal::connect::<Ws>("localhost:8000").await?;

    // Signin as a namespace, database, or root user
    client
        .signin(Root {
            username: &username,
            password: &password,
        })
        .await?;

    // Select a specific namespace and database
    client.use_ns(ns).use_db(db).await?;

    Ok(client)
}
