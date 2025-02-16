// #[cfg(test)]
// mod tests {
//     use zeroize::Zeroize;
//     use crate::crypto::SecureMemory;
//
//     // A test type that allows us to detect when zeroize is called.
//     struct TestData(Vec<u8>);
//
//     impl Zeroize for TestData {
//         fn zeroize(&mut self) {
//             // Overwrite the data with zeros.
//             for byte in &mut self.0 {
//                 *byte = 0;
//             }
//             // Indicate that zeroize was called (used in some tests).
//             ZEROIZE_CALLED.with(|flag| *flag.borrow_mut() = true);
//         }
//     }
//
//     use std::cell::RefCell;
//     thread_local! {
//         static ZEROIZE_CALLED: RefCell<bool> = RefCell::new(false);
//     }
//
//     #[test]
//     fn test_secure_memory_new_and_as_ref() {
//         // Test that SecureMemory::new correctly initializes the data
//         // and that as_ref provides read access.
//
//         let data = vec![1, 2, 3, 4];
//         let _secure_mem = SecureMemory::new(data.clone());
//         assert_eq!(_secure_mem.as_ref(), &data);
//     }
//
//     #[test]
//     fn test_secure_memory_as_mut() {
//         // Test that as_mut provides mutable access to the data.
//
//         let mut secure_mem = SecureMemory::new(vec![1, 2, 3, 4]);
//         secure_mem.as_mut().push(5);
//         assert_eq!(secure_mem.as_ref(), &vec![1, 2, 3, 4, 5]);
//     }
//
//     #[test]
//     fn test_zeroize_on_drop() {
//         // Test that data is zeroized when SecureMemory is dropped.
//
//         ZEROIZE_CALLED.with(|flag| *flag.borrow_mut() = false);
//
//         {
//             let _secure_mem = SecureMemory::new(TestData(vec![1, 2, 3, 4]));
//             // _secure_mem goes out of scope here, triggering Drop.
//         }
//
//         // Verify that zeroize was called during drop.
//         ZEROIZE_CALLED.with(|flag| {
//             assert!(*flag.borrow(), "Zeroize was not called on drop");
//         });
//     }
//
//     #[test]
//     fn test_zeroize_in_into_inner() {
//         // Test that data is not zeroized when using into_inner
//         ZEROIZE_CALLED.with(|flag| *flag.borrow_mut() = false);
//
//         let secure_mem = SecureMemory::new(TestData(vec![1, 2, 3, 4]));
//         let mut data = unsafe { secure_mem.into_inner() };
//
//         // Verify that zeroize was NOT called before returning the data
//         ZEROIZE_CALLED.with(|flag| {
//             assert!(!*flag.borrow(), "Zeroize should not be called in into_inner");
//         });
//
//         // Ensure that we received the correct data
//         assert_eq!(data.0, vec![1, 2, 3, 4]);
//
//         // The caller is responsible for zeroizing the data
//         data.zeroize();
//         // Optionally check that zeroize works as expected
//         assert_eq!(data.0, vec![0, 0, 0, 0]);
//     }
//
//     #[test]
//     fn test_into_inner_safety() {
//         // Test that using into_inner does not lead to memory safety issues
//         ZEROIZE_CALLED.with(|flag| *flag.borrow_mut() = false);
//
//         let secure_mem = SecureMemory::new(TestData(vec![1, 2, 3, 4]));
//         let mut data = unsafe { secure_mem.into_inner() };
//
//         // Verify that zeroize was NOT called
//         ZEROIZE_CALLED.with(|flag| {
//             assert!(!*flag.borrow(), "Zeroize should not be called in into_inner");
//         });
//
//         // Use the data to ensure it's still valid
//         assert_eq!(data.0, vec![1, 2, 3, 4]);
//
//         // The caller should zeroize the data after use
//         data.zeroize();
//         // Optionally check that zeroize works as expected
//         assert_eq!(data.0, vec![0, 0, 0, 0]);
//     }
//
//     #[test]
//     fn test_secure_memory_debug() {
//         // Test that the Debug trait does not expose sensitive data.
//
//         let secure_mem = SecureMemory::new(vec![1, 2, 3, 4]);
//         let debug_output = format!("{:?}", secure_mem);
//
//         assert!(
//             debug_output.contains("[REDACTED]"),
//             "Debug output should contain [REDACTED]"
//         );
//         assert!(
//             !debug_output.contains("1") && !debug_output.contains("2"),
//             "Debug output should not contain actual data"
//         );
//     }
//
//     #[test]
//     fn test_clone_trait_not_implemented() {
//         // Ensure that SecureMemory does not implement Clone.
//
//         let _secure_mem = SecureMemory::new(vec![1, 2, 3, 4]);
//
//         // Attempting to clone should result in a compile-time error.
//         // Uncommenting the next line should fail to compile.
//         // let _clone = secure_mem.clone();
//
//         // Since we cannot test compile-time errors directly in code,
//         // this test serves as documentation for the intended behavior.
//     }
//
//
//     #[test]
//     fn test_with_different_types() {
//         // Test SecureMemory with different types that implement Zeroize.
//
//         // Using a simple u32 type.
//         {
//             let secure_mem = SecureMemory::new(42u32);
//             assert_eq!(*secure_mem.as_ref(), 42);
//         }
//
//         // Using a string.
//         {
//
//             let secure_mem = SecureMemory::new(String::from("secret"));
//             assert_eq!(secure_mem.as_ref(), "secret");
//         }
//
//         // Using a custom struct.
//         {
//             #[derive(Debug)]
//             struct CustomData {
//                 field1: u8,
//                 field2: Vec<u8>,
//             }
//
//             impl Zeroize for CustomData {
//                 fn zeroize(&mut self) {
//                     self.field1.zeroize();
//                     self.field2.zeroize();
//                 }
//             }
//
//             let data = CustomData {
//                 field1: 10,
//                 field2: vec![1, 2, 3],
//             };
//
//             let secure_mem = SecureMemory::new(data);
//             assert_eq!(secure_mem.as_ref().field1, 10);
//             assert_eq!(secure_mem.as_ref().field2, vec![1, 2, 3]);
//         }
//     }
//
//     // Advanced test for zeroization on panic (optional).
//     #[test]
//     fn test_zeroize_on_panic() {
//         // Ensure that data is zeroized even if a panic occurs.
//
//         ZEROIZE_CALLED.with(|flag| *flag.borrow_mut() = false);
//
//         let result = std::panic::catch_unwind(|| {
//             let _secure_mem = SecureMemory::new(TestData(vec![1, 2, 3, 4]));
//             panic!("Simulated panic");
//             // _secure_mem should be dropped here, zeroizing the data.
//         });
//
//         assert!(result.is_err(), "Expected a panic");
//
//         // Verify that zeroize was called during drop.
//         ZEROIZE_CALLED.with(|flag| {
//             assert!(*flag.borrow(), "Zeroize was not called during panic");
//         });
//     }
//
//     #[test]
//     fn test_empty_data() {
//         // Tests handling of empty data
//         let empty: Vec<u8> = vec![];
//         let secure_mem = SecureMemory::new(empty);
//         assert!(secure_mem.as_ref().is_empty());
//     }
//
//     #[test]
//     fn test_large_data() {
//         // Tests handling of large data sets
//         let large_data = vec![0u8; 1_000_000]; // 1 million bytes
//         let secure_mem = SecureMemory::new(large_data.clone());
//         assert_eq!(secure_mem.as_ref().len(), 1_000_000);
//         assert_eq!(secure_mem.as_ref(), &large_data);
//     }
//
//     #[test]
//     fn test_array_type() {
//         // Tests SecureMemory with fixed-size arrays
//         let arr = [1u8; 32]; // Common size for cryptographic keys
//         let secure_mem = SecureMemory::new(arr);
//         assert_eq!(secure_mem.as_ref(), &arr);
//     }
//
//     #[test]
//     fn test_nested_secure_memory() {
//         // Tests nesting of SecureMemory instances
//         let inner = SecureMemory::new(vec![1, 2, 3]);
//         let outer = SecureMemory::new(inner);
//         assert_eq!(outer.as_ref().as_ref(), &vec![1, 2, 3]);
//     }
//
//     #[test]
//     fn test_concurrent_access() {
//         // Tests thread-safe read-only access
//         use std::sync::Arc;
//         use std::thread;
//
//         let original_data = vec![1, 2, 3, 4, 5];
//         let secure_mem = Arc::new(SecureMemory::new(original_data.clone()));
//
//         let mut handles = vec![];
//
//         for _ in 0..5 {
//             let secure_clone = Arc::clone(&secure_mem);
//             let comparison_data = original_data.clone();
//             handles.push(thread::spawn(move || {
//                 assert_eq!(secure_clone.as_ref().as_ref(), &comparison_data);
//             }));
//         }
//
//         for handle in handles {
//             handle.join().expect("Thread panicked");
//         }
//     }
//
//     #[test]
//     fn test_custom_drop_interaction() {
//         // Tests that both custom Drop and Zeroize are called
//         use std::sync::Arc;
//         use std::sync::atomic::{AtomicBool, Ordering};
//
//         struct CustomDrop {
//             data: Vec<u8>,
//             drop_occurred: Arc<AtomicBool>,
//             zero_occurred: Arc<AtomicBool>,
//         }
//
//         impl Drop for CustomDrop {
//             fn drop(&mut self) {
//                 self.drop_occurred.store(true, Ordering::SeqCst);
//             }
//         }
//
//         impl Zeroize for CustomDrop {
//             fn zeroize(&mut self) {
//                 self.data.zeroize();
//                 self.zero_occurred.store(true, Ordering::SeqCst);
//             }
//         }
//
//         let drop_occurred = Arc::new(AtomicBool::new(false));
//         let zero_occurred = Arc::new(AtomicBool::new(false));
//
//         {
//             let custom = CustomDrop {
//                 data: vec![1, 2, 3],
//                 drop_occurred: Arc::clone(&drop_occurred),
//                 zero_occurred: Arc::clone(&zero_occurred),
//             };
//             let _secure_mem = SecureMemory::new(custom);
//         }
//
//         assert!(drop_occurred.load(Ordering::SeqCst), "Custom drop didn't occur");
//         assert!(zero_occurred.load(Ordering::SeqCst), "Zeroize didn't occur");
//     }
//
//     #[test]
//     fn test_zeroize_on_panic_two() {
//         // Ensures data is zeroized even if a panic occurs
//         use std::panic::{catch_unwind, AssertUnwindSafe};
//         use std::sync::Arc;
//         use std::sync::atomic::{AtomicBool, Ordering};
//
//         struct PanicData {
//             data: Vec<u8>,
//             zeroized: Arc<AtomicBool>,
//         }
//
//         impl Zeroize for PanicData {
//             fn zeroize(&mut self) {
//                 self.data.zeroize();
//                 self.zeroized.store(true, Ordering::SeqCst);
//             }
//         }
//
//         let zeroized = Arc::new(AtomicBool::new(false));
//
//         let result = catch_unwind(AssertUnwindSafe(|| {
//             let _secure_mem = SecureMemory::new(PanicData {
//                 data: vec![1, 2, 3, 4],
//                 zeroized: Arc::clone(&zeroized),
//             });
//             panic!("Simulated panic");
//         }));
//
//         assert!(result.is_err(), "Expected a panic");
//         assert!(zeroized.load(Ordering::SeqCst), "Data was not zeroized on panic");
//     }
//
// }
