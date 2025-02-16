// #[cfg(test)]
// mod tests {
//     use serde_cbor;
//     use crate::crypto::EncryptedData;
//     use crate::error::crypto_error::CryptoError;
//     // For serialization tests
//
//     const VALID_CIPHERTEXT: [u8; 32] = [42; 32];
//     const VALID_NONCE: [u8; 12] = [1; 12];
//     const VALID_TAG: [u8; 16] = [2; 16];
//
//     fn create_valid_data() -> EncryptedData {
//         EncryptedData::new(
//             VALID_CIPHERTEXT.to_vec(),
//             VALID_NONCE,
//             VALID_TAG,
//         ).unwrap()
//     }
//
//     #[test]
//     fn test_valid_creation() {
//         let result = EncryptedData::new(
//             VALID_CIPHERTEXT.to_vec(),
//             VALID_NONCE,
//             VALID_TAG,
//         );
//         assert!(result.is_ok());
//     }
//
//     #[test]
//     fn test_empty_ciphertext() {
//         let result = EncryptedData::new(
//             vec![],  // Empty ciphertext
//             VALID_NONCE,
//             VALID_TAG,
//         );
//         assert!(matches!(result.unwrap_err(), CryptoError::InvalidCiphertext));
//     }
//
//     #[test]
//     fn test_getters() {
//         let data = create_valid_data();
//
//         assert_eq!(data.ciphertext(), &VALID_CIPHERTEXT);
//         assert_eq!(data.nonce(), &VALID_NONCE);
//         assert_eq!(data.tag(), &VALID_TAG);
//     }
//
//     #[test]
//     fn test_debug_output() {
//         let data = create_valid_data();
//         let debug_string = format!("{:?}", data);
//
//         // Verify debug output contains our struct name
//         assert!(debug_string.contains("EncryptedData"));
//         // Verify it shows vector and array contents
//         assert!(debug_string.contains("ciphertext"));
//         assert!(debug_string.contains("nonce"));
//         assert!(debug_string.contains("tag"));
//     }
//
//     #[test]
//     fn test_clone() {
//         let original = create_valid_data();
//         let cloned = original.clone();
//
//         assert_eq!(original.ciphertext(), cloned.ciphertext());
//         assert_eq!(original.nonce(), cloned.nonce());
//         assert_eq!(original.tag(), cloned.tag());
//     }
//
//     #[test]
//     fn test_serialization_roundtrip() {
//         let original = create_valid_data();
//
//         // Serialize to CBOR
//         let serialized = serde_cbor::to_vec(&original).unwrap();
//
//         // Deserialize back
//         let deserialized: EncryptedData = serde_cbor::from_slice(&serialized).unwrap();
//
//         // Compare all fields
//         assert_eq!(original.ciphertext(), deserialized.ciphertext());
//         assert_eq!(original.nonce(), deserialized.nonce());
//         assert_eq!(original.tag(), deserialized.tag());
//     }
//
//     #[test]
//     fn test_zero() {
//         let zero = EncryptedData::zero();
//
//         // Check that all fields are zeroed
//         assert!(zero.ciphertext().iter().all(|&x| x == 0));
//         assert!(zero.nonce().iter().all(|&x| x == 0));
//         assert!(zero.tag().iter().all(|&x| x == 0));
//     }
//
//     #[test]
//     fn test_validation() {
//         let data = create_valid_data();
//         assert!(data.validate().is_ok());
//     }
//
//     // Test different ciphertext lengths
//     #[test]
//     fn test_various_ciphertext_lengths() {
//         // Test with small but valid ciphertext
//         let result = EncryptedData::new(
//             vec![1],
//             VALID_NONCE,
//             VALID_TAG,
//         );
//         assert!(result.is_ok());
//
//         // Test with larger ciphertext
//         let result = EncryptedData::new(
//             vec![1; 1000],
//             VALID_NONCE,
//             VALID_TAG,
//         );
//         assert!(result.is_ok());
//     }
// }