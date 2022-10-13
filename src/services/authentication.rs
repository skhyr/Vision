use crate::repos::{OrganizationRepo, UserRepo};
use crate::types::{Organization, User};
use crate::utils::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub struct TokenContent {
    id: Uuid,
    access_code: i32,
}

pub fn extract_content_from_token_string(token_string: String) -> Result<TokenContent, Errors> {
    let parts = token_string.split("@").collect::<Vec<&str>>();
    match parts.len() {
        2 => Ok(TokenContent {
            id: Uuid::parse_str(parts[0]).map_err(|_| Errors::Unauthorized)?,
            access_code: parts[1].parse().map_err(|_| Errors::Unauthorized)?,
        }),
        _ => Err(Errors::Unauthorized),
    }
}

pub fn auth_user(token: String, conn: &PgConnection) -> Result<User, Errors> {
    // let token_string = token.user_token.ok_or(Errors::Unauthorized)?;
    let content = extract_content_from_token_string(token)?;
    let user = UserRepo::get_by_id(content.id, conn)?;
    match user.access_code == content.access_code {
        true => Ok(user),
        false => Err(Errors::Unauthorized),
    }
}

pub fn auth_org(token: String, conn: &PgConnection) -> Result<Organization, Errors> {
    let content = extract_content_from_token_string(token)?;
    let org = OrganizationRepo::get_by_id(content.id, conn)?;
    match org.access_code == content.access_code {
        true => Ok(org),
        false => Err(Errors::Unauthorized),
    }
}
