use std::fmt::Display;
use std::time::Duration;

use base64::{Engine as _, engine::general_purpose::STANDARD};

use crate::error::YubicoError;

/// From the Validation Protocol documentation:
///
/// A value 0 to 100 indicating percentage of syncing between validation servers required by client,
/// or strings "fast" or "secure" to use server-configured values; if absent, let the server decide.
#[derive(Copy, Clone, Debug)]
pub struct SyncLevel(u8);

impl SyncLevel {
    /// The most quick validation, no sync is done between the servers is needed
    /// This could mean a replay attack might be possible
    #[must_use]
    pub fn fast() -> SyncLevel {
        SyncLevel(0)
    }

    /// The most secure validation, all validation servers need to be in-sync
    /// and agree on the current counter state
    #[must_use]
    pub fn secure() -> SyncLevel {
        SyncLevel(100)
    }

    /// A custom value somewhere from 0..=100
    /// Values larger than 100 are capped to 100.
    #[must_use]
    pub fn custom(level: u8) -> SyncLevel {
        SyncLevel(std::cmp::min(level, 100))
    }
}

impl Display for SyncLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

/// `ProxyConfig` struct containing both proxy url and if needed username and password
#[derive(Clone)]
pub struct ProxyConfig {
    pub(crate) url: String,
    pub(crate) auth: Option<(String, String)>, // (username, password)
}

impl ProxyConfig {
    /// Create a new `ProxyConfig`
    /// The proxy needs to be supported by `reqwest`, else this will not work.
    ///
    /// ```rust
    /// use yubico_ng::config::ProxyConfig;
    /// let proxy = ProxyConfig::new("socks5://127.0.0.1:1080");
    /// ```
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            auth: None,
        }
    }

    /// Add a username and password to the proxy configuration
    ///
    /// ```rust
    /// use yubico_ng::config::ProxyConfig;
    /// let proxy = ProxyConfig::new("socks5://127.0.0.1:1080")
    ///     .basic_auth("my-username", "MySecretPassword");
    /// ```
    #[must_use]
    pub fn basic_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.auth = Some((username.into(), password.into()));
        self
    }
}

impl std::fmt::Debug for ProxyConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProxyConfig")
            .field("url", &self.url)
            .field(
                "auth",
                &self
                    .auth
                    .as_ref()
                    .map(|(_, _)| ("<redacted>", "<redacted>")),
            )
            .finish()
    }
}

/// `Config` struct containing all the configuration items used to validate a Yubico OTP
///
/// If you use a custom `Transport` the following fields are not used:
/// - `user_agent`
/// - `request_timeout`
/// - `proxy`
///
/// Ensure you configure your `Transport` to have the correct configuration.
///
/// ```rust
/// use yubico_ng::config::Config;
/// let config = Config::default()
///     .set_client_id("012345678901")
///     .set_key("Base64/Base64/Base64/Base64=")
///     .expect("Invalid API key (must be Base64)");
/// ```
#[derive(Clone)]
pub struct Config {
    pub(crate) client_id: String,
    pub(crate) key: Vec<u8>, // Stores the decoded API key
    pub(crate) api_host: String,
    pub(crate) sync_level: SyncLevel,

    /// The following config options are only used when the `reqwest` feature is enabled
    /// If you use a custom `Transport` ensure you provide your own user-agent, timeout and proxy config
    pub(crate) user_agent: String,
    pub(crate) request_timeout: Duration, // The timeout for HTTP requests.
    pub(crate) proxy: Option<ProxyConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            key: Vec::new(),
            api_host: "https://api.yubico.com/wsapi/2.0/verify".to_owned(),
            sync_level: SyncLevel::secure(),

            // Only used when the `reqwest` feature is enabled
            user_agent: "github.com/BlackDex/yubico-rs".to_owned(),
            request_timeout: Duration::from_secs(30),
            proxy: None,
        }
    }
}

impl Config {
    /// Required config option
    /// Client ID, exactly as issued by <https://upgrade.yubico.com/getapikey/>
    #[must_use]
    pub fn set_client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = client_id.into();
        self
    }

    /// Required config option
    /// Key as Base64 (RFC4648), exactly as issued by <https://upgrade.yubico.com/getapikey/>
    /// That includes one or more `=` characters used for padding, without it will fail
    ///
    /// # Errors
    /// Returns `YubicoError::DecodeError` when the key is not valid base64
    /// or `YubicoError::InvalidKeyLength` if the key length is invalid
    pub fn set_key(mut self, key: impl AsRef<[u8]>) -> Result<Self, YubicoError> {
        let key = STANDARD.decode(key.as_ref())?;
        if key.len() != 20 {
            return Err(YubicoError::InvalidKeyLength);
        }
        self.key = key;
        Ok(self)
    }

    /// The host to use to validate the OTP
    /// Defaults to `https://api.yubico.com/wsapi/2.0/verify`
    #[must_use]
    pub fn set_api_host(mut self, host: impl Into<String>) -> Self {
        self.api_host = host.into();
        self
    }

    /// Set the user-agent to be used during the API calls
    /// Defaults to `github.com/BlackDex/yubico-rs`
    #[must_use]
    pub fn set_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// A value 0 to 100 indicating percentage of syncing between validation servers required by client.
    /// The default is `100` a.k.a. `Secure`
    #[must_use]
    pub fn set_sync_level(mut self, level: SyncLevel) -> Self {
        self.sync_level = level;
        self
    }

    /// The maximum time a request can take before it will timeout
    /// Defaults to 30 seconds
    #[must_use]
    pub fn set_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Manually set a proxy via the `ProxyConfig` struct
    /// By default there is no proxy configuration used unless you configured reqwest to accept `_PROXY` environment variables
    #[must_use]
    pub fn set_proxy(mut self, proxy: ProxyConfig) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("client_id", &"<redacted>")
            .field("key", &"<redacted>")
            .field("api_host", &self.api_host)
            .field("sync_level", &self.sync_level)
            .field("user_agent", &self.user_agent)
            .field("request_timeout", &self.request_timeout)
            .field("proxy", &self.proxy)
            .finish_non_exhaustive()
    }
}
