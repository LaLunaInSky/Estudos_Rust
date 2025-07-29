use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod models;

use models::{
    create_user,
    get_user,
    update_user_email,
    delete_user
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
                        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .expect("Failed to create pool.");

    println!(
        "Connected to the database!"
    );

    // Create a new user
    create_user(
        &pool, "LalunaInSky", "moonaflordapele@gmail.com"
    ).await.unwrap();

    // Read the user
    let user = get_user(
        &pool, 1
    ).await.unwrap();

    println!(
        "User: {}",
        user
    );

    //
}