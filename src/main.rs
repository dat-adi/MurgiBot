mod commands;

use commands::{howl::*};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    utils::MessageBuilder,
    client::{Client, Context, EventHandler},
    framework::standard::{
        StandardFramework,
        macros::{
            group
        }
    }
};

use std::env;
use regex::Regex;
use tracing::{error, info};

struct Handler;

// Defining a howl counter, so, when it hits 4 counts, it howls.
static mut HOWL_COUNTER: i32 = 0;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        let channel = match msg.channel_id.to_channel(&context).await {
            Ok(channel) => channel,
            Err(why) => {
                println!("Error getting channel: {:?}", why);

                return;
            },
        };

        if msg.content == "m>howl" {
            // The message builder allows for creating a message by
            // mentioning users dynamically, pushing "safe" versions of
            // content (such as bolding normalized content), displaying
            // emojis, and more.
            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" summoned the Murgi overlord in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }

        unsafe {
            let howl_checker = Regex::new(r"^MURGI CLAN AW[O]*$").unwrap();
            if howl_checker.is_match(&msg.content) == true {
                HOWL_COUNTER += 1;
                if HOWL_COUNTER == 4 {
                    let response = MessageBuilder::new()
                        .push("MURGI CLAN AWOOOOOOOOOOOOOOOOOO")
                        .build();

                    if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                        println!("Error sending message: {:?}", why);
                    }

                    HOWL_COUNTER = 0;
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(howl)]
struct General;

#[tokio::main]
async fn main() {
    // Loading the environment variables
    dotenv::dotenv().expect("Failed to load the .env file");

    // Initializing the logger to use environment variables
    // This is a bit intense, so, proceed with caution.
    // tracing_subscriber::fmt::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("m>")) // set the bot's prefix to "m>"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
