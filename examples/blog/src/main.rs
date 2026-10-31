use sqlwright::{
    generate,
    pg::Pg,
    table,
    types::{BigInt, Nullable, Text},
};
use sqlx::postgres::PgPoolOptions;

#[table(name = "users", backend = Pg)]
pub struct User {
    id: Nullable<BigInt>,
    name: Text,
    bio: Nullable<Text>,
}

#[table(name = "posts", backend = Pg)]
pub struct Post {
    id: BigInt,
    user_id: BigInt,
    title: Text,
    body: Text,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/blog".to_owned());

    let pool = PgPoolOptions::new().connect(&url).await?;

    let (version,): (String,) = sqlx::query_as("SELECT version()").fetch_one(&pool).await?;
    println!("connected: {version}");

    generate! {
        tables: [
            User,
            Post
        ],
        pool,
        out : "migrations"
    };

    Ok(())
}
