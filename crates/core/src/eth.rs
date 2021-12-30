//! eth client
use crate::{
    result::{Error, Result},
    types::Transport,
};
use web3::{
    transports::{Http, WebSocket},
    types::BlockId,
    Web3,
};

/// Ethereum client
pub struct Eth(Web3<Transport>);

impl Eth {
    /// new etheruem client with url, either ws or http
    pub async fn new(url: &str) -> Result<Self> {
        let trans = if url.starts_with("ws") {
            Transport::Left(WebSocket::new(url).await?)
        } else if url.starts_with("http") {
            Transport::Right(Http::new(url)?)
        } else {
            return Err(Error::InvalidEndpoint);
        };

        Ok(Self(Web3::new(trans)))
    }

    /// testing
    pub async fn test_block(&self) -> Result<()> {
        let latest = self.0.eth().block_number().await?.as_u64();
        let block = self.0.eth().block(BlockId::Number(latest.into())).await?;
        println!("{:?}", block);

        Ok(())
    }
}
