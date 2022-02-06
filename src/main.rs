use anyhow::Error;
use dotenv::dotenv;
use serenity::{
    async_trait, framework::standard::StandardFramework, http::Http, model::prelude::*, prelude::*,
};
use std::env;
use tailwind_bot::{
    init::set_global_commands,
    slashcommands::tailwind::{docs, links},
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::watching("tailwindcss.com"))
            .await;
        let commands = set_global_commands(&ctx.http).await;
        println!("Available global slash commands: {:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "docs" => {
                    docs(&ctx, &command).await;
                }
                "links" => links(&ctx, &command).await,
                _ => println!("This slash command is not implemented yet"),
            };
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    let http = Http::new_with_token(&token);

    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new().configure(|c| {
        c.with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("~")
            .delimiters(vec![", ", ","])
    });

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(application_id)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    Ok(())
}
