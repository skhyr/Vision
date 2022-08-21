use dotenv::dotenv;
use rocket::config::Config;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method};
use rocket::{Request, Response, Rocket};
use rocket_contrib::database;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::collections::HashMap;

use rocket::figment::Figment;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn init_routes() -> Rocket<rocket::Build> {
    dotenv().ok();
    // let mut database_config = HashMap::new();
    // let mut databases = HashMap::new();

    // database_config.insert("url", std::env::var("DATABASE_URL").unwrap());
    // databases.insert("alcalc-db", Value::from(database_config));
    let port_number: u16 = std::env::var("ROCKET_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    /* let mut config = Config::build(Environment::Staging)
    .port(port_number)
    .extra("databases", databases)
    .finalize()
    .unwrap(); */
    /* let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![
            Method::Get,
            Method::Post,
            Method::Patch,
            Method::Delete,
            Method::Put,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
    )
    .allow_credentials(true); */
    let config = Figment::from(Config::default()).merge(("port", port_number));

    // rocket::custom(config);
    rocket::custom(config).mount("/", routes![index])
    // .attach(cors.to_cors().unwrap())
    // .attach(diesel::PgConnection::fairing())
    // .mount("/user", mutations::user::get_routes())
}
