//! turbo contracts

/// contract metadata
pub trait Metadata {
    /// contract abi
    fn abi() -> Vec<u8>;

    /// contract address
    fn address() -> [u8; 20];
}

#[cfg(any(feature = "uniswap-v2", feautre = "uniswap-v3"))]
pub mod uniswap {
    /// uniswap v2
    #[cfg(feature = "uniswap-v2")]
    pub mod v2 {
        #[turbo_abi_derive::meta(0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f)]
        pub struct UniswapV2;
    }

    /// uniswap v3
    #[cfg(feature = "uniswap-v3")]
    pub mod v3 {
        #[turbo_abi_derive::meta(0xE592427A0AEce92De3Edee1F18E0157C05861564)]
        pub struct UniswapV3;
    }
}
