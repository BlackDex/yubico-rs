use std::{error::Error as StdError, fmt};

use base64::DecodeError as Base64Error;

/// `YubicoError` enum containing possible errors which might occur during config or validation
#[derive(Debug)]
#[non_exhaustive]
pub enum YubicoError {
    /// Returned if a custom transport failed to execute the API call
    /// NOTE: depending on the transport client used, an error may contain sensitive data!
    Transport(Box<dyn StdError + Send + Sync>),
    /// Returned if the API Key is not able to be decoded
    DecodeError(Base64Error),
    /// Returned if there was an issue executing the API call
    HTTPStatus(u16),
    /// Returned if the OTP is not valid
    BadOtp,
    /// Returned if the OTP seems to be replayed
    ReplayedOtp,
    /// Returned if the signature of the API call is invalid
    BadSignature,
    /// Returned if there are parameters missing
    MissingParameter,
    /// Returned if the Client ID is not valid or does not exists
    NoSuchClient,
    /// Returned when not allowed to validate OTP tokens
    OperationNotAllowed,
    /// Returned if the API servers have issues
    BackendError,
    /// Returned if the servers were not able to meet the `SyncLevel` before timing out
    NotEnoughAnswers,
    /// Returned if the OTP/Nonce has been seen before
    ReplayedRequest,
    /// Returned if we received an unknown status
    UnknownStatus,
    /// Returned if the returned OTP mismatched
    OTPMismatch,
    /// Returned if the returned Nonce mismatched
    NonceMismatch,
    /// Returned if something went wrong during a random Nonce generation
    NonceGeneration,
    /// Returned if the generated signature mismatched
    SignatureMismatch,
    /// Returned if the generated HMAC-SHA1 was invalid
    InvalidKeyLength,
    /// Returned if the response from the API Server was invalid
    InvalidResponse,
    /// Returned if required configuration items like Client ID and API Key are missing
    MissingCredentials,
}

impl YubicoError {
    /// Wrap a custom transport's error.
    #[must_use]
    pub fn transport<E>(err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        YubicoError::Transport(Box::new(err))
    }
}

impl fmt::Display for YubicoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            YubicoError::Transport(ref err) => write!(f, "Transport error: {err}"),
            YubicoError::DecodeError(ref err) => write!(f, "Decode error: {err}"),
            YubicoError::HTTPStatus(code) => write!(f, "Error found: {code}"),
            YubicoError::BadOtp => write!(f, "The OTP is invalid format."),
            YubicoError::ReplayedOtp => write!(f, "The OTP has already been seen by the service."),
            YubicoError::BadSignature => write!(f, "The HMAC signature verification failed."),
            YubicoError::MissingParameter => write!(f, "The request lacks a parameter."),
            YubicoError::NoSuchClient => write!(f, "The request id does not exist."),
            YubicoError::OperationNotAllowed => {
                write!(f, "The request id is not allowed to verify OTPs.")
            }
            YubicoError::BackendError => write!(
                f,
                "Unexpected error in our server. Please contact us if you see this error."
            ),
            YubicoError::NotEnoughAnswers => {
                write!(f, "Server could not get requested number of syncs during before timeout")
            }
            YubicoError::ReplayedRequest => {
                write!(f, "Server has seen the OTP/Nonce combination before")
            }
            YubicoError::UnknownStatus => {
                write!(f, "Unknown status sent by the OTP validation server")
            }
            YubicoError::OTPMismatch => write!(f, "OTP mismatch, It may be an attack attempt"),
            YubicoError::NonceMismatch => write!(f, "Nonce mismatch, It may be an attack attempt"),
            YubicoError::SignatureMismatch => {
                write!(f, "Signature mismatch, It may be an attack attempt")
            }
            YubicoError::InvalidKeyLength => {
                write!(f, "Invalid key length encountered while building signature")
            }
            YubicoError::InvalidResponse => {
                write!(f, "Invalid response from the validation server")
            }
            YubicoError::NonceGeneration => write!(f, "Unable to generate random nonce"),
            YubicoError::MissingCredentials => write!(f, "Missing credentials"),
        }
    }
}

impl StdError for YubicoError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            YubicoError::Transport(ref err) => Some(&**err),
            YubicoError::DecodeError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<Base64Error> for YubicoError {
    fn from(err: Base64Error) -> YubicoError {
        YubicoError::DecodeError(err)
    }
}
