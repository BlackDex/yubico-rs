use hmac::{Hmac, KeyInit, Mac, digest::CtOutput};
use sha1::Sha1;

use crate::error::YubicoError;

type HmacSha1 = Hmac<Sha1>;

pub(crate) fn build_hmac(key: &[u8], input: &[u8]) -> Result<CtOutput<HmacSha1>, YubicoError> {
    let hmac = generate_hmac(key, input)?;
    Ok(hmac.finalize())
}

pub(crate) fn verify_hmac(key: &[u8], input: &[u8], expected: &[u8]) -> Result<(), YubicoError> {
    let hmac = generate_hmac(key, input)?;
    hmac.verify_slice(expected)
        .map_err(|_| YubicoError::SignatureMismatch)
}

//  Apply the HMAC-SHA-1 algorithm on the line as an octet string using the API key as key
fn generate_hmac(key: &[u8], input: &[u8]) -> Result<HmacSha1, YubicoError> {
    let Ok(mut hmac) = HmacSha1::new_from_slice(key) else {
        return Err(YubicoError::InvalidKeyLength);
    };

    hmac.update(input);
    Ok(hmac)
}
