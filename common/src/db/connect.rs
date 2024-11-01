
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {

    let database_url= "postgres://postgres:password@localhost:5432/diesel_demo".to_string();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}