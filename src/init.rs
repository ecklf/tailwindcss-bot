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
                command
                    .name("docs")
                    .description("Search the Tailwind CSS documentation")
                    .create_option(|option| {
                        option
                            .name("q")
                            .description("The search query")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
                    .create_option(|option| {
                        option
                            .name("user")
                            .description("User to mention")
                            .kind(ApplicationCommandOptionType::User)
                            .required(false)
                    })
            })
            .create_application_command(|command| {
                command
                    .name("links")
                    .description("Post common links")
                    .create_option(|option| {
                        option
                            .name("choice")
                            .description("The link choice")
                            .kind(ApplicationCommandOptionType::String)
                            .add_string_choice("Tailwind Play", "<https://play.tailwindcss.com>")
                            .add_string_choice("Awesome Tailwind CSS", "<https://github.com/aniftyco/awesome-tailwindcss>")
                            .add_string_choice("Best Practises", "<https://gist.github.com/sandren/0f22e116f01611beab2b1195ab731b63>")
                            .add_string_choice("Robin's Good Example", "<https://gist.github.com/RobinMalfait/490a0560a7cfde985d435ad93f8094c5>")
                            .add_string_choice("Configuration Stubs", "<https://github.com/tailwindlabs/tailwindcss/tree/master/stubs>")
                            .required(true)
                    })
            })
    })
    .await?;

    Ok(commands)
}
