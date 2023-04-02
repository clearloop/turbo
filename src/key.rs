//! Hierarchical deterministic wallet with bip39 support.
use crate::{
    result::{Error, Result},
    utils::keccak,
};
use bip32::XPrv;
use bip39::{Language, Mnemonic};
use rand_core::OsRng;
use std::fmt;

// TODO: support other networks
const ETH_HD_PATH: &'static str = "m/44'/60'/0'/0";

/// Wallet account.
pub struct Account {
    key: [u8; 32],
    address: [u8; 20],
}

impl From<XPrv> for Account {
    fn from(xprv: XPrv) -> Self {
        let mut key = [0u8; 32];
        key.copy_from_slice(&xprv.to_bytes());

        let uncompressed_pub = xprv.public_key().public_key().to_encoded_point(false);
        let mut address = [0u8; 20];
        address.copy_from_slice(&keccak(&uncompressed_pub.as_bytes()[1..])[12..]);

        Self { key, address }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account")
            .field("key", &("0x".to_owned() + &hex::encode(&self.key)))
            .field(
                "address",
                &("0x".to_owned() + &hex::encode(&self.address).to_uppercase()),
            )
            .finish()
    }
}

/// Hierarchical deterministic wallet with bip39 support.
pub struct Wallet {
    root: XPrv,
    _accounts: Vec<XPrv>,
}

impl Wallet {
    /// Generate random mnemonic.
    pub fn mnemonic() -> Result<String> {
        Ok(Mnemonic::generate_in_with(&mut OsRng, Default::default(), 12)?.to_string())
    }

    /// Create a new wallet.
    pub fn new() -> Result<Self> {
        Self::from_mnemonic(&Self::mnemonic()?)
    }

    /// Get the root account.
    pub fn root(&self) -> Account {
        self.root.clone().into()
    }

    /// Create a new wallet from a seed.
    pub fn from_seed(seed: [u8; 64]) -> Result<Self> {
        Ok(Self {
            root: XPrv::derive_from_path(&seed, &ETH_HD_PATH.parse()?)?,
            _accounts: Default::default(),
        })
    }

    /// Create a new wallet from a seed slice.
    pub fn from_seed_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != 64 {
            return Err(Error::InvalidSeedLength(slice.len()));
        }

        let mut seed = [0u8; 64];
        seed.copy_from_slice(slice);

        Ok(Self::from_seed(seed)?)
    }

    /// Create a new wallet from a mnemonic.
    pub fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        Self::from_seed(Mnemonic::parse_in(Language::English, mnemonic)?.to_seed(""))
            .map_err(Into::into)
    }

    /// Get the master key at a given index.
    ///
    /// This is equivalent to `master_key().derive_child(index)`.
    pub fn derive(&self, index: u32) -> Result<XPrv> {
        self.root.derive_child(index.into()).map_err(Into::into)
    }
}

impl From<&[u8]> for Wallet {
    fn from(seed: &[u8]) -> Self {
        Self::from_seed_slice(&seed).expect("invalid seed length")
    }
}

impl From<[u8; 32]> for Wallet {
    fn from(seed: [u8; 32]) -> Self {
        Self::from_seed_slice(&seed).expect("invalid seed length")
    }
}

impl From<[u8; 64]> for Wallet {
    fn from(seed: [u8; 64]) -> Self {
        Self::from_seed(seed).expect("invalid seed")
    }
}

impl From<&str> for Wallet {
    fn from(mnemonic: &str) -> Self {
        if let Ok(wallet) = Self::from_mnemonic(mnemonic) {
            return wallet;
        }

        Self::from_seed_slice(
            &hex::decode(mnemonic.trim_start_matches("0x").to_lowercase()).expect("invalid seed"),
        )
        .expect("invalid seed")
    }
}

impl From<XPrv> for Wallet {
    fn from(key: XPrv) -> Self {
        Self::from_seed_slice(&key.to_bytes()).expect("invalid private key")
    }
}

#[test]
fn test_keypair() {
    // fuzz from https://github.com/danfinlay/mnemonic-account-generator/blob/master/generators/ethereum.js
    use bip32::Prefix;
    let wallet =
        Wallet::from("lens shove senior gun subway hero transfer image ozone shield twelve cause");

    assert_eq!(
        &wallet.root.to_string(Prefix::XPRV).to_string(),
        "xprvA23Bmox1Y85V4YU5uSx36f6PcZzKKEcHxAY2iM7qi5qwrqnXNyyheZGKonKTyKohb7Z4ccHW41m61FW9jE8TEDyGygpSy7GqrjFRRd99iVE"
    );

    assert_eq!(
        &wallet.root.public_key().to_string(Prefix::XPUB).to_string(),
        "xpub6F2YBKUuNVdnH2YZ1UV3To38AbpoihL9KPTdWjXTGRNvje7fvXHxCMaof29uvXYW6qm8wDXQsG21tUZxJAswzWKGc8jHjqDFSznuwnGrNPj"
    );
}

#[test]
fn derived_key() {
    // fuzz from https://github.com/danfinlay/mnemonic-account-generator/blob/master/generators/ethereum.js
    let wallet =
        Wallet::from("lens shove senior gun subway hero transfer image ozone shield twelve cause");
    let key = wallet.derive(0).expect("Failed to derive key from index 0");

    assert_eq!(
        hex::encode(key.to_bytes()),
        "12a84e82f632a77649d25d35ec8d6a6f52c4be82db23730b7a24179e385cc3cf"
    );

    assert_eq!(
        hex::encode(
            &key.public_key()
                .public_key()
                .to_encoded_point(false)
                .as_bytes()[1..]
        ),
        "2a56cee2f93b2bd3cd6f70db54d3bb516288f5abad717751f8486f78a48d93b1d259b99af4a9a2f69ffe0c2b900499400ce18fd38e0c0940825b8220826498d7"
    );
}

#[test]
fn public_key_to_address() {
    // fuzz from https://www.quicknode.com/guides/ethereum-development/wallets/how-to-generate-a-new-ethereum-address-in-python/
    let pk = "0x04345f1a86ebf24a6dbeff80f6a2a574d46efaa3ad3988de94aa68b695f09db9ddca37439f99548da0a1fe4acf4721a945a599a5d789c18a06b20349e803fdbbe3";
    let pk_bytes = hex::decode(pk.trim_start_matches("0x04")).expect("invalid public key");

    assert_eq!(pk_bytes.len(), 64);
    assert_eq!(
        hex::encode(&keccak(pk_bytes)[12..]),
        "d5e099c71b797516c10ed0f0d895f429c2781142"
    );
}
