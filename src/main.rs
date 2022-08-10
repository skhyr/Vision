use lib::repos::user as UserRepo;
use lib::types::user::User;
use lib::utils::establish_connection::establish_connection;

fn main() {
    let connection = establish_connection();
    let new_user = User {
        id: uuid::Uuid::new_v4(),
        name: "Maciek".into(),
        surname: "Michalczyk".into(),
        access_code: 2137,
        accounting_day: 1,
    };
    // UserRepo::insert(new_user, &connection);
    UserRepo::get_all(&connection).map(|v| println!("{:?}", v));
}
