//! utils
use sha3::{Digest, Keccak256 as Keccak};

/// keccak hash
pub fn keccak(bytes: impl AsRef<[u8]>) -> [u8; 32] {
    let mut keccak = Keccak::default();
    keccak.update(bytes.as_ref());
    let mut out = [0u8; 32];
    out.copy_from_slice(&keccak.finalize());
    out
}
