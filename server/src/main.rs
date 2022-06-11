use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tracing_error::ErrorLayer;

/// App Configuration
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(default_value = "50051", long, short, env)]
    port: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get args
    let args = Args::parse();
    // Pattern match configuration
    let Args {
        port,
    } = args;

    // Setup logging
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = tracing_subscriber::fmt::layer();
    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer) // stdout/stderr logger
        .with(ErrorLayer::default());
    registry.try_init()?;

    xkcd_explorer_server::server(port).await?;

    Ok(())
}
