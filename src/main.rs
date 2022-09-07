#[macro_use]
extern crate rocket;
use lib::api::initial::init_routes;
use lib::repos::{TransitionRepo, VacationRepo};
use lib::services::{organization, vacation};
use lib::types::{user::NewUser, vacation::NewVacation, Vacation};
use lib::utils::establish_connection::establish_connection;

#[launch]
fn rocket() -> _ {
    let conn = establish_connection();
    let user_id = uuid::Uuid::parse_str("64c11ecd-3b6e-4086-9c43-2f3e4de17025").unwrap();
    let organization_id = uuid::Uuid::parse_str("e795b831-db6e-429e-b47a-a3c600ad3fa5").unwrap();

    let new_vacations = NewVacation {
        start_date: chrono::NaiveDate::from_ymd(2021, 09, 20),
        end_date: chrono::NaiveDate::from_ymd(2021, 09, 22),
        title: "Góry".into(),
    };

    let new_user = NewUser {
        name: "Kacper".into(),
        surname: "Samuś".into(),
        accounting_day: 1,
        access_code: 4200,
    };

    // organization::add_user(new_user, organization_id, &conn);

    // vacation::add(new_vacations, user_id, &conn);

    init_routes()
}
