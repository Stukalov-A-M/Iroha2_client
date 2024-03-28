use super::*;
use diesel::prelude::*;

mod connection {
    use diesel::{Connection, PgConnection};
    use dotenv::dotenv;
    use std::env;

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

pub mod queries {
    use crate::database::connection::establish_connection;
    use crate::models::Users;
    use crate::schema::users::dsl::users;
    use diesel::query_dsl::select_dsl::SelectDsl;
    use diesel::{RunQueryDsl, SelectableHelper};

    pub fn print_all_users() {
        let mut connection = establish_connection();
        let result = users
            .select(Users::as_select())
            .load(&mut connection)
            .unwrap();

        for user in result {
            println!("{:?}", user)
        }
    }
}
