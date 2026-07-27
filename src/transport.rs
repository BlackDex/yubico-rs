use crate::error::YubicoError;

/// Minimal HTTP response struct
#[derive(Clone, Debug)]
pub struct Response {
    /// HTTP Status code
    pub status: u16,
    /// Response body as returned by the validation server unmodified
    /// NOTE: This might contain sensitive data!
    pub body: String,
}

/// Async transport trait
pub trait AsyncTransport {
    /// Error returned by this transport
    // type Error: std::error::Error + Send + Sync + 'static;
    type Error: Into<YubicoError>;

    /// We only need `GET` for Yubico OTP validation
    ///
    /// # Errors
    /// Returns `YubicoError::Transport` containing the original error from the used client
    fn yubico_get(&self, url: &str) -> impl Future<Output = Result<Response, Self::Error>> + Send;
}

/// If the `reqwest` feature is enabled, which is by default we implement a `Transport` for `reqwest::Client`
#[cfg(feature = "reqwest")]
impl AsyncTransport for reqwest::Client {
    type Error = YubicoError;

    /// # Errors
    /// Returns `YubicoError::Transport` containing the original error from the used client
    async fn yubico_get(&self, url: &str) -> Result<Response, Self::Error> {
        let response = self.get(url).send().await.map_err(YubicoError::transport)?;
        Ok(Response {
            status: response.status().as_u16(),
            body: response.text().await.map_err(YubicoError::transport)?,
        })
    }
}

/// Blocking transport trait
pub trait BlockingTransport {
    /// Error returned by this transport
    type Error: Into<YubicoError>;

    /// We only need `GET` for Yubico OTP validation
    ///
    /// # Errors
    /// Returns `YubicoError::Transport` containing the original error from the used client
    fn yubico_get(&self, url: &str) -> Result<Response, Self::Error>;
}

/// If the `blocking` feature is enabled, which implies reqwest
/// create a `BlockingTransport` for `reqwest::blocking::Client`
#[cfg(feature = "reqwest-blocking")]
impl BlockingTransport for reqwest::blocking::Client {
    type Error = YubicoError;

    /// # Errors
    /// Returns `YubicoError::Transport` containing the original error from the used client
    fn yubico_get(&self, url: &str) -> Result<Response, Self::Error> {
        let response = self.get(url).send().map_err(YubicoError::transport)?;
        Ok(Response {
            status: response.status().as_u16(),
            body: response.text().map_err(YubicoError::transport)?,
        })
    }
}
