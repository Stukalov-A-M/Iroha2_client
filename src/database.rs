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

#[allow(non_snake_case)]
pub mod queries {
    use crate::database::connection::establish_connection;
    use crate::models::{NewUser, Users};
    use crate::schema::users;
    use diesel::query_dsl::select_dsl::SelectDsl;
    use diesel::{insert_into, RunQueryDsl, SelectableHelper};

    pub fn print_all_users() {
        let mut connection = establish_connection();
        let result = users::table
            .select(Users::as_select())
            .load(&mut connection)
            .unwrap();

        result.iter().for_each(|user| println!("{user}"))
    }

    pub fn add_user(user: NewUser) {
        let mut connection = establish_connection();

        let result = insert_into(users::table)
            .values(user)
            .returning(Users::as_returning())
            .get_result(&mut connection)
            .unwrap();
        println!("{result}")
    }
}
