//! hdwallet key generator follows bip32 with bip39.
use crate::{
    key::{Account, Wallet},
    Result,
};
use clap::Parser;

/// hdwallet key generator follows bip32 with bip39.
///
/// This implementation uses the `secp256k1` algorithm.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub enum Key {
    /// Generate new wallet
    Generate,
    /// Derive new keys from {seed,mnemonic}
    Derive {
        /// The {seed,mnemonic} to derive from
        suri: String,
        /// Derive from the given index
        #[clap(short, long, default_value = "0")]
        from: u32,
        /// Derive the given number of keys
        #[clap(short, long, default_value = "1")]
        count: u32,
    },
}

impl Key {
    /// Run the key command.
    pub fn run(&self) -> Result<()> {
        match self {
            Key::Generate => {
                let mnemonic = Wallet::mnemonic()?;
                println!("MNEMONIC: {}", mnemonic);
                println!("{:#}", Wallet::from_mnemonic(&mnemonic)?.root());
            }
            Key::Derive { suri, from, count } => {
                let wallet = Wallet::from(suri.as_str());
                for i in *from..*from + *count {
                    println!("{:#}", Account::from(wallet.derive(i)?));
                }
            }
        }

        Ok(())
    }
}
