#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_sentry_logger;

use rocket::http::Status;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_sentry_logger::{self as logger, LogLevel};

#[get("/panic")]
fn panic() -> &'static str {
    panic!("AAAA!!! SOMETHING IS WRONG!!");
}

#[get("/fail")]
fn just_fail() -> Status {
    logger::log("Failing!", LogLevel::Info);
    Status::NotAcceptable
}

fn main() {
    dotenv::dotenv().ok();
    logger::init();
    rocket::ignite()
        .mount("/", routes![panic, just_fail])
        .attach(SpaceHelmet::default())
        .launch();
}
