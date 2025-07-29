use sqlx::{
    FromRow,
    PgPool,
    Error,
    query,
    query_as
};

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str
) -> Result<(), Error> {
    query(
        "INSERT INTO users (name, email) VALUES ($1, $2)"
    ).bind(name)
     .bind(email)
     .execute(pool)
     .await?;

    Ok(())
}

pub async fn get_user(
    pool: &PgPool,
    user_id: i32
) -> Result<User, Error> {
    let user = query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    ).bind(user_id)
     .fecth_one(pool)
     .await?;

    Ok(user)
}