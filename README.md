## Sentry Rust Integration

### Init

```rust
let _guard = sentry::init(("https://examplePublicKey@o0.ingest.sentry.io/0", sentry::ClientOptions {
    release: sentry::release_name!(),
    ..Default::default()
}));
```

### Verify Setup

Causing a panic will trigger an event to sentry

```rust
panic!("Everythins is on fire!")
```

### Configuration

- #### Basic Options

  Options are passed to the init() function as a tuple where the first argument is the DSN and the second the options.

  The sentry API provides the struct `ClientOptions` to set them.

  Also, options can be set from **environment variables**:

  - SENTRY_DSN: Tells the SDK where to send the events
  - SENTRY_ENVIRONMENT: A release can be associated with more than one environment to separate them in the UI (staging vs prod for example)
  - SENTRY_RELEASE: Sets the release name

- #### More options available:

  - stack_trace_mode
  - debug: If enabled, the SDK will attempt to print out useful debugging information if something goes wrong with sending the event. Not recommended to turn it on in production.
  - sample_rate: Sets the % of errors that are sent, by default is set to 100%
  - max_breadcrumbs
  - attach_stacktrace: When enabled, stack traces are automatically attached to all messages looged
  - send_default_pii: Sends certain personally identifiable information. By default is off. If possible, is recommended to turn it on and manually remove the data that you dont want to be sent using the _Sensitive Data_ features of sentry
  - in_app_include
  - in_app_exclude

- #### Hooks

  These options can be used to hook the SDK to customize the reporting of events.

  - before_send: called before an event is sent with an event object passed by parameters. This can be used, for instance, for manual PII stripping before sending.
  - before_breadcrumb: called before a breadcrumb is added to the scope with a breadcrumb object passed by parameters.

## Releases & Health

A release if a version of your code that is deployed to an environment.
When you give Sentry information about your releases, you can:

- Determine issues and regressions introduced in a new release
- Predict which commit caused an issue and who is likely responsible
- Resolve issues by including the issue number in your commit message
- Receive email notifications when your code gets deployed

```rust
let _guard = sentry::init(("https://examplePublicKey@o0.ingest.sentry.io/0", sentry::ClientOptions {
    release: Some("my-project-name@2.3.12".into()),
    // OR automatically:
    // release: sentry::release_name!(),
    ..Default::default()
}));
```

### Release Health

Monitor the health of releases by observing user adoption, usage of the application, percentage of crashes, and session data. Release health will provide insight into the impact of crashes and bugs as it relates to user experience, and reveal trends with each new issue through the release details, graphs, and filters.

### Filtering Events

In order to filter events or breadcrumbs we can use the before_send function the SDK provides us.

Returning None will dismiss the event.

## API Usage

Sentry's SDK hooks into your runtime environment and automatically reports errors, exceptions, and rejections.

**Key Terms**:

- An _event_ is one instance of sending data to Sentry. Generally, this data is an error or exception.
- An _issue_ is a grouping of similar events
- The reporting of an event is called _capturing_. When an event is captured, it's sent to Sentry.

While capturing an event, you can also record the breadcrumbs that lead up to that event. Breadcrumbs are different from events: they will not create an event in Sentry, but will be buffered until the next event is sent.

### Capturing errors:

- Any std::error::Error type can be captured

```rust
let result = match function_returns_error() {
    Ok(result) => result,
    Err(err) => {
        sentry::capture_error(&err);
        return Err(err);
    }
};
```

Some integrations may provide more specialized capturing methods.

### Capturing messages:

```rust
sentry::capture_message("Something went wrong", sentry::Level::Info);
```

### Logging Level

It can be set within an event:

```rust
sentry::configure_scope(|scope| {
    scope.set_level(Some(Level::Warning));
});
```

## Breadcrumbs 

Sentry uses *breadcrumbs* to create a trail of events that happened prior to an issue.
These events are similar to traditional logs, but can record more rich structrured data.

To add a breadcrumb the SDK provides an *add_breadcrumb* function to add a new one:
This method accepts any object that implements **IntoBreadcrumbs**, the most common implementations that can be passed:

+ Breadcrumb
+ Vec<<BreadCrumb>>
+ Option<<Breadcrumb>>
+ FnOnce() -> impl IntoBreadcrumbs

```rust
add_breadcrumb(Breadcrumb {
    category: Some("auth".into()),
    message: Some(format!("Authenticated user {}", user.email)),
    level: Level::Info,
    ..Default::default()
});
```

## Scopes and Hubs

When an event is captured and sent to Sentry, SDKs will merge that event data with extra information from the current scope.

You can think of the hub as the central point that our SDKs use to route an event to Sentry. When you call **init()** a hub is created and a client and a blank scope are created on it. That hub is then associated with the current thread and will internally hold a stack of scopes.


### Scope

The scope is an object that can be cloned efficiently and stores data that is locally relevant to an event. For instance the scope will hold recorded breadcrumbs and similar information.

The scope can be interacted with in two ways:

1. The scope is routinely updated with information by functions such as add_breadcrumb which will modify the currently top-most scope.
2. The topmost scope can also be configured through the configure_scope method.

The most useful operation when working with scopes is the configure-scope function. It can be used to reconfigure the current scope:

```rust
configure_scope(|scope| {
    scope.set_tag("my-tag", "my value");
    scope.set_user(Some(User {
        id: Some(42.to_string()),
        email: Some("john.doe@exmaple.com".into()),
        ..Default::default()
    }));
});
```

## Integrations

An integration in sentry has two primary purposes. It can act as an _Event Source_, which will caprture new events; or as an _Event Processor_, which can modify every **Event** flowing through the pipeline.

[more about integrations here](https://docs.rs/sentry/0.22.0/sentry/integrations/index.html)


