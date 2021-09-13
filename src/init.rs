use anyhow::Error;
use serenity::{
    http::Http,
    model::interactions::application_command::{ApplicationCommand, ApplicationCommandOptionType},
};

pub async fn set_global_commands(http: &Http) -> Result<Vec<ApplicationCommand>, Error> {
    // Listing global commands
    // let commands = ApplicationCommand::get_global_application_commands(http).await?;
    // dbg!(commands);

    // Deleting global commands
    // ApplicationCommand::delete_global_application_command(&http, CommandId(858059701580070942))
    //     .await?;

    let commands = ApplicationCommand::set_global_application_commands(&http, |commands| {
        commands
            .create_application_command(|command| {
                command.name("ping").description("A ping command")
            })
            .create_application_command(|command| {
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
                            .required(false)
                    })
            })
    })
    .await?;

    Ok(commands)
}
