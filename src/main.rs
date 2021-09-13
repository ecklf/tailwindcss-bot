use anyhow::Error;
use dotenv::dotenv;
use serenity::{
    async_trait,
    builder::CreateSelectMenuOption,
    framework::standard::StandardFramework,
    http::Http,
    model::{
        interactions::application_command::ApplicationCommandInteractionDataOptionValue, prelude::*,
    },
    prelude::*,
};
use std::env;
use tailwind_bot::{algolia::search_tailwind_docs, init::set_global_commands};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "docs" => {
                    let search_term = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected term option")
                        .resolved
                        .as_ref()
                        .expect("Expected term string");

                    if let ApplicationCommandInteractionDataOptionValue::String(term) = search_term
                    {
                        let doc_entries = match search_tailwind_docs(term).await {
                            Ok(data) => data,
                            _ => vec![],
                        };

                        if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                            response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| {
                                message.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                                match doc_entries.len() {
                                    0 => {
                                        message.content(format!("No results found for `{}`!", &term))
                                    },
                                    _ => {
                                        message.content(format!("Displaying resuls for `{}`.\n\nPlease select a result to display:", &term)).components(|c| {
                                            c.create_action_row(|ar| {
                                                ar.create_select_menu(|sm| {
                                                    sm.custom_id("select");
                                                    sm.placeholder("Please select an entry");
                                                    sm.min_values(1);
                                                    sm.max_values(1);
                                                    sm.options(|o| {
                                                        for (_, entry) in doc_entries.iter().enumerate() {
                                                            let mut option = CreateSelectMenuOption::default();
                                                            option.label(&entry.label);
                                                            option.value(&entry.url);
                                                            if let Some(d) = &entry.description {
                                                                option.description(format!("└ {}", d));
                                                            }
                                                            o.add_option(option.to_owned());
                                                        }
                                                        o
                                                    })
                                                })
                                            })
                                        })
                                    }
                                }
                            })
                        }).await {
                            println!("Cannot respond to slash command: {}", why);
                        };
                        let message = command.get_interaction_response(&ctx.http).await.unwrap();
                        let collector = message
                            .await_component_interaction(&ctx)
                            .author_id(command.user.id)
                            .await;

                        if let Some(component) = collector {
                            let user_object = match command.data.options.get(1) {
                                Some(option) => match option.to_owned().resolved {
                                    Some(ApplicationCommandInteractionDataOptionValue::User(
                                        user,
                                        _member,
                                    )) => Some(user),
                                    _ => None,
                                },
                                _ => None,
                            };

                            let content = match user_object {
                                // Wrapping in "<>" hides URL preview
                                Some(user) => {
                                    format!(
                                        "{}: <{}>",
                                        user.id.mention(),
                                        &component.data.values[0]
                                    )
                                }
                                None => format!("<{}>", &component.data.values[0]),
                            };

                            command
                                .create_followup_message(&ctx.http, |response| {
                                    response.content(content)
                                })
                                .await
                                .unwrap();

                            command
                                .edit_original_interaction_response(&ctx.http, |f| {
                                    f.content("Thanks! You can now dismiss this message");
                                    f.components(|c| c.set_action_rows(vec![]))
                                })
                                .await
                                .unwrap();
                        }
                    }
                }
                _ => println!("Not implemented"),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let commands = set_global_commands(&ctx.http).await;
        println!("Available global slash commands: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's id.
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
