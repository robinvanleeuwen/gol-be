#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{RunCount, NewRunCount};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn new_run_count<'a>(conn: &PgConnection, run: i32, count: i32) -> QueryResult<RunCount>{
    use schema::runcount;
    let new_run_count = NewRunCount {
        run: &Some(run),
        count: &Some(count),
    };

    let result = diesel::insert_into(runcount::table)
        .values(&new_run_count)
        .get_result(conn);

    result

}