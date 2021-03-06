//! Configuration for software-backed signer (using ed25519-dalek)

use std::path::{Path, PathBuf};

/// Software signer configuration
#[derive(Clone, Deserialize, Debug)]
pub struct SoftSignConfig {
    /// Identifier for this key
    pub id: String,

    /// Path to a file containing a cryptographic key
    // TODO: use `abscissa::Secret` to wrap this `PathBuf`
    pub path: SoftPrivateKey,
}

/// Software-backed private key (stored in a file)
#[derive(Clone, Deserialize, Debug)]
pub struct SoftPrivateKey(PathBuf);

impl SoftPrivateKey {
    /// Borrow this private key as a path
    pub fn as_path(&self) -> &Path {
        self.0.as_ref()
    }
}
