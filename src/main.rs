#[macro_use]
extern crate rocket;

mod fairings;
mod lib;
mod routes;
mod catcher;

use fairings::x_headers;
use once_cell::sync::Lazy;
use reqwest::Client;
use rocket::{
    shield::{Referrer, Shield, XssFilter},
    Config,
};

pub static HTTP: Lazy<Client> = Lazy::new(Client::new);

#[launch]
fn rocket() -> _ {
    color_eyre::install().unwrap();

    let armor = Shield::default()
        .enable(XssFilter::Disable)
        .enable(Referrer::NoReferrer);

    let config = Config {
        port: 8080,
        keep_alive: 60 * 60,
        ..Config::default()
    };

    rocket::build()
        .configure(config)
        .attach(armor)
        .attach(x_headers::XHeaders::default())
        .register("/", catchers![
          catcher::not_found,
          catcher::internal_error,
          catcher::unauthorized,
          catcher::forbidden,
          catcher::broken_request
      ])
}
