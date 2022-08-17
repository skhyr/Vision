use lib::repos::transition as TransitionRepo;
// use lib::repos::user as UserRepo;
use lib::services::calculator;
use lib::types::transition::Transition;
use lib::types::vacation::Vacation;
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
    // let my_user = UserRepo::get_by_id(my_id, &connection).unwrap();
    // println!("{:?}", my_user);

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
        date: chrono::NaiveDate::from_ymd(2021, 7, 1),
        fraction: 0.5,
    };

    // let res = TransitionRepo::insert(newTransition, &connection).unwrap();
    let transitions = TransitionRepo::get_all(&connection).unwrap();
    // let res = calculator::count_used_hours(my_id, &connection);
    let gen = calculator::count_generated_hours(my_id, &connection);
    let used = calculator::count_used_hours(my_id, &connection);
    let left = calculator::count_hours_left(my_id, &connection);
    /* let res = calculator::count_days_in_vacation(&Vacation {
        id: uuid::Uuid::new_v4(),
        user_id: uuid::Uuid::new_v4(),
        title: "test".into(),
        start_date: chrono::NaiveDate::from_ymd(2022, 5, 7),
        end_date: chrono::NaiveDate::from_ymd(2022, 5, 12),
    }); */

    let vacation = Vacation {
        id: uuid::Uuid::new_v4(),
        user_id: uuid::Uuid::new_v4(),
        title: "test".into(),
        start_date: chrono::NaiveDate::from_ymd(2021, 9, 7),
        end_date: chrono::NaiveDate::from_ymd(2021, 9, 12),
    };

    // let res = calculator::match_transition_to_vacation(&vacation, &transitions);

    // VacationRepo::insert(newVacation, &connection);
    // let res = VacationRepo::get_all(&connection).unwrap();
    // let res = TransitionRepo::get_by_user_id(my_id, &connection).unwrap();
    println!("gen: {:?}", gen);
    println!("used: {:?}", used);
    println!("left: {:?}", left);
}
