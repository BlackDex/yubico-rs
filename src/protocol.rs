use std::collections::BTreeMap;

// use base64::{Engine as _, engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD}};
use base64::{Engine as _, engine::general_purpose::STANDARD};

use crate::{Result, config::Config, error::YubicoError, sec, transport::Response};

pub(crate) struct Request<'a> {
    api_host: &'a str,
    query: String,
    otp: String,
    nonce: String,
    key: &'a [u8],
}

impl Request<'_> {
    pub(crate) fn url(&self) -> String {
        format!("{}?{}", self.api_host, self.query)
    }

    pub(crate) fn verify_transport_response(&self, response: &Response) -> Result<()> {
        if !(200..300).contains(&response.status) {
            return Err(YubicoError::HTTPStatus(response.status));
        }
        self.verify_response(&response.body)
    }

    pub(crate) fn verify_response(&self, raw_response: &str) -> Result<()> {
        let response_map = build_response_map(raw_response);

        let Some(status) = response_map.get("status") else {
            return Err(YubicoError::UnknownStatus);
        };

        // Some `BadOtp` errors might also be a `SignatureMismatch`
        // Checking for a `SignatureMismatch` is not possible, since multiple other errors
        // always generate a `SignatureMismatch` or there is not signature at all
        // The order of checking for != "OK" first and signature after is important
        if status != "OK" {
            return Err(match status.as_str() {
                "BAD_OTP" => YubicoError::BadOtp,
                "REPLAYED_OTP" => YubicoError::ReplayedOtp,
                "BAD_SIGNATURE" => YubicoError::BadSignature,
                "MISSING_PARAMETER" => YubicoError::MissingParameter,
                "NO_SUCH_CLIENT" => YubicoError::NoSuchClient,
                "OPERATION_NOT_ALLOWED" => YubicoError::OperationNotAllowed,
                "BACKEND_ERROR" => YubicoError::BackendError,
                "NOT_ENOUGH_ANSWERS" => YubicoError::NotEnoughAnswers,
                "REPLAYED_REQUEST" => YubicoError::ReplayedRequest,
                _ => YubicoError::UnknownStatus,
            });
        }

        // Signature in the response must match the one we build ourselves
        let signature = response_map
            .get("h")
            .ok_or(YubicoError::InvalidResponse)?
            .as_str();
        verify_signature(signature, &response_map, self.key)?;

        // The same for both "otp" and "nonce"
        verify_response_field(&response_map, "otp", &self.otp, YubicoError::OTPMismatch)?;
        verify_response_field(&response_map, "nonce", &self.nonce, YubicoError::NonceMismatch)
    }
}

fn verify_response_field(
    map: &BTreeMap<String, String>,
    field: &str,
    expected: &str,
    err: YubicoError,
) -> Result<()> {
    let actual = map.get(field).ok_or(YubicoError::InvalidResponse)?;
    if actual == expected { Ok(()) } else { Err(err) }
}

fn build_response_map(result: &str) -> BTreeMap<String, String> {
    let mut parameters = BTreeMap::new();
    for line in result.lines() {
        if let Some((k, v)) = line.split_once('=') {
            parameters.insert(k.to_owned(), v.to_owned());
        }
    }
    parameters
}

fn verify_signature(
    signature_response: &str,
    response_map: &BTreeMap<String, String>,
    key: &[u8],
) -> Result<()> {
    let mut query = String::new();
    for (key, value) in response_map.iter().filter(|(k, _)| k.as_str() != "h") {
        query.push_str(key);
        query.push('=');
        query.push_str(value);
        query.push('&');
    }
    query.pop(); // remove last &

    let decoded_signature = STANDARD.decode(signature_response)?;
    sec::verify_hmac(key, query.as_bytes(), &decoded_signature)
}

pub(crate) fn build_request(otp: impl Into<String>, config: &Config) -> Result<Request<'_>> {
    // A Yubikey can be configured to add line ending chars, or not
    let otp = otp.into().trim().to_owned();

    if !valid_otp_string(&otp) {
        return Err(YubicoError::BadOtp);
    }

    let nonce = generate_nonce()?;
    // The query parameters need to be in alphabetical order as specs demand
    // If this is not the case an error will be returned
    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("id", &config.client_id)
        .append_pair("nonce", &nonce)
        .append_pair("otp", &otp)
        .append_pair("sl", &config.sync_level.to_string())
        .finish();

    // Generate the hmac signature based upon the key and query
    let signature = sec::build_hmac(&config.key, query.as_bytes())?;

    // This signature needs to be Base64 Encoded according to RFC 4648
    // And added as `h` parameter to the query
    let query = form_urlencoded::Serializer::new(query)
        .append_pair("h", &STANDARD.encode(signature.into_bytes()))
        .finish();

    Ok(Request {
        api_host: &config.api_host,
        query,
        otp,
        nonce,
        key: &config.key,
    })
}

// Recommendation is that clients only check that the input consists of 32-48 printable characters
// If the keyboard layout is different the characters might not be ModHex, but still the same keypress
fn valid_otp_string(otp: &str) -> bool {
    (32..=48).contains(&otp.len()) && otp.bytes().all(|b| b.is_ascii_graphic())
}

// Generate a random string based upon the 62 allowed characters
// This function has a modulo bias since 62 is not dividable by 256 which means some chars might be selected more often
// The risk of this being an issue is very small.
fn generate_nonce() -> Result<String> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let mut buf = [0u8; 40];
    getrandom::fill(&mut buf).map_err(|_| YubicoError::NonceGeneration)?;
    Ok(buf
        .iter()
        .map(|b| CHARSET[(*b as usize) % CHARSET.len()] as char)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otp_length() {
        // To short
        assert!(!valid_otp_string(&"v".repeat(31)));
        // Valid min-char
        assert!(valid_otp_string(&"v".repeat(32)));
        // Valid max-char
        assert!(valid_otp_string(&"v".repeat(48)));
        // To large
        assert!(!valid_otp_string(&"v".repeat(49)));
        // No printable characters
        assert!(!valid_otp_string(&format!("{v} {v}", v = "v".repeat(20))));
    }
}
