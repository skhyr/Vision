#[macro_use]
extern crate rocket;
use lib::api::initial::init_routes;
/* use lib::repos::{TransitionRepo, VacationRepo};
use lib::services::{organization, transition, vacation};
use lib::types::{transition::NewTransition, user::NewUser, vacation::NewVacation, Vacation};
use lib::utils::establish_connection::establish_connection; */

#[launch]
fn rocket() -> _ {
    println!("Server is up");
    init_routes()
}
