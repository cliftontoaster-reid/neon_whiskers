use mongodb;
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio;
use wit_ai;
mod types {
    pub mod db;
}

struct Bot {
    mongo_client: mongodb::Client,
    db: mongodb::Database,
    nlp: wit_ai::api::Api,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _ctx: Context, _msg: Message) {

    }
}

#[tokio::main]
async fn main() {
    let discord_token = std::env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment \"DISCORD_TOKEN\"");
    let wit_token =
        std::env::var("WITAI_TOKEN").expect("Expected a token in the environment \"WITAI_TOKEN\"");
    let database_name = std::env::var("DB_NAME")
        .expect("Expected a name in the environment \"DB_NAME\"");

    let client_options = mongodb::options::ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    let client = mongodb::Client::with_options(client_options).unwrap();

    let bot = Bot {
        mongo_client: client.clone(),
        db: client.database(&database_name).clone(),
        nlp: wit_ai::client(wit_token),
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&discord_token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");
    client.start().await.unwrap();
}
