use crate::repos::vacation as VacationRepo;
use crate::services::transition as TransitionService;
use crate::services::user as UserService;
use crate::types::stats::Stats;
use crate::types::transition::Transition;
use crate::types::vacation::{NewVacation, Vacation};
use crate::utils::errors::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub fn calc_vacation(vacation: &Vacation, transitions: &Vec<Transition>) -> Result<Stats, Errors> {
    UserService::gen_stats(vec![vacation], transitions)
}

pub fn get_calc_vacation(
    vacation: &Vacation,
    user_id: Uuid,
    conn: &PgConnection,
) -> Result<Stats, Errors> {
    let transitions = TransitionService::get_sorted_transitions(user_id, conn)?;
    calc_vacation(vacation, &transitions)
}

pub fn add(
    new_vacation: NewVacation,
    user_id: Uuid,
    conn: &PgConnection,
) -> Result<Vacation, Errors> {
    let vacation = Vacation {
        id: Uuid::new_v4(),
        user_id,
        title: new_vacation.title,
        start_date: new_vacation.start_date,
        end_date: new_vacation.end_date,
    };
    VacationRepo::insert(vacation, conn)
}
