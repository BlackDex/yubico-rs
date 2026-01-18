# Yubico NG &emsp; [![Latest Version]][crates.io] [![deps.rs]][deps.rs] [![MIT licensed]][MIT] [![Apache-2.0 licensed]][APACHE]

[Latest Version]: https://img.shields.io/crates/v/yubico_ng.svg
[crates.io]: https://crates.io/crates/yubico_ng
[deps.rs]: https://deps.rs/repo/github/BlackDex/yubico-rs/status.svg
[MIT licensed]: https://img.shields.io/badge/License-MIT-blue.svg
[MIT]: ./LICENSE-MIT
[Apache-2.0 licensed]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[APACHE]: ./LICENSE-APACHE

**Enables integration with the Yubico validation platform, so you can use Yubikey's one-time-password in your Rust application, allowing a user to authenticate via Yubikey.**

---

## Current features

- [X] Synchronous Yubikey client API library, [validation protocol version 2.0](https://developers.yubico.com/OTP/Specifications/OTP_validation_protocol.html).
- [X] Asynchronous Yubikey client API library relying on [Tokio](https://github.com/tokio-rs/tokio)

**Note:** The USB-related features have been moved to a separated repository, [yubico-manager](https://github.com/wisespace-io/yubico-manager)

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
yubico_ng = "0.14"
```

Or, since this crate is still backwards compatible with the yubico crate.
```toml
[dependencies]
yubico = { version = "0.14", package = "yubico_ng" }
```

The following are a list of Cargo features that can be enabled or disabled:

- online-tokio (enabled by default): Provides integration to Tokio using futures.
- native-tls (enabled by default): Use native-tls provided by the OS.
- rustls-tls: Use rustls instead of native-tls.

You can enable or disable them using the example below:

  ```toml
  [dependencies.yubico_ng]
  version = "0.13"
  # don't include the default features (online-tokio, native-tls)
  default-features = false
  # cherry-pick individual features
  features = []
  ```

[Request your api key](https://upgrade.yubico.com/getapikey/).

### OTP with Default Servers

```rust
extern crate yubico_ng;

use yubico_ng::config::*;
use yubico_ng::verify;

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
use yubico_ng::verify;
use yubico_ng::config::Config;

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
use yubico_ng::config::Config;
use yubico_ng::verify_async;

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

- 0.15.0 (2026-01-18):
    * Use reqwest v0.13 or higher
    * Switched to edition 2024
    * Set MSRV to v1.85.0 which supports edition 2024 by default
    * Removed `native-tls` and `rustls-tls` and use `reqwest/default-tls` by default.<br>
      All other reqwest features are disabled in this crate it self!

    #### Hightlights

    In this version I removed the specific `reqwest` features because it would limit `reqwest` to those specific features.<br>
    Also updated to `reqwest` v0.13 as a minimal version. If you need to use v0.12 of `reqwest`, just keep using v0.14 of `yubico_ng`.<br>
    I default to the `default-tls` feature via the `default` feature of the crate it self, which should be fine for most use cases.

    If you want to use anything else besides `default-tls`, use `default-features = false`, define `reqwest` as a custom dependency and define the wanted features. This way you can use `rustls-no-provider` and use any provider supported by `reqwest`.

    ```toml
    [dependencies]
    yubico_ng = { version = "0.15", default-features = false, features = ["online-tokio"] }
    reqwest = { version = "0.13.1", default-features = false, features = ["rustls-no-provider"] }
    rustls = { version = "0.23.36", default-features = false, features = ["ring"] }
    ```

    ```rust
    fn main() {
        // Initialize rustls with ring so reqwest v0.13+ will work without aws-lc for example
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider for Reqwest");
    }
    ```

- 0.14.1 (2025-08-13):
    * Exclude several files from the crate package

- 0.14.0 (2025-08-13) (not published to crates.io):
    * Upgrade to `tokio` 1.47
    * Bumped MSRV to v1.82.0 needed by latest packages
    * Added more clippy/rust lints including `pedantic` and fixed found items
    * Use only the main api server, the others are deprecated
    * Updated GHA
    * Added dotenvy as a dev dependency to load `.env` files

- 0.13.0 (2025-04-23):
    * Upgrade to `tokio` 1.44, `rand` 0.9
    * Renamed to yubico_ng and published crate
    * Made edition 2024 compatible
    * Added several clippy/rust lints and fixed those
    * Fixed a panic if the `YK_API_HOST` was invalid
    * Use only the main api server, the others are deprecated
    * Run cargo fmt
    * Updated GHA to use hashes and run/fix zizmor

- 0.12.0: Upgrade to `tokio` 1.37, `reqwest` 0.12, `base64` 0.22, clippy fixes.
- 0.10.0: Upgrade to `tokio` 1.1 and `reqwest` 0.11
- 0.9.2: (Yanked) Dependencies update
- 0.9.1: Set HTTP Proxy (Basic-auth is optional)
- 0.9.0: Moving to `tokio` 0.2 and `reqwest` 0.10
- 0.9.0-alpha.1: Moving to `futures` 0.3.0-alpha.19
- 0.8: Rename the `sync` and `async` modules to `sync_verifier` and `async_verifier` to avoid the use of the `async` reserved keyword.
