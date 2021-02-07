use sentry::{Breadcrumb, ClientOptions, IntoDsn, Scope, User, add_breadcrumb, protocol::Event};
use std::sync::Arc;

pub fn sentry_options() -> ClientOptions {
    ClientOptions {
        dsn: "http://82b5a065d18642218a1f26cf3d037943@localhost:9000/1"
            .into_dsn()
            .unwrap(),
        release: sentry::release_name!(),
        debug: true,
        attach_stacktrace: true,
        send_default_pii: true,
        before_send: Some(Arc::new(|mut event: Event| {
            event.tags.insert("Sending".into(), "true".into());
            event.server_name = None;
            Some(event)
        })),
        ..Default::default()
    }
}

pub fn default_scope(scope: &mut Scope) {
    let user = Some(User {
        username: Some("Jaster Rogue".to_string()),
        ..Default::default()
    });
    scope.set_extra("character.name", "Mighty Fighter".to_owned().into());
    scope.set_user(user);

    add_breadcrumb(Breadcrumb {
        category: Some("Startup".into()),
        message: Some("Application ready".into()),
        ..Default::default()
    })
}
