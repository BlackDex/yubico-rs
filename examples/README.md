# Usage

## General

All these examples need some environment variables to work.

If you run the example it will show you which variables are missing.

You can also copy the `.env.template` file to `.env` adjust the variables where needed and run the examples.

The `.env` file is excluded from git.

## OTP (One Time Password)

```bash
cargo run --release --example otp
```

## OTP (One Time Password) with a HTTP Proxy

```bash
cargo run --release --example otp_with_proxy
```

## OTP (One Time Password) with Custom Servers

```bash
cargo run --release --example otp_custom
```

## Asynchronous OTP check

```bash
cargo run --release --example otp_async
```
