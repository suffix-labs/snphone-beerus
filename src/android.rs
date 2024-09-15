use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use jni::{
    objects::{JClass, JIntArray, JString},
    JNIEnv,
};

use super::config::Config;
use clap::Parser;
use helios::config::networks::Network;
use tokio::sync::RwLock;

const RPC_SPEC_VERSION: &str = "0.6.0";

// using to test that this can execute from android.
// should be removed once that is working.
fn run() {
    println!("Running!");
}

/*
#[tokio::main]
async fn run(
    eth_execution_rpc: String,
    starknet_rpc: String,
    data_dir: String,
) -> eyre::Result<()> {
    /*
    tracing_subscriber::fmt::init();

    let config = Config {
        network: Network::SEPOLIA, // TODO: take network as input
        eth_execution_rpc,
        starknet_rpc,
        data_dir: PathBuf::from(data_dir),
        poll_secs,
        rpc_addr: SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            socket_port,
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
    */


    Ok(())
}
*/

#[no_mangle]
pub extern "C" fn Java_com_example_beerus_1android_Beerus_run<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    eth_execution_rpc: JString<'local>,
    starknet_rpc: JString<'local>,
    data_dir: JString<'local>,
) {
    let eth_execution_rpc: String = env
        .get_string(&eth_execution_rpc)
        .expect("Unable to get eth_execution_rpc string")
        .into();

    let starknet_rpc: String = env
        .get_string(&starknet_rpc)
        .expect("Unable to get starknet_rpc string")
        .into();

    let data_dir: String = env
        .get_string(&data_dir)
        .expect("Unable to get data_dir string")
        .into();

    //run(eth_execution_rpc, starknet_rpc, data_dir, 5, 8080);
    run();
}
