# Yubico NG &emsp; [![Latest Version]][crates.io] [![deps.rs]][deps.rs] [![MIT licensed]][MIT] [![Apache-2.0 licensed]][APACHE]

[Latest Version]: https://img.shields.io/crates/v/yubico_ng.svg
[crates.io]: https://crates.io/crates/yubico_ng
[deps.rs]: https://deps.rs/repo/github/BlackDex/yubico-rs/status.svg
[MIT licensed]: https://img.shields.io/badge/License-MIT-blue.svg
[MIT]: ./LICENSE
[Apache-2.0 licensed]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[APACHE]: ./LICENSE

**Enables integration with the Yubico validation platform, so you can use Yubikey's one-time-password in your Rust application, allowing a user to authenticate via Yubikey.**

---

## Current features

- [X] Synchronous Yubikey client API library, [Validation protocol version 2.0](https://developers.yubico.com/OTP/Specifications/OTP_validation_protocol.html).
- [X] Asynchronous Yubikey client API library relying on [Tokio](https://github.com/tokio-rs/tokio)

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
yubico_ng = "1"
```

Or, since this crate is still backwards compatible with the yubico crate.
```toml
[dependencies]
yubico = { version = "1", package = "yubico_ng" }
```

The following are a list of Cargo features that can be enabled or disabled:

- `reqwest`: (default) Enables the reqwest crate and provides client transports for ease of use
- `default-tls` (default): Uses the `default-tls` set at the `reqwest` crate
- `native-tls`: Uses the `reqwest/native-tls` feature to use the system provided TLS library
- `reqwest-blocking`: Provides a blocking reqwest client transport for ease of use
  Ensure that if you use `default-features = false` that you also enable a TLS engine either by
  using one of the `-tls` features mentioned above, or enable a TLS engine your self

You can enable or disable them using the example below:

```toml
# Enable blocking support
[dependencies]
yubico_ng = { version = "1", default-features = true, features = ["reqwest-blocking"] }

# Or, use reqwest's native-tls
[dependencies]
yubico_ng = { version = "1", default-features = false, features = ["native-tls"] }

# Or, use your own client transport like ureq maybe
[dependencies]
yubico_ng = { version = "1", default-features = false }
```

[Request your api key](https://upgrade.yubico.com/getapikey/).

### OTP with Default Servers

```rust
use yubico_ng::{config::Config, verify};

#[tokio::main]
async fn main() {
    // Extract the `client_id` and `api_key` from a safe location, do not embed them in your code!
    let client_id = "012345678901";
    let api_key = "Base64/Base64/Base64/Base64=";
    let config = Config::default()
        .set_client_id(client_id)
        .set_key(api_key)
        .expect("Invalid API key (must be Base64)");
    // Retrieve the users OTP via a safe way
    let otp = "vvikighdhkhehvgvhuhidtikighdhkhehvgvhuhigvik";
    match verify(otp, config).await {
        Ok(()) => println!("Valid OTP."),
        Err(e) => println!("Error: {e}"),
    }
}
```

## Docker

For convenience and reproducibility, a Docker image can be generated via the provided repo's Dockerfile.

### General

You can use a build-arg to select which example to be used. For example use `--build-arg=EXAMPLE=otp --build-arg=FEATURES="--features reqwest-blocking"` to build the blocking example instead of the default `otp_async` example.

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

- 1.0.0 (2026-07-31):
    ### This release has breaking changes!

    In general I suggest to checkout the examples on how to use this new version.
    Less dependencies are needed, and if you use `default-features = false` and create your own `Transport` it's only 5 direct dependencies.
    This also makes it possible to use reqwest v0.12 again by creating your own `Transport`.

    * Totally rebuild this crate and made it more flexible
    * Added proper crate documentation
    * Bumped MSRV to v1.85.1
    * Removed dependencies not needed or switched to use others
    * Created `Transport` traits `AsyncTransport` and `BlockingTransport`
      This makes it possible to use a client of your liking and not tied to reqwest.
    * Changed all the `features`, checkout the [Usage](#Usage) section
    * `YubicoErrors` changed, removed, renamed, if you match any specific error type, validate them!
    * `Config` changed. Some `set` function might return an error or are renamed or removed,
      `set_api_hosts()` for example is renamed to `set_api_host()` and only accepts one host/url
    * Added an `ureq` example which demonstrates the usage of the `Transport` traits


---


- 0.15.0 (2026-01-18):
    * Use reqwest v0.13 or higher
    * Switched to edition 2024
    * Set MSRV to v1.85.0 which supports edition 2024 by default
    * Removed `native-tls` and `rustls-tls` and use `reqwest/default-tls` by default.<br>
      All other reqwest features are disabled in this crate it self!

    #### Highlights

    In this version I removed the specific `reqwest` features because it would limit `reqwest` to those specific features.<br>
    Also updated to `reqwest` v0.13 as a minimal version. If you need to use v0.12 of `reqwest`, just keep using v0.14 of `yubico_ng`.<br>
    I default to the `default-tls` feature via the `default` feature of the crate it self, which should be fine for most use cases.

    If you want to use anything else besides `default-tls`, use `default-features = false`, define `reqwest` as a custom dependency and define the wanted features. This way you can use `rustls-no-provider` and use any provider supported by `reqwest`.

    ```toml
    [dependencies]
    yubico_ng = { version = "1.0.0", default-features = false }
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
