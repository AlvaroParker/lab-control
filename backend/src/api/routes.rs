use super::{admins, personas, registros};
use crate::database::pool::Pool;
use axum::{http::Method, routing, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

const QUOTES: &[&str] = &[
    "Release 1.0 when?",
    "\"I commit too often, people can't catch up\" - PPP",
    "This text is random.",
    "\"There are reasons to not use rust.\" - No one, ever",
    "Read the wiki.",
    "Compile, wait for 20 minutes, notice a new commit, compile again.",
    "\"please make this message a splash\"",
    "Segmentation fault (core dumped) - This is literally not an error, is a healthcheck",
    "some basic startup code",
    "\"I think I am addicted\" - bob",
    "\"And then I saw her face, Now I'm a believer\"",
    "I see a red door and I want it painted black.",
    "Take on me, take me on...",
    "You spin me right round baby right round",
    "Stayin' alive, stayin' alive",
    "Say no way, say no way ya, no way!",
    "Is this the real life, is this just fantasy",
    "I'm still standing, better than I ever did",
    "Two trailer park girls go round the outside",
    "Black bird, black moon, black sky",
    "Your brain gets smart, but your head gets dumb.",
    "Ding ding pch n daa, bam-ba-ba-re-bam baram bom bom baba-bam-bam-bommm",
    "Súbeme la radio que esta es mi canción",
    "I'm beggin', beggin' you",
    "Never gonna let you down (I am trying!)",
    "\"I use Arch, btw\" - John Cena",
    "\"stop playing league loser\" - bot",
    "\"If it ain't broke, don't fix it\" - yiyi",
];

use std::time::{SystemTime, UNIX_EPOCH};

pub async fn healt_check() -> String {
    // Get the current time in milliseconds since UNIX_EPOCH as a seed
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as usize;
    let index = seed % QUOTES.len();
    // Use the seed to generate a random index
    QUOTES[index].into()
}

// Create the routes of our webserver, state is the DB Pool, where
// we can get a connection to the DB
pub async fn create_routes(state: Arc<Pool>) -> Router {
    // We create a CORS layer, this allows us to make requests from
    // different origins
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::DELETE,
            Method::OPTIONS,
            Method::PUT,
        ]);
    // Create the router with layer CORS and state DB Pool
    Router::new()
        .nest(
            "/api/usuarios",
            personas::routes::create_routes(state.clone()).await,
        )
        .nest(
            "/api/admin",
            admins::routes::_create_routes(state.clone()).await,
        )
        .nest(
            "/api/registros",
            registros::routes::create_routes(state.clone()).await,
        )
        .nest(
            "/api/metadata",
            crate::api::metadata::create_routes(state.clone()).await,
        )
        .with_state(state)
        .route("/api/health", routing::get(healt_check))
        .layer(cors)
}
