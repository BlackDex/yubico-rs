//! General sync example using `yubico_ng` with a Proxy
//! This example shows the usage of setting a proxy to be used

use std::io::stdin;

use dotenvy::dotenv;
use yubico_ng::{
    blocking::verify,
    config::{Config, ProxyConfig},
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

    let config = Config::default()
        .set_client_id(client_id)
        .set_key(api_key)
        .expect("Invalid API key (must be Base64)");

    let config = if let Ok(proxy) = std::env::var("YK_PROXY") {
        config.set_proxy(ProxyConfig::new(proxy))
    } else {
        config
    };

    let otp = read_user_input();

    match verify(otp, config) {
        Ok(()) => println!("Valid OTP."),
        Err(e) => println!("Error: {e}"),
    }
}

fn read_user_input() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Could not read user input.");

    buf
}
