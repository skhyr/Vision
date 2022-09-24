use crate::repos::OrganizationRepo;
use crate::services::authorization::{authorize, AccessLevels::*, AuthObj};
use crate::services::organization;
use crate::types::{Organization, Token, User, Vacation};
use crate::utils::errors::Errors;
use diesel::PgConnection;

/*
  auth: Co-worker
*/
pub fn get_organization(token: Token, conn: &PgConnection) -> Result<Organization, Errors> {
    let organization_id = authorize(token, Organization(AuthObj::None), conn)?;
    OrganizationRepo::get_by_id(organization_id, conn)
}

/*
  auth: Co-worker
*/
pub fn get_users(token: Token, conn: &PgConnection) -> Result<Vec<User>, Errors> {
    let organization_id = authorize(token, Organization(AuthObj::None), conn)?;
    organization::get_all_users(organization_id, conn)
}

/*
  auth: Co-worker
*/
pub fn get_vacations(token: Token, conn: &PgConnection) -> Result<Vec<Vacation>, Errors> {
    let organization_id = authorize(token, Organization(AuthObj::None), conn)?;
    organization::get_all_vacations(organization_id, conn)
}
