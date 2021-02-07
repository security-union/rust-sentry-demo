#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate sentry;

mod config;

use config::{default_scope, sentry_options};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket_contrib::helmet::SpaceHelmet;
use sentry::{add_breadcrumb, protocol::Event, types::Uuid, Breadcrumb, Level};
use std::env::VarError;

#[get("/panic")]
fn panic() -> &'static str {
    panic!("AAAA!!! SOMETHING IS WRONG!!");
}

#[get("/error")]
fn capture_err() -> &'static str {
    sentry::capture_error(&VarError::NotPresent);
    "Hello, Error!"
}

#[get("/message")]
fn capture_message() -> &'static str {
    sentry::capture_message("Hey, this is a message for sentry!", Level::Info);
    "Hello, Message!"
}

#[get("/scoped_message")]
fn capture_scoped_message() -> &'static str {
    sentry::with_scope(
        |scope| {
            scope.set_tag("character", "rick");
            scope.set_tag("planet", "earth");
            scope.set_tag("label", "fight");
            scope.set_level(Some(Level::Warning));
        },
        || {
            sentry::capture_message("Warning! Rick is coming ¬¬", Level::Info);
        },
    );
    "Hello, scopes!"
}

#[post("/breadcrumbs")]
fn set_breadcrumbs() -> &'static str {
    add_breadcrumb(Breadcrumb {
        category: Some("Aliens things".into()),
        message: Some("Com que una frase random?".into()),
        ty: "info".into(),
        level: Level::Info,
        ..Default::default()
    });

    add_breadcrumb(|| {
        //I could do some cool stuff here
        Breadcrumb {
            category: Some("Openning a room".into()),
            message: Some("Wow whats that".into()),
            ty: "warning".into(),
            level: Level::Warning,
            ..Default::default()
        }
    });

    sentry::capture_message("mmh here we have some breadcrumbs", Level::Info);
    "Hey! Done!"
}

#[get("/event")]
fn capture_event() -> &'static str {
    let uuid = Uuid::new_v4();
    let event = Event {
        event_id: uuid,
        message: Some("This is a raw event, how are you?".to_string()),
        ..Default::default()
    };
    sentry::capture_event(event);
    "Hello, Event!"
}

#[get("/fail")]
fn just_fail() -> Status {
    Status::NotAcceptable
}

fn main() {
    let _guard = sentry::init(sentry_options());
    sentry::configure_scope(default_scope);

    rocket::ignite()
        .mount(
            "/",
            routes![
                panic,
                capture_err,
                capture_message,
                capture_event,
                capture_scoped_message,
                set_breadcrumbs,
                just_fail
            ],
        )
        .attach(SpaceHelmet::default())
        .attach(AdHoc::on_response("log responses", |_req, resp| {
            if resp.status().code >= 400 {
                sentry::capture_message(resp.status().reason, Level::Error);
            }
        }))
        .launch();
}
