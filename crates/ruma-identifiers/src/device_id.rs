#[cfg(feature = "rand")]
use crate::generate_localpart;

/// A Matrix key ID.
///
/// Device identifiers in Matrix are completely opaque character sequences. This type is
/// provided simply for its semantic value.
#[repr(transparent)]
pub struct DeviceId(str);

opaque_identifier!(DeviceId);

impl DeviceId {
    /// Generates a random `DeviceId`, suitable for assignment to a new device.
    #[cfg(feature = "rand")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
    pub fn new() -> Box<Self> {
        Self::from_owned(generate_localpart(8))
    }
}

#[cfg(all(test, feature = "rand"))]
mod tests {
    use super::DeviceId;

    #[test]
    fn generate_device_id() {
        assert_eq!(DeviceId::new().as_str().len(), 8);
    }
}
