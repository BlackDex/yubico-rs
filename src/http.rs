use crate::{Result, config::Config, error::YubicoError};

pub(crate) fn build_proxy(config: &Config) -> Result<Option<reqwest::Proxy>> {
    let Some(proxy) = &config.proxy else {
        return Ok(None);
    };

    let mut p = reqwest::Proxy::all(&proxy.url).map_err(YubicoError::transport)?;
    if let Some((username, password)) = &proxy.auth {
        p = p.basic_auth(username, password);
    }

    Ok(Some(p))
}

macro_rules! build_client {
    ($builder:expr, $config:expr) => {{
        let config: &$crate::config::Config = $config;
        let mut builder = $builder
            .timeout(config.request_timeout)
            .user_agent(&config.user_agent);

        if let Some(proxy) = $crate::http::build_proxy(config)? {
            builder = builder.proxy(proxy);
        }

        builder
            .build()
            .map_err($crate::error::YubicoError::transport)
    }};
}

pub(crate) use build_client;
