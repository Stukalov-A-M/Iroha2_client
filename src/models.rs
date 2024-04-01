use diesel::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(non_snake_case)]
pub struct Users {
    id: i32,
    name: String,
    publicKey: String,
    privateKey: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(non_snake_case)]
pub struct NewUser {
    pub(crate) name: String,
    pub(crate) publicKey: String,
    pub(crate) privateKey: String,
}

impl Display for Users {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ id = {}, name = {}, publicKey = {}, privateKey = {} }}",
            &self.id, &self.name, &self.publicKey, &self.privateKey
        )
    }
}
