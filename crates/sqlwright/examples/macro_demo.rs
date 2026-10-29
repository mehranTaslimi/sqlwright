use sqlwright::{
    Table,
    pg::Pg,
    table,
    types::{BigInt, Nullable, Text},
};

#[table(name = "users", backend = Pg)]
pub struct User {
    id: BigInt,
    name: Text,
    bio: Nullable<Text>,
}

fn main() {
    println!("{:?}", User)
}
