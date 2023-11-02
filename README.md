# Discord Interaction
A light-weight, easy-to-use library for building Discord bots under the Discord interaction modal. Integrates with AWS Lambda.

Much of a Discord bot's behavior can be described using an request/response model, which a slash command, button press, or other user interaction is sent to a backend, and the backend returns a single response. This request/response is called an *interaction* by Discord, which is specified in detail in the Discord [developer docs](https://discord.com/developers/docs/). This library wraps these request and response types, handling authentication, exposing application-friendly types, and integrating these types with AWS Lambda. To create a lambda endpoint program, you need a bot public key, an implementation of the `InteractionHandler` trait, and an invocation of the `run` function using that implementation. As a minimal example, taken from `demos/hello_world`:

```
use discord_interaction::{run, ApplicationCommand, InteractionHandler, Message, Response};
use lambda_http::Error;

const APPLICATION_PUBLIC_KEY: &str = env!("DEMO_PUBLIC_KEY");

#[tokio::main]
async fn main() -> Result<(), Error> {
    run::<DemoHandler>(APPLICATION_PUBLIC_KEY).await
}

struct DemoHandler;

impl InteractionHandler for DemoHandler {
    fn handle_application_command(ac: ApplicationCommand) -> Response {
        let text_content = format!("Hello <@{}>!", ac.user_id);
        Response::Message(
            Message::new().text(&text_content))
    }
}
```

This program can then be deployed to an AWS lambda using the [cargo lambda build](https://www.cargo-lambda.info/commands/build.html) and [cargo lambda deploy](https://www.cargo-lambda.info/commands/deploy.html) commands. Once the lambda is created, you can create a [Discord application](https://discord.com/developers/applications) and register your app's interactions endpoint URL to a gateway attached to your lambda. Additionally, you should register the [application commands](https://discord.com/developers/docs/interactions/application-commands) that your bot will use. For examples of interaction endpoint programs, as well as the scripts involved in their deployment and application command setup, see the source code and deployment scripts in the `demos/` folder. 

It's worth noting that lambda (a.k.a. serverless) deployments do limit what the bot can do. Since we are limited to each interaction only consisting of a request/response pair, our app can't do things that require a longer lifetime, such as stream music. But the tradeoff is that lambdas are easy to deploy, incredibly inexpensive, and scale very well in terms of how well they can handle concurrency. 

