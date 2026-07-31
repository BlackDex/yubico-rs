//! Custom sync example using `yubico_ng` with `ureq` instead of `reqwest`
//!
//! This example shows how to plug your own HTTP client into `yubico_ng` by
//! implementing the `BlockingTransport` trait. Nothing here touches `reqwest`,
//! so this also works with `default-features = false`.
//!
//! Like the `otp_custom` example it sets a custom `api_host`.

use std::io::stdin;
use std::time::Duration;

use dotenvy::dotenv;
use yubico_ng::{
    Verifier, YubicoError,
    config::Config,
    transport::{BlockingTransport, Response},
};

fn main() {
    match dotenv() {
        Ok(_) => println!("Loaded .env"),
        Err(_) => eprintln!("Unable to load .env, provide proper environment variables manually"),
    }

    println!("Please plug in a yubikey and enter an OTP");

    let client_id = std::env::var("YK_CLIENT_ID")
        .expect("Please set a value to the YK_CLIENT_ID environment variable.");

    let api_key = std::env::var("YK_API_KEY")
        .expect("Please set a value to the YK_API_KEY environment variable.");

    let api_host = std::env::var("YK_API_HOST")
        .expect("Please set a value to the YK_API_HOST environment variable.");

    let config = Config::default()
        .set_client_id(client_id)
        .set_api_host(api_host)
        .set_key(api_key)
        .expect("Invalid API key (must be Base64)");

    let transport = UreqTransport::new("github.com/BlackDex/yubico-rs", Duration::from_secs(30));
    let verifier = Verifier::with_client(config, transport).expect("Unable to create the verifier");

    let otp = read_user_input();

    match verifier.verify_blocking(otp) {
        Ok(()) => println!("Valid OTP."),
        Err(e) => {
            println!("Error '{{e}}': {e}");
            println!("Error '{{e:?}}': {e:?}");
            println!("Error '{{e:#?}}': {e:#?}");
        }
    }
}

fn read_user_input() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Could not read user input.");

    buf
}

/// Wraps a `ureq::Agent` so it can be used as a `yubico_ng` transport.
///
/// An `Agent` owns the connection pool, so build it once and reuse it for
/// every verification instead of creating one per call.
struct UreqTransport {
    agent: ureq::Agent,
}

impl UreqTransport {
    fn new(user_agent: &str, timeout: Duration) -> Self {
        // `Config::set_user_agent`, `set_request_timeout` and `set_proxy` only apply to the
        // built-in reqwest client. When you bring your own client you configure the
        // equivalents on that client yourself.
        let agent = ureq::Agent::config_builder()
            .user_agent(user_agent)
            .timeout_global(Some(timeout))
            .build()
            .new_agent();

        Self { agent }
    }
}

impl BlockingTransport for UreqTransport {
    // Converting to `YubicoError` inside the impl keeps `Verifier` free of any
    // client specific error type. Returning your own error type also works, as long
    // as it implements `Into<YubicoError>`.
    type Error = YubicoError;

    fn yubico_get(&self, url: &str) -> Result<Response, Self::Error> {
        let mut response = self.agent.get(url).call().map_err(YubicoError::transport)?;

        // Read the status before the body, since `body_mut()` borrows mutably
        let status = response.status().as_u16();
        let body = response
            .body_mut()
            .read_to_string()
            .map_err(YubicoError::transport)?;

        Ok(Response { status, body })
    }
}
