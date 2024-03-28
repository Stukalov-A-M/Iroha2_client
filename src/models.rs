use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    name: String,
    publicKey: String,
    privateKey: String
}