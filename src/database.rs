use diesel::pg::PgConnection;
use diesel::prelude::*;


pub fn establish_connection() -> PgConnection {
    dotenv::from_filename(".env").ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
