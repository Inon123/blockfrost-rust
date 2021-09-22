//! Rust SDK for Blockfrost.io

// Internal testing macros
#[macro_use]
mod macros;

mod api;
mod utils;

/// Utils for loading common settings from config file and environment variables.
pub mod env;
/// Custom errors from this crate.
pub mod error;
/// Definitions for types returned in requests.
pub mod types;

pub use api::*;
pub use error::*;
pub use types::*;

/// The URL of the BlockFrost API for the Cardano mainnet.
pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
/// The URL of the BlockFrost API for the Cardano testnet.
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";
