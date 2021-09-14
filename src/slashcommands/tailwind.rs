use crate::algolia::search_tailwind_docs;
use std::time::Duration;

use serenity::{
    builder::CreateSelectMenuOption,
    client::Context,
    model::interactions::{
        application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
        },
        InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
    },
    prelude::Mentionable,
};

pub async fn links(ctx: &Context, command: &ApplicationCommandInteraction) {
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected link option")
        .resolved
        .as_ref()
        .expect("Expected string");

    if let ApplicationCommandInteractionDataOptionValue::String(selection_value) = options {
        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(selection_value))
            })
            .await
        {
            println!("Cannot respond to slash command: {}", why);
        };
    }
}

pub async fn docs(ctx: &Context, command: &ApplicationCommandInteraction) {
    let query = command
        .data
        .options
        .get(0)
        .expect("Expected query option")
        .resolved
        .as_ref()
        .expect("Expected string");

    if let ApplicationCommandInteractionDataOptionValue::String(q) = query {
        let doc_entries = match search_tailwind_docs(q).await {
            Ok(data) => data,
            _ => vec![],
        };

        if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                            response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| {
                                message.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                                match doc_entries.len() {
                                    0 => {
                                        message.content(format!("No results found for `{}`!", &q))
                                    },
                                    _ => {
                                        message.content(format!("Displaying resuls for `{}`.\n\nPlease select a result to display:", &q)).components(|c| {
                                            c.create_action_row(|ar| {
                                                ar.create_select_menu(|sm| {
                                                    sm.custom_id("select_doc_entry");
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
            .timeout(Duration::from_secs(60))
            .author_id(command.user.id)
            .await;

        if let Some(component) = collector {
            let user_object = match command.data.options.get(1) {
                Some(option) => match option.to_owned().resolved {
                    Some(ApplicationCommandInteractionDataOptionValue::User(user, _member)) => {
                        Some(user)
                    }
                    _ => None,
                },
                _ => None,
            };

            let content = match user_object {
                // Wrapping in "<>" hides URL preview
                Some(user) => {
                    format!("{}: <{}>", user.mention(), &component.data.values[0])
                }
                None => format!("<{}>", &component.data.values[0]),
            };

            command
                .create_followup_message(&ctx.http, |response| response.content(content))
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
