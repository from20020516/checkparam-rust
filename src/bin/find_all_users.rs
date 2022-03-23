use checkparam::models::User;
use checkparam::schema::users::dsl::*;
use checkparam::utils::establish_connection;
use diesel::prelude::*;

fn main() {
    let conn = establish_connection();
    let all_users = users.load::<User>(&conn);
    println!("{:?}", all_users);
}
