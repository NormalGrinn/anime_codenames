extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL has to be set");

    let res = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    res
}