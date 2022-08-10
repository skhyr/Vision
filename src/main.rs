use lib::repos::transition as TransitionRepo;
use lib::repos::user as UserRepo;
use lib::types::transition::Transition;
use lib::utils::establish_connection::establish_connection;
use uuid::Uuid;

fn main() {
    let connection = establish_connection();
    /* let new_user = User {
        id: uuid::Uuid::new_v4(),
        name: "Maciek".into(),
        surname: "Michalczyk".into(),
        access_code: 2137,
        accounting_day: 1,
    }; */
    // UserRepo::insert(new_user, &connection);
    let my_id: Uuid = Uuid::parse_str("f751002c-20de-4f33-aabb-6b42adfa193e").unwrap();
    let my_user = UserRepo::get_by_id(my_id, &connection).unwrap();
    println!("{:?}", my_user);

    // action
    /* let newVacation = Vacation {
        id: Uuid::new_v4(),
        user_id: my_id,
        title: "Chorobowe".to_string(),
        start_date: chrono::NaiveDate::from_ymd(2022, 06, 18),
        end_date: chrono::NaiveDate::from_ymd(2022, 06, 18),
    }; */

    let newTransition = Transition {
        id: Uuid::new_v4(),
        user_id: my_id,
        date: chrono::NaiveDate::from_ymd(2022, 1, 1),
    };

    // let res = TransitionRepo::insert(newTransition, &connection).unwrap();

    // VacationRepo::insert(newVacation, &connection);
    // let res = VacationRepo::get_all(&connection).unwrap();
    let res = TransitionRepo::get_by_user_id(my_id, &connection).unwrap();
    println!("{:?}", res);
}
