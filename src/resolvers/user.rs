use crate::repos::UserRepo;
use crate::services::authorization::{authorize, AccessLevels::*, AuthObj};
use crate::services::user;
use crate::types::{Info, Initials, Token, User};
use crate::utils;
use crate::utils::errors::Errors;
use diesel::PgConnection;

/*
  auth: Co-worker
*/
pub fn get_user(token: Token, user_id: String, conn: &PgConnection) -> Result<User, Errors> {
  let user_uuid = utils::parse_uuid(user_id)?;
  authorize(token, Organization(AuthObj::User(user_uuid)), conn)?;
  UserRepo::get_by_id(user_uuid, &conn)
}

/*
  auth: Myself
*/
pub fn get_me(token: Token, conn: &PgConnection) -> Result<User, Errors> {
  let my_id = authorize(token, User(AuthObj::None), conn)?;
  UserRepo::get_by_id(my_id, conn)
}

/*
  auth: Co-worker
*/
pub fn get_info(token: Token, user_id: String, conn: &PgConnection) -> Result<Info, Errors> {
  let user_uuid = utils::parse_uuid(user_id)?;
  authorize(token, Organization(AuthObj::User(user_uuid)), conn)?;
  let Initials(vacations, transitions, config) = user::get_initials(user_uuid, conn)?;
  user::get_info(vacations, transitions, &config)
}

/*
  auth: Myself
*/
pub fn get_my_info(token: Token, conn: &PgConnection) -> Result<Info, Errors> {
  let my_id = authorize(token, User(AuthObj::None), conn)?;
  let Initials(vacations, transitions, config) = user::get_initials(my_id, conn)?;
  user::get_info(vacations, transitions, &config)
}
