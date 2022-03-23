use checkparam::models::User;
use checkparam::schema::users::dsl::*;
use checkparam::utils::establish_connection;
use diesel::prelude::*;

fn main() {
    let conn = establish_connection();
    let user = users.filter(id.eq(1)).first::<User>(&conn);
    println!("{:?}", user);
}
