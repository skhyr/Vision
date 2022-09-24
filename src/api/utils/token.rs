use crate::types::Token;
use crate::utils::Errors;
use rocket::request::{FromRequest, Outcome, Request};

use rocket::http::Status;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = Errors;
    // fn from_request(request: &'r Request<'_>) -> Outcome<Token, ()> {
    // let h1: Vec<_> = request.headers().get("Authorization").collect();
    // let h2: Vec<_> = request.headers().get("Authorization").collect();

    // if h1.len() != 1 && h2.len() != 1 {
    //     return Outcome::Forward(());
    // }
    // let token = keys[0].to_string();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let h1 = req.headers().get_one("Authorization-Admin");
        let h2 = req.headers().get_one("Authorization-User");

        match (h1, h2) {
            (None, None) => Outcome::Failure((Status::BadRequest, Errors::Unauthorized)),
            (t1, t2) => Outcome::Success(Token {
                admin_token: t1.map(|s| s.to_string()),
                user_token: t2.map(|s| s.to_string()),
            }),
        }
    }

    // match JwtTokenService::validate::<SessionToken>(token) {
    //     Ok(payload) => Outcome::Success(SessionToken {
    //         session_id: payload.session_id,
    //     }),
    //     Err(_) => Outcome::Failure((rocket::http::Status::new(401, "Unauthorized"), ())),
    // }
    // }
}
