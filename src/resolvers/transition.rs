use crate::services::authorization::{authorize, AccessLevels::*, AuthObj};
use crate::services::transition;
use crate::types::transition::NewTransition;
use crate::types::{Token, Transition};
use crate::utils;
use crate::utils::errors::Errors;
use diesel::PgConnection;

/*
  auth: Admin
*/
pub fn add_transition(
    new_transition: NewTransition,
    user_id: String,
    token: Token,
    conn: &PgConnection,
) -> Result<Transition, Errors> {
    let user_uuid = utils::parse_uuid(user_id)?;
    authorize(token, Admin(AuthObj::User(user_uuid)), conn)?;
    transition::add(new_transition, user_uuid, conn)
}
