use crate::repos::{UserRepo, VacationRepo};
use crate::services::authentication;
use crate::types::Token;
use crate::utils::Errors;
use diesel::PgConnection;
use uuid::Uuid;

/*

  Authorize will return the Uuid of the authorized entity:
    Admin -> organization_id
    Organization -> organization_id
    User -> user_id

  An optional AuthObj can be passed to directly make the match test
  and return an error in case of incompatibility.

*/

pub enum AccessLevels {
    Admin(AuthObj),
    Organization(AuthObj),
    User(AuthObj),
}

pub enum AuthObj {
    User(Uuid),
    Vacation(Uuid),
    None,
}

pub fn authorize(token: Token, level: AccessLevels, conn: &PgConnection) -> Result<Uuid, Errors> {
    match level {
        AccessLevels::Admin(obj) => authorize_admin(token, obj, conn),
        AccessLevels::Organization(obj) => authorize_organization(token, obj, conn),
        AccessLevels::User(obj) => authorize_user(token, obj, conn),
        _ => Err(Errors::Unauthorized),
    }
}

fn authorize_admin(token: Token, obj: AuthObj, conn: &PgConnection) -> Result<Uuid, Errors> {
    let token_string = token.admin_token.ok_or(Errors::Unauthorized)?;
    let org_id = authentication::auth_org(token_string, conn).map(|r| r.id)?;
    match obj {
        AuthObj::None => Ok(org_id),
        AuthObj::User(id) => {
            let user = UserRepo::get_by_id(id, conn)?;
            match user.organization_id == org_id {
                true => Ok(org_id),
                false => Err(Errors::Unauthorized),
            }
        }
        AuthObj::Vacation(id) => {
            let vacation = VacationRepo::get_by_id(id, conn)?;
            let user = UserRepo::get_by_id(vacation.user_id, conn)?;
            match user.organization_id == org_id {
                true => Ok(org_id),
                false => Err(Errors::Unauthorized),
            }
        }
    }
}

fn authorize_organization(token: Token, obj: AuthObj, conn: &PgConnection) -> Result<Uuid, Errors> {
    // let org_id = authentication::auth_user(&token, conn)
    //     .map(|r| r.organization_id)
    //     .or_else(|_| authentication::auth_org(&token, conn).map(|r| r.id))?;

    let org_id = match (token.admin_token, token.user_token) {
        (None, None) => Err(Errors::Unauthorized),
        (_, Some(t)) => authentication::auth_user(t, conn).map(|r| r.organization_id),
        (Some(t), _) => authentication::auth_org(t, conn).map(|r| r.id),
    }?;

    match obj {
        AuthObj::None => Ok(org_id),
        AuthObj::User(id) => {
            let user = UserRepo::get_by_id(id, conn)?;
            match user.organization_id == org_id {
                true => Ok(org_id),
                false => Err(Errors::Unauthorized),
            }
        }
        AuthObj::Vacation(id) => {
            let vacation = VacationRepo::get_by_id(id, conn)?;
            let user = UserRepo::get_by_id(vacation.user_id, conn)?;
            match user.organization_id == org_id {
                true => Ok(org_id),
                false => Err(Errors::Unauthorized),
            }
        }
    }
}

fn authorize_user(token: Token, obj: AuthObj, conn: &PgConnection) -> Result<Uuid, Errors> {
    let token_string = token.user_token.ok_or(Errors::Unauthorized)?;
    let user_id = authentication::auth_user(token_string, conn).map(|r| r.id)?;
    match obj {
        AuthObj::None => Ok(user_id),
        AuthObj::User(id) => match user_id == id {
            true => Ok(user_id),
            false => Err(Errors::Unauthorized),
        },
        AuthObj::Vacation(id) => {
            let vacation = VacationRepo::get_by_id(id, conn)?;
            match vacation.user_id == user_id {
                true => Ok(user_id),
                false => Err(Errors::Unauthorized),
            }
        }
    }
}
