use std::{env, str::FromStr};

use yawn_server::Server;

use async_nats::ServerAddr;

#[tokio::main]
async fn main() {
    let nats_url = ServerAddr::from_str(env::var("NATS_URL").expect("NATS_URL wasn't supplied").as_str()).expect("NATS_URL isn't a valid server URL");
    let outpost_name = env::var("OUTPOST_NAME").expect("OUTPOST_NAME wasn't supplied");

    let server = Server::setup(outpost_name, nats_url).await.expect("Client couldn't connect to nats URL");
    server.run().await.expect("Server couldn't subscribe to its subject");
}
