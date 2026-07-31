use crate::{Result, Verifier, protocol::build_request, transport::BlockingTransport};

impl<C: BlockingTransport> Verifier<C> {
    /// Verify a provided OTP token. `Ok<()>` if valid `Err<YubicoError>` if not or something else went wrong.
    ///
    /// # Errors
    /// Return `YubicoError::HTTPStatus` or `YubicoError::Transport` when HTTP call fails.
    pub fn verify_blocking(&self, otp: impl Into<String>) -> Result<()> {
        let request = build_request(otp, &self.config)?;
        let response = self.client.yubico_get(&request.url()).map_err(Into::into)?;
        request.verify_transport_response(&response)
    }
}

#[cfg(feature = "reqwest-blocking")]
use crate::config::Config;

/// # Errors
/// Will return `YubicoError` if
/// - Something in `config` results in any error
/// - `reqwest` Generates an error during `send()`
/// - Verifying the response failed
#[cfg(feature = "reqwest-blocking")]
pub fn verify(otp: impl Into<String>, config: Config) -> Result<()> {
    Verifier::<reqwest::blocking::Client>::new(config)?.verify_blocking(otp)
}

#[cfg(feature = "reqwest-blocking")]
impl Verifier<reqwest::blocking::Client> {
    /// A Convenience wrapper that builds a fresh HTTP client for a single verification.
    ///
    /// If you verify more than one OTP, build a [`Verifier`] once and reuse it
    /// This function creates a new connection pool on every call.
    ///
    /// # Errors
    /// Returns `YubicoError::Transport` when the HTTP client cannot be built or
    /// `YubicoError::MissingCredentials` if required credentials are missing.
    pub fn new(config: Config) -> Result<Self> {
        let client = crate::http::build_client!(reqwest::blocking::Client::builder(), &config)?;
        Self::with_client(config, client)
    }
}
