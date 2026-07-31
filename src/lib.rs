#![cfg_attr(docsrs, feature(doc_cfg))]

//! # `yubico_ng`
//!
//! A Yubico OTP validation library for Rust.
//!
//! Should be compatible with third-party servers which support the Yubico Validation Protocol Version 2.0.
//!
//! ## Usage
//!
//! Be sure to add the `yubico_ng` crate to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! yubico_ng = "1"
//! ```
//!
//! You need to have a valid Client ID and API Key. These can be obtained from Yubico via <https://upgrade.yubico.com/getapikey/>
//! Without a valid Client ID and API Key you can not validate any OTP key registered at Yubico.
//!
//! ## Features
//!
//! As a default features we have `reqwest` and `default-tls` enabled.
//! This provides quick usage of this crate with reqwest as the transport client.
//!
//! Other features you can use, either together with the default or without the default.
//! - `reqwest`: (default) Enables the reqwest crate and provides client transports for ease of use
//! - `default-tls` (default): Uses the `default-tls` set at the `reqwest` crate
//! - `native-tls`: Uses the `reqwest/native-tls` feature to use the system provided TLS library
//! - `reqwest-blocking`: Provides a blocking reqwest client transport for ease of use
//!   Ensure that if you use `default-features = false` that you also enable a TLS engine either by
//!   using one of the `-tls` features mentioned above, or enable a TLS engine your self
//!
//! ## Example
//!
//! A simple example on how to use this library.
//! Also checkout the `examples` directory for some more examples.
//!
//! ```rust,no_run
//! # #[tokio::main]
//! async fn main() {
//!     use yubico_ng::{config::Config, verify};
//!     // Extract the `client_id` and `api_key` from a safe location, do not embed them in your code!
//!     let client_id = "012345678901";
//!     let api_key = "Base64/Base64/Base64/Base64=";
//!     let config = Config::default()
//!         .set_client_id(client_id)
//!         .set_key(api_key)
//!         .expect("Invalid API key (must be Base64)");
//!
//!     // Retrieve the users OTP via a safe way
//!     let otp = "vvcbdefghijklnrtuvcbdefghijklnrtuvcbdefghijk";
//!
//!     match verify(otp, config).await {
//!         Ok(()) => println!("Valid OTP."),
//!         Err(e) => println!("Error: {e}"),
//!     }
//! }
//! ```
//!
//! For more example on how to use this crate I would suggest to checkout the `examples` directory.
//!

/// Provides async `yubico_ng::verify(otp, cfg).await`
pub mod asynchronous;

/// If `reqwest` is enabled, re-export `verify`
#[cfg(feature = "reqwest")]
pub use asynchronous::verify;

/// Provides blocking `yubico_ng::blocking::verify(otp, cfg)`
pub mod blocking;

/// Provides `Config` and `ProxyConfig`
pub mod config;
/// Provided `YubicoError`
pub mod error;

/// Provide a client agnostic transport if you do not want to use reqwest or use your own client
pub mod transport;

/// The `http` mod is only used to build blocking and async reqwest clients, not needed if not enabled
#[cfg(feature = "reqwest")]
mod http;

mod protocol;
mod sec;

pub use error::YubicoError;

/// Result used by `yubico_ng`
/// Provides either `Ok<T>` or `Err<YubicoError>`
pub type Result<T> = ::std::result::Result<T, YubicoError>;

/// `Verifier` struct which contains the config and HTTP client
#[derive(Debug)]
pub struct Verifier<C> {
    config: config::Config,
    client: C,
}

impl<C> Verifier<C> {
    /// Use a custom HTTP client
    ///
    /// # Errors
    /// Returns `YubicoError::MissingCredentials` if required credentials are missing.
    pub fn with_client(config: config::Config, client: C) -> Result<Self> {
        if config.client_id.is_empty() || config.key.is_empty() {
            return Err(YubicoError::MissingCredentials);
        }
        Ok(Self { config, client })
    }
}
