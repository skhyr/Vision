use crate::types::User;
use rocket::serde::{Serialize, Serializer};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiUser {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub accounting_day: i32,
}

impl ApiUser {
    pub fn from_user(user: User) -> ApiUser {
        ApiUser {
            id: user.id,
            name: user.name,
            surname: user.surname,
            accounting_day: user.accounting_day,
        }
    }
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let clone = self.clone();
        let api_user = ApiUser::from_user(clone);
        Serialize::serialize(&api_user, serializer)
    }
}
