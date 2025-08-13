use futures::TryFutureExt;

use dotenvy::dotenv;
use std::io::stdin;
use yubico_ng::config::Config;
use yubico_ng::verify_async;

#[tokio::main]
async fn main() -> Result<(), ()> {
    match dotenv() {
        Ok(_) => println!("Loaded .env"),
        Err(_) => eprintln!("Unable to load .env, provide proper environment variables manually"),
    }

    println!("Please plug in a yubikey and enter an OTP");

    let client_id = std::env::var("YK_CLIENT_ID")
        .expect("Please set a value to the YK_CLIENT_ID environment variable.");

    let api_key = std::env::var("YK_API_KEY")
        .expect("Please set a value to the YK_API_KEY environment variable.");

    let config = Config::default().set_client_id(client_id).set_key(api_key);

    let otp = read_user_input();

    verify_async(otp, config)
        .map_ok(|()| println!("Valid OTP."))
        .map_err(|err| println!("Invalid OTP. Cause: {err:?}"))
        .await
}

fn read_user_input() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Could not read user input.");

    buf
}
