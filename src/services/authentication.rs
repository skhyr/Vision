use crate::repos::{OrganizationRepo, UserRepo};
use crate::types::{Organization, User};
use crate::utils::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub enum Levels {
    User,
    Organization,
    Sysadmin,
}

pub struct TokenContent {
    id: Uuid,
    access_code: i32,
}

pub fn extract_content_from_token(token: String) -> Result<TokenContent, Errors> {
    let parts = token.split("@").collect::<Vec<&str>>();
    match parts.len() {
        2 => Ok(TokenContent {
            id: Uuid::parse_str(parts[0]).map_err(|_| Errors::Unauthorized)?,
            access_code: parts[1].parse().map_err(|_| Errors::Unauthorized)?,
        }),
        _ => Err(Errors::Unauthorized),
    }
}

pub fn auth_user(token: String, conn: &PgConnection) -> Result<User, Errors> {
    let content = extract_content_from_token(token)?;
    let user = UserRepo::get_by_id(content.id, conn)?;
    match user.access_code == content.access_code {
        true => Ok(user),
        false => Err(Errors::Unauthorized),
    }
}

pub fn auth_org(token: String, conn: &PgConnection) -> Result<Organization, Errors> {
    let content = extract_content_from_token(token)?;
    let org = OrganizationRepo::get_by_id(content.id, conn)?;
    match org.access_code == content.access_code {
        true => Ok(org),
        false => Err(Errors::Unauthorized),
    }
}

