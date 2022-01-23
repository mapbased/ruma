use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// A wrapper around `B` (usually `Vec<u8>`) that (de)serializes from / to a base64 string.
///
/// The base64 character set (and miscellaneous other encoding / decoding options) can be customized
/// through the generic parameter `C`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Base64<C = Standard, B = Vec<u8>> {
    bytes: B,
}

pub trait Base64Config {
    const CONF: base64::Config;
}

/// Standard base64 character set without padding.
///
/// Allows trailing bits in decoding for maximum compatibility.
#[non_exhaustive]
pub struct Standard;

impl Base64Config for Standard {
    // See https://github.com/matrix-org/matrix-doc/issues/3211
    const CONF: base64::Config = base64::STANDARD_NO_PAD.decode_allow_trailing_bits(true);
}

/// Url-safe base64 character set without padding.
///
/// Allows trailing bits in decoding for maximum compatibility.
#[non_exhaustive]
pub struct UrlSafe;

impl Base64Config for UrlSafe {
    const CONF: base64::Config = base64::URL_SAFE_NO_PAD.decode_allow_trailing_bits(true);
}

impl<B: AsRef<[u8]>> Base64<B> {
    /// Create a `Base64` instance from raw bytes, to be base64-encoded in serialialization.
    pub fn new(bytes: B) -> Self {
        Self { bytes }
    }

    /// Get a reference to the raw bytes held by this `Base64` instance.
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    /// Encode the bytes contained in this `Base64` instance to unpadded base64.
    pub fn encode(&self) -> String {
        base64::encode_config(&self.bytes, BASE64_CONFIG)
    }
}

impl<B> Base64<B> {
    /// Get the raw bytes held by this `Base64` instance.
    pub fn into_inner(self) -> B {
        self.bytes
    }
}

impl Base64 {
    /// Create a `Base64` instance containing an empty `Vec<u8>`.
    pub fn empty() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Parse some base64-encoded data to create a `Base64` instance.
    pub fn parse(encoded: impl AsRef<[u8]>) -> Result<Self, base64::DecodeError> {
        base64::decode_config(encoded, BASE64_CONFIG).map(Self::new)
    }
}

impl<B: AsRef<[u8]>> fmt::Debug for Base64<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.encode().fmt(f)
    }
}

impl<B: AsRef<[u8]>> fmt::Display for Base64<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.encode().fmt(f)
    }
}

impl<'de> Deserialize<'de> for Base64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = crate::deserialize_cow_str(deserializer)?;
        Self::parse(&*encoded).map_err(de::Error::custom)
    }
}

impl<B: AsRef<[u8]>> Serialize for Base64<B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.encode())
    }
}