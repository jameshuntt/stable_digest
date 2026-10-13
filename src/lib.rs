//! A stable, `no_std` content digest.
//!
//! [`StableDigest`] is a SHA-256 of some byte material, rendered as a lowercase
//! hex string. It is deliberately tiny and generic — content-addressing has no
//! business living inside a diagnostics crate, so this is the shared primitive
//! that `liaise` (and anything else that needs a stable digest) builds on.
//!
//! The rendering is fixed (lowercase hex, 64 chars) so the same material always
//! produces the same string on any platform — safe to embed in receipts, keys,
//! or on-disk artifacts.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::string::String;
use sha2::{Digest, Sha256};

/// A SHA-256 digest of some material, held as a lowercase-hex string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableDigest(String);

impl StableDigest {
    /// Digest arbitrary byte material into a stable lowercase-hex SHA-256.
    pub fn sha256(material: impl AsRef<[u8]>) -> Self {
        let digest = Sha256::digest(material.as_ref());
        let mut out = String::with_capacity(64);
        for byte in digest {
            use core::fmt::Write;
            let _ = write!(&mut out, "{byte:02x}");
        }
        Self(out)
    }

    /// The digest as a lowercase-hex string slice (always 64 chars).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for StableDigest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}
