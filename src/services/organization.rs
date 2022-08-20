use crate::repos::user as UserRepo;
use crate::repos::vacation as VacationRepo;
use crate::types::user::User;
use crate::types::vacation::Vacation;
use crate::utils::errors::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub fn get_all_users(organization_id: Uuid, conn: &PgConnection) -> Result<Vec<User>, Errors> {
    UserRepo::get_all_by_organization(organization_id, conn)
}

pub fn get_all_vacations(
    organization_id: Uuid,
    conn: &PgConnection,
) -> Result<Vec<Vacation>, Errors> {
    UserRepo::get_all_by_organization(organization_id, conn)?
        .iter()
        .map(|user| VacationRepo::get_by_user_id(user.id, conn))
        .collect::<Result<Vec<Vec<Vacation>>, Errors>>()
        .map(|res| res.into_iter().flatten().collect())
}
