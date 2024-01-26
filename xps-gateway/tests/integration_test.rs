use anyhow::Error;
use jsonrpsee::{
    server::Server,
    ws_client::{WsClient, WsClientBuilder},
};

use futures::future::FutureExt;
use std::{future::Future, time::Duration};
use tokio::time::timeout as timeout_tokio;

use ethers::providers::{Http, Provider};
use xps_gateway::{rpc::*, types::Message, XpsMethods};

const TEST_TIMEOUT: Duration = Duration::from_secs(10);
pub const SERVER_HOST: &str = "127.0.0.1";

pub const DID_ETH_REGISTRY: &str = "0xd1D374DDE031075157fDb64536eF5cC13Ae75000";
pub(crate) const DEFAULT_PROVIDER: &str = "http://127.0.0.1:8545";

#[cfg(test)]
mod it {
    use ethers::abi::Address;

    use super::*;

    #[tokio::test]
    async fn test_say_hello() -> Result<(), Error> {
        with_xps_client(None, |client| async move {
            let result = client.status().await?;
            assert_eq!(result, "OK");
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_fail_send_message() -> Result<(), Error> {
        with_xps_client(None, |client| async move {
            let message = Message {
                conversation_id: b"abcdefg".iter().map(|c| *c as u8).collect(),
                payload: b"Hello World".iter().map(|c| *c as u8).collect(),
                v: vec![],
                r: vec![],
                s: vec![],
            };
            let result = client.send_message(message).await;
            assert!(result.is_err());
            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn test_wallet_address() -> Result<(), Error> {
        with_xps_client(None, |client| async move {
            let result = client.wallet_address().await?;
            assert_ne!(result, Address::zero());
            Ok(())
        })
        .await
    }
}

async fn with_xps_client<F, R, T>(timeout: Option<Duration>, f: F) -> Result<T, Error>
where
    F: FnOnce(WsClient) -> R + 'static + Send,
    R: Future<Output = Result<T, Error>> + FutureExt + Send + 'static,
{
    let server_addr = format!("{}:{}", SERVER_HOST, 0);
    let server = Server::builder().build(server_addr).await.unwrap();
    let addr = server.local_addr().unwrap();
    let provider = Provider::<Http>::try_from(DEFAULT_PROVIDER).unwrap();
    let xps_methods = XpsMethods::new(provider, DID_ETH_REGISTRY.to_string());
    let handle = server.start(xps_methods.into_rpc());
    let client = WsClientBuilder::default()
        .build(&format!("ws://{addr}"))
        .await
        .unwrap();
    let result = timeout_tokio(timeout.unwrap_or(TEST_TIMEOUT), f(client)).await;

    handle.stop().unwrap();
    handle.stopped().await;

    match result {
        Ok(v) => v,
        Err(_) => panic!("Test timed out"),
    }
}
