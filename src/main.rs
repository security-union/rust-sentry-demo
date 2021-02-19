#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate sentry;

mod logger;

use logger::{LogLevel, Step, StepType};
use rocket::http::Status;
use rocket_contrib::helmet::SpaceHelmet;

#[get("/panic")]
fn panic() -> &'static str {
    panic!("AAAA!!! SOMETHING IS WRONG!!");
}

#[get("/steps")]
fn add_steps() -> &'static str {
    let step = Step {
        ty: StepType::Info,
        title: "Bad request".into(),
        message: "Mike made a bad request".into(),
        level: LogLevel::Info,
        body: None,
    };
    logger::track_step(step);
    logger::log("Failing!", LogLevel::Fatal);
    "Hi, Steps!"
}

#[get("/fail")]
fn just_fail() -> Status {
    logger::log("Failing!", LogLevel::Info);
    Status::NotAcceptable
}

fn main() {
    dotenv::dotenv().ok();
    let _guard = logger::init();
    rocket::ignite()
        .mount("/", routes![panic, just_fail, add_steps])
        .attach(SpaceHelmet::default())
        .attach(logger::fairing())
        .launch();
}
