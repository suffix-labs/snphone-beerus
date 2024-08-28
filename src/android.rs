use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use super::config::Config;
use clap::Parser;
use helios::config::networks::Network;
use tokio::sync::RwLock;

const RPC_SPEC_VERSION: &str = "0.6.0";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config {
        network: Network::SEPOLIA,
        eth_execution_rpc: String::from(""),
        starknet_rpc: String::from(""),
        data_dir: PathBuf::from("tmp"),
        poll_secs: 5,
        rpc_addr: SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            8080,
        ),
    };
    config.check().await?;

    let beerus = super::client::Client::new(&config).await?;
    beerus.start().await?;

    let rpc_spec_version = beerus.spec_version().await?;
    if rpc_spec_version != RPC_SPEC_VERSION {
        eyre::bail!("RPC spec version mismatch: expected {RPC_SPEC_VERSION} but got {rpc_spec_version}");
    }

    let state = beerus.get_state().await?;
    tracing::info!(?state, "initialized");

    let state = Arc::new(RwLock::new(state));

    {
        let state = state.clone();
        let period = Duration::from_secs(config.poll_secs);
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(period);
            loop {
                tick.tick().await;
                match beerus.get_state().await {
                    Ok(update) => {
                        *state.write().await = update;
                        tracing::info!(?state, "updated");
                    }
                    Err(e) => {
                        tracing::error!(error=?e, "state update failed");
                    }
                }
            }
        });
    }

    let server =
        super::rpc::serve(&config.starknet_rpc, &config.rpc_addr, state)
            .await?;

    tracing::info!(port = server.port(), "rpc server started");
    server.done().await;

    Ok(())
}
