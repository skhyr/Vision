use lib::repos::transition as TransitionRepo;
// use lib::repos::user as UserRepo;
use lib::services::user as UserService;
use lib::types::transition::Transition;
use lib::types::vacation::Vacation;
use lib::utils::establish_connection::establish_connection;
use uuid::Uuid;

fn main() {
    let connection = establish_connection();
    let my_id: Uuid = Uuid::parse_str("f751002c-20de-4f33-aabb-6b42adfa193e").unwrap();

    let res = UserService::get_info(my_id, &connection).unwrap();

    println!("{:?}", res);
}
