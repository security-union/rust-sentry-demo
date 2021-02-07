## Sentry Rust Integration
## Usage

To install the latest self-hosted sentry version, execute the *install-sentry-latest.sh* script. 
It will create a *sentry-client* folder with the downloaded files there and install sentry locally for you.
Create a user when it prompts to so you are able to login locally on the app.

### Sentry run

```bash
make run-sentry
```

### Sentry stop

```bash
make down-sentry
```

To run the Rocket server, execute the following steps:

1. copy the *.env.example* to an *.env* file
2. run the sentry app and go to **localhost:9000**
3. Go to your project (Create one if necessary) > settings > Client Keys
4. Copy the DSN key and set the *SENTRY_DSN* variable on the .env file with it
3. cargo run!

Have fun! :rocket:
