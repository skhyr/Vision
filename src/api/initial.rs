use crate::repos::user as UserRepo;
use crate::types::user::User;
use dotenv::dotenv;
use rocket::config::Config;
use rocket::figment::Figment;
use rocket::figment::{
    util::map,
    value::{Map, Value},
};
use rocket::serde::json::Json;
use rocket::Rocket;
use rocket_sync_db_pools::{database, diesel};

#[database("vision_db")]
pub struct DbConn(diesel::PgConnection);

pub fn init_routes() -> Rocket<rocket::Build> {
    dotenv().ok();

    let port_number: u16 = std::env::var("ROCKET_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let db: Map<_, Value> = map! {
        "url" => std::env::var("DATABASE_URL").unwrap().into()
    };

    let config = Figment::from(Config::default())
        .merge(("port", port_number))
        .merge(("databases", map!["vision_db" => db]));

    rocket::custom(config)
        .attach(DbConn::fairing())
        .mount("/", routes![index, all_users])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
async fn all_users(conn: DbConn) -> Json<Vec<User>> {
    conn.run(|c| Json(UserRepo::get_all(c).unwrap())).await
}
