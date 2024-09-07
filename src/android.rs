use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use super::config::Config;
use clap::Parser;
use helios::config::networks::Network;
use jni::objects::JIntArray;
use tokio::sync::RwLock;

const RPC_SPEC_VERSION: &str = "0.6.0";

#[tokio::main]
async fn run(
    eth_execution_rpc: String,
    starknet_rpc: String,
    data_dir: String,
    poll_secs: u64,
    socket_port: u64,
) -> eyre::Result<()> {
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

    Ok(())
}

/*

/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class com_example_beerus_android_Beerus */

#ifndef _Included_com_example_beerus_android_Beerus
#define _Included_com_example_beerus_android_Beerus
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     com_example_beerus_android_Beerus
 * Method:    run
 * Signature: (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;II)Ljava/lang/Void;
 */
JNIEXPORT jobject JNICALL Java_com_example_beerus_1android_Beerus_run
  (JNIEnv *, jclass, jstring, jstring, jstring, jint, jint);

#ifdef __cplusplus
}
#endif
#endif

*/

#[no_mangle]
pub extern "C" fn Java_com_example_beerus_1android_Beerus_run(
    mut env: JNIEnv,
    _class: JClass,
    eth_execution_rpc: JString,
    starknet_rpc: JString,
    data_dir: JString,
    poll_secs: JIntArray, // NOTE : is this the right type?
    socket_port: JIntArray,
) {
    main(eth_execution_rpc, starknet_rpc, data_dir, poll_secs, socket_port);
}
