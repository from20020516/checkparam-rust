use checkparam::models::NewUser;
use checkparam::schema::users as users_schema;
use checkparam::utils::establish_connection;
use diesel::prelude::*;

fn main() {
    let connection = establish_connection();
    let new_user = NewUser {
        name: String::from("new_user"),
    };
    diesel::insert_into(users_schema::dsl::users)
        .values(new_user)
        .execute(&connection)
        .expect("Error saving new user");
}
