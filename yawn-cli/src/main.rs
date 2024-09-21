use clap::Parser;
use yawn_cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error>{
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Wake(wake)) =>  wake.wake_outpost().await?,
        None => {},
    }

    Ok(())
}
