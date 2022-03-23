use diesel::Queryable;

use crate::schema::users;

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
}
