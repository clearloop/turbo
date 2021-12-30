//! shared types

/// web3 api transport
pub type Transport = web3::transports::Either<web3::transports::WebSocket, web3::transports::Http>;
