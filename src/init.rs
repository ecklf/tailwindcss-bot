use anyhow::Error;
use serenity::{
    http::Http,
    model::interactions::application_command::{ApplicationCommand, ApplicationCommandOptionType},
};

pub async fn create_global_commands(http: &Http) -> Result<(), Error> {
    // Listing global commands
    // let commands = ApplicationCommand::get_global_application_commands(http).await?;
    // dbg!(commands);

    // Deleting global commands
    // ApplicationCommand::delete_global_application_command(&http, CommandId(858059701580070942))
    //     .await?;

    ApplicationCommand::create_global_application_command(&http, |command| {
        command.name("ping").description("A simple ping command")
    })
    .await?;

    ApplicationCommand::create_global_application_command(&http, |command| {
        command
            .name("docs")
            .description("Search the Tailwind CSS documentation")
            .create_option(|option| {
                option
                    .name("term")
                    .description("The term to search for")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user to ping")
                    .kind(ApplicationCommandOptionType::User)
            })
    })
    .await?;

    Ok(())
}
