use crate::api::queries;
use dotenv::dotenv;
use rocket::config::Config;
use rocket::figment::Figment;
use rocket::figment::{
    util::map,
    value::{Map, Value},
};
use rocket::http::Method;
use rocket::Rocket;
use rocket_cors::{AllowedOrigins, CorsOptions};
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

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    let config = Figment::from(Config::default())
        .merge(("port", port_number))
        .merge(("databases", map!["vision_db" => db]));

    rocket::custom(config)
        .attach(DbConn::fairing())
        .attach(cors)
        .mount("/", queries::get_routes())
}
