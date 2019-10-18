use reqwest::header::USER_AGENT;
use reqwest::Client;

use crate::config::Config;
use crate::yubicoerror::YubicoError;
use crate::{build_request, Request, Result};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::sync::Arc;

pub async fn verify_async<S>(otp: S, config: Config) -> Result<()>
where
    S: Into<String>,
{
    AsyncVerifier::new(config)?.verify(otp).await
}

pub struct AsyncVerifier {
    client: Client,
    config: Config,
}

impl AsyncVerifier {
    pub fn new(config: Config) -> Result<AsyncVerifier> {
        let client = Client::builder().timeout(config.request_timeout).build()?;

        Ok(AsyncVerifier { client, config })
    }

    pub async fn verify<S>(&self, otp: S) -> Result<()>
    where
        S: Into<String>,
    {
        let request = Arc::new(build_request(otp, &self.config)?); // Arc because we need the future to be Send.

        let mut responses = FuturesUnordered::new();
        self.config
            .api_hosts
            .iter()
            .for_each(|api_host| responses.push(self.request(request.clone(), api_host)));

        let mut errors = vec![];

        while let Some(response) = responses.next().await {
            match response {
                Ok(()) => return Ok(()),
                Err(err @ YubicoError::ReplayedRequest) => errors.push(err),
                Err(YubicoError::HTTPStatusCode(code)) => {
                    errors.push(YubicoError::HTTPStatusCode(code))
                }
                Err(err) => return Err(err),
            }
        }

        Err(YubicoError::MultipleErrors(errors))
    }

    async fn request(&self, request: Arc<Request>, api_host: &str) -> Result<()> {
        let url = request.build_url(api_host);
        let http_request = self
            .client
            .get(&url)
            .header(USER_AGENT, self.config.user_agent.clone());

        let response = http_request.send().await?;
        let status_code = response.status();

        if !status_code.is_success() {
            return Err(YubicoError::HTTPStatusCode(status_code));
        }

        let text = response.text().await?;

        request.response_verifier.verify_response(text)
    }
}
