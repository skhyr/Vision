#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

pub mod api;
pub mod logic;
pub mod repos;
pub mod resolvers;
pub mod schema;
pub mod services;
pub mod types;
pub mod utils;

#[cfg(test)]
pub mod tests;
