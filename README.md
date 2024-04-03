# Yubico &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![MIT licensed]][MIT] [![Apache-2.0 licensed]][APACHE]

[Build Status]: https://travis-ci.org/wisespace-io/yubico-rs.png?branch=master
[travis]: https://travis-ci.org/wisespace-io/yubico-rs
[Latest Version]: https://img.shields.io/crates/v/yubico.svg
[crates.io]: https://crates.io/crates/yubico
[MIT licensed]: https://img.shields.io/badge/License-MIT-blue.svg
[MIT]: ./LICENSE-MIT
[Apache-2.0 licensed]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[APACHE]: ./LICENSE-APACHE

**Enables integration with the Yubico validation platform, so you can use Yubikey's one-time-password in your Rust application, allowing a user to authenticate via Yubikey.**

---

## Current features

- [X] Synchronous Yubikey client API library, [validation protocol version 2.0](https://developers.yubico.com/yubikey-val/Validation_Protocol_V2.0.html).
- [X] Asynchronous Yubikey client API library relying on [Tokio](https://github.com/tokio-rs/tokio)

**Note:** The USB-related features have been moved to a sepatated repository, [yubico-manager](https://github.com/wisespace-io/yubico-manager)

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
yubico = "0.12"
```

The following are a list of Cargo features that can be enabled or disabled:

- online-tokio (enabled by default): Provides integration to Tokio using futures.

You can enable or disable them using the example below:

  ```toml
  [dependencies.yubico]
  version = "0.12"
  # don't include the default features (online-tokio)
  default-features = false
  # cherry-pick individual features
  features = []
  ```

[Request your api key](https://upgrade.yubico.com/getapikey/).

### OTP with Default Servers

```rust
extern crate yubico;

use yubico::config::*;
use yubico::verify;

fn main() {
   let config = Config::default()
       .set_client_id("CLIENT_ID")
       .set_key("API_KEY");

   match verify("OTP", config) {
      Ok(answer) => println!("{}", answer),
      Err(e) => println!("Error: {}", e),
   }
}
```

## OTP with custom API servers

```rust
use yubico::verify;
use yubico::config::Config;

fn main() {
   let config = Config::default()
       .set_client_id("CLIENT_ID")
       .set_key("API_KEY")
       .set_api_hosts(vec!["https://api.example.com/verify".into()]);

   match verify("OTP", config) {
      Ok(answer) => println!("{}", answer),
      Err(e) => println!("Error: {}", e),
   }
}
```

### Asynchronous OTP validation

```rust
#![recursion_limit="128"]
use futures::TryFutureExt;

use std::io::stdin;
use yubico::config::Config;
use yubico::verify_async;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Please plug in a yubikey and enter an OTP");

    let client_id = std::env::var("YK_CLIENT_ID")
        .expect("Please set a value to the YK_CLIENT_ID environment variable.");

    let api_key = std::env::var("YK_API_KEY")
        .expect("Please set a value to the YK_API_KEY environment variable.");

    let config = Config::default()
        .set_client_id(client_id)
        .set_key(api_key);

    let otp = read_user_input();

    verify_async(otp, config)
        .map_ok(|()| println!("Valid OTP."))
        .map_err(|err| println!("Invalid OTP. Cause: {:?}", err))
        .await
}

fn read_user_input() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Could not read user input.");
    buf
}
```

## Docker

For convenience and reproducibility, a Docker image can be generated via the provided repo's Dockerfile.

### General

You can use a build-arg to select which example to be used. For example use `--build-arg=EXAMPLE=otp_async` to build the async example instead of the default `otp` example.

Build:
```bash
$ docker build -t yubico-rs .
...
Successfully built 983cc040c78e
Successfully tagged yubico-rs:latest
```

Run:
```bash
$ docker run --rm -it -e YK_CLIENT_ID=XXXXX -e YK_API_KEY=XXXXXXXXXXXXXX yubico-rs:latest
Please plug in a yubikey and enter an OTP
ccccccXXXXXXXXXXXXXXXXXXXX
The OTP is valid.
```

### Static

A static binary can be extracted from the container and run on almost any Linux system.

Build:
```bash
$ docker build -t yubico-rs-static . -f Dockerfile.static
...
Successfully built 983cc040c78e
Successfully tagged yubico-rs-static:latest
```

Run:
```bash
$ docker run --rm -it -e YK_CLIENT_ID=XXXXX -e YK_API_KEY=XXXXXXXXXXXXXX yubico-rs-static:latest
Please plug in a yubikey and enter an OTP
ccccccXXXXXXXXXXXXXXXXXXXX
The OTP is valid.
```

## Changelog

    - 0.12.0: Upgrade to `tokio` 1.37, `reqwest` 0.12, `base64` 0.22, clippy fixes.
    - 0.10.0: Upgrade to `tokio` 1.1 and `reqwest` 0.11
    - 0.9.2: (Yanked) Dependencies update
    - 0.9.1: Set HTTP Proxy (Basic-auth is optional)
    - 0.9.0: Moving to `tokio` 0.2 and `reqwest` 0.10
    - 0.9.0-alpha.1: Moving to `futures` 0.3.0-alpha.19
    - 0.8: Rename the `sync` and `async` modules to `sync_verifier` and `async_verifier` to avoid the use of the `async` reserved keyword.
