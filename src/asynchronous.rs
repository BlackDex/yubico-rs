use crate::{Result, Verifier, protocol::build_request, transport::AsyncTransport};

impl<C: AsyncTransport> Verifier<C> {
    /// Verify a provided OTP token. `Ok<()>` if valid `Err<YubicoError>` if not or something else went wrong.
    ///
    /// # Errors
    /// Returns `YubicoError::HTTPStatus` or `YubicoError::Transport` when HTTP call fails.
    pub async fn verify(&self, otp: impl Into<String>) -> Result<()> {
        let request = build_request(otp, &self.config)?;
        let response = self
            .client
            .yubico_get(&request.url())
            .await
            .map_err(Into::into)?;
        request.verify_transport_response(&response)
    }
}

#[cfg(feature = "reqwest")]
use crate::config::Config;

/// # Errors
/// Will return `YubicoError` if
/// - Something in `config` results in any error
/// - `reqwest` Generates an error during `send()`
/// - Verifying the response failed
#[cfg(feature = "reqwest")]
pub async fn verify(otp: impl Into<String>, config: Config) -> Result<()> {
    Verifier::<reqwest::Client>::new(config)?.verify(otp).await
}

#[cfg(feature = "reqwest")]
impl Verifier<reqwest::Client> {
    /// A Convenience wrapper that builds a fresh HTTP client for a single verification.
    ///
    /// If you verify more than one OTP, build a [`Verifier`] once and reuse it
    /// This function creates a new connection pool on every call.
    ///
    /// # Errors
    /// Returns `YubicoError::Transport` when the HTTP client cannot be built
    pub fn new(config: Config) -> Result<Self> {
        let client = crate::http::build_client!(reqwest::Client::builder(), &config)?;
        Self::with_client(config, client)
    }
}
