use std::str::FromStr;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Send messages to a NATS server that yawn reacts to", long_about = None, name = "yawn")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Wake device with MAC address on outpost
    #[command(arg_required_else_help = true)]
    Wake(Wake),
}

#[derive(Parser)]
pub struct Wake {
    /// NATS server URL endpoint
    #[arg(short, long)]
    nats_url: String,
    /// Name of yawn outpost server
    #[arg(short, long)]
    outpost_name: String,
    /// MAC address to wake
    #[arg(short, long)]
    mac_address: String
}

impl Wake {
    pub async fn wake_outpost(&self) -> Result<(), async_nats::Error>{
        let address = async_nats::ServerAddr::from_str(self.nats_url.as_str()).expect("nats_url is not a valid nats url");

        let client = async_nats::connect(address.clone()).await?;
        // TODO: Serialize the message spec here
        let payload: bytes::Bytes = self.mac_address.clone().into();

        client.publish(format!("yawn.outposts.{}", self.outpost_name), payload.clone()).await?;
        println!("published {:?} in yawn.outposts.{} to {}:{}", payload, self.outpost_name, address.host(), address.port());

        client.flush().await?;
        Ok(())
    }
}
