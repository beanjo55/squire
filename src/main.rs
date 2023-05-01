use chrono::{DateTime, FixedOffset, Local, Utc};
use dotenv::dotenv;
use std::{
    env,
    error::Error,
    sync::{Arc, Mutex},
};
use tracing;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::{request::channel::reaction::RequestReactionType, Client as HttpClient};

pub mod database;
pub mod redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;

    // Specify intents requesting events about things like new and updated
    // messages in a guild and direct messages.
    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES | Intents::MESSAGE_CONTENT;

    // Create a single shard.
    let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);

    // The http client is separate from the gateway, so startup a new
    // one, also use Arc such that it can be cloned to other threads.
    let http = Arc::new(HttpClient::new(token));
    let redis = Arc::new(Mutex::new(redis::connect_redis(&env::var("REDIS_URL")?)?));
    let reqwest_client = reqwest::Client::new(); // reqwest uses arc internally, so we don't need to wrap it in an arc

    // Since we only care about messages, make the cache only process messages.
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Startup the event loop to process each event in the event stream as they
    // come in.
    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };
        // Update the cache.
        cache.update(&event);

        // Spawn a new task to handle the event
        tokio::spawn(handle_event(event, Arc::clone(&http)));
    }

    Ok(())
}

const ABBY_HA_ENDPOINT: &str = "snip";
const ABBY_HA_CHANNELS: [u64; 2] = [1097704917092794540, 1084011209096974437];

async fn handle_event(
    event: Event,
    http: Arc<HttpClient>,
    reqwest_client: reqwest::Client,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) => {
            if ABBY_HA_CHANNELS.contains(&msg.channel_id.get()) {
                if msg.attachments.len() == 1 && msg.flags.unwrap().bits() >> 13 == 1 {
                    let att = msg.attachments.get(0).unwrap();
                    println!("url is: {}", att.url)
                }
            }
            match msg.content.as_str() {
                "!ping" => {
                    http.create_message(msg.channel_id)
                        .content("Pong!")?
                        .await?;
                    println!("Ping-ed by {}", msg.author.name)
                }
                "!time" => {
                    let utc_time = DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc);
                    let mut offset = FixedOffset::west_opt(0);
                    match msg.author.id.get() {
                        // alex
                        108429628560924672 => offset = FixedOffset::west_opt(8 * 3600),
                        // abby
                        253233185800847361 => offset = FixedOffset::west_opt(4 * 3600),
                        _ => {}
                    }
                    let stamp = utc_time
                        .with_timezone(&offset.unwrap())
                        .format("%d/%m/%Y %H:%M");
                    http.create_message(msg.channel_id)
                        .content(format!("It's {stamp} right now!").as_str())?
                        .await?;
                }
                "!react_self" => {
                    let msg = http
                        .create_message(msg.channel_id)
                        .content("Test!")?
                        .await?
                        .model()
                        .await?;

                    let react = RequestReactionType::Unicode { name: "🌃" };
                    http.create_reaction(msg.channel_id, msg.id, &react).await?;
                }
                _ => {}
            }
        }
        Event::Ready(_) => {
            println!("Shard is ready");
        }
        _ => {}
    }

    Ok(())
}
