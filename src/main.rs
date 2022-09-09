#[macro_use]
extern crate rocket;
use lib::api::initial::init_routes;
/* use lib::repos::{TransitionRepo, VacationRepo};
use lib::services::{organization, transition, vacation};
use lib::types::{transition::NewTransition, user::NewUser, vacation::NewVacation, Vacation};
use lib::utils::establish_connection::establish_connection; */

#[launch]
fn rocket() -> _ {
    /* let conn = establish_connection();
    let user_id = uuid::Uuid::parse_str("31b67891-d7e0-49ab-bdf3-f3f7d24bdb86").unwrap();
    let organization_id = uuid::Uuid::parse_str("e795b831-db6e-429e-b47a-a3c600ad3fa5").unwrap();

    let new_vacations = NewVacation {
        start_date: chrono::NaiveDate::from_ymd(2021, 09, 20),
        end_date: chrono::NaiveDate::from_ymd(2021, 09, 22),
        title: "Domki z eiti".into(),
    };

    let nt = NewTransition {
        date: chrono::NaiveDate::from_ymd(2022, 06, 01),
        fraction: 1.0,
    };

    // transition::add(nt, user_id, &conn);

    let new_user = NewUser {
        name: "Kacper".into(),
        surname: "Samuś".into(),
        accounting_day: 1,
        access_code: 4200,
    }; */

    // organization::add_user(new_user, organization_id, &conn);

    // vacation::add(new_vacations, user_id, &conn);

    init_routes()
}
