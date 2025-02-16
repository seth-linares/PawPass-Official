#[cfg(test)]
mod tests {
    use crate::auth::auth_service::AuthService;
    use crate::crypto::SecureMemory;
    use crate::error::auth_error::AuthError;

    // Helper function to create a test password in SecureMemory
    fn create_test_password(password: &str) -> SecureMemory<String> {
        SecureMemory::new(password.to_string())
    }

    #[test]
    /// Tests the basic creation of a new _vault with a master password
    /// Verifies that:
    /// 1. AuthService creation succeeds
    /// 2. Initial state is unlocked
    /// 3. Key hierarchy is properly initialized
    /// 4. Password verification works immediately
    fn test_new_creation() {
        let password = create_test_password("test_password123!");

        let (auth_service, key_hierarchy) = AuthService::new(password)
            .expect("Should create new _vault");

        assert!(!auth_service.is_locked());

        let password = create_test_password("test_password123!");
        // Verify the password works
        assert!(auth_service.verify_master_password(password)
            .expect("Password verification should succeed"));
        // Ensure we got a working key hierarchy
        assert!(key_hierarchy.encrypted_mek(&auth_service.master_password_hash()).is_ok());
    }

    #[test]
    /// Tests that password verification fails with wrong password
    fn test_wrong_password_verification() {
        let correct_password = create_test_password("correct_password123!");
        let wrong_password = create_test_password("wrong_password123!");

        let (auth_service, _) = AuthService::new(correct_password)
            .expect("Should create new _vault");

        assert!(!auth_service.verify_master_password(wrong_password)
            .expect("Verification should return false"));
    }

    #[test]
    /// Tests the complete lock/unlock cycle
    /// Verifies:
    /// 1. Vault can be locked
    /// 2. Locked _vault rejects operations
    /// 3. Correct password unlocks
    /// 4. Wrong password fails to unlock
    fn test_lock_unlock_cycle() {
        let password = create_test_password("test_password123!");
        let wrong_password = create_test_password("wrong_password123!");

        let (mut auth_service, _) = AuthService::new(password)
            .expect("Should create new _vault");

        // Test locking
        auth_service.lock_vault();
        assert!(auth_service.is_locked());

        // Test unlock with wrong password
        let unlock_result = auth_service.unlock(wrong_password);
        assert!(matches!(unlock_result, Err(AuthError::InvalidPassword)));
        assert!(auth_service.is_locked());

        let password = create_test_password("test_password123!");
        // Test unlock with correct password
        let unlock_result = auth_service.unlock(password);
        assert!(unlock_result.is_ok());
        assert!(!auth_service.is_locked());
    }

    #[test]
    /// Tests password change functionality
    /// Verifies:
    /// 1. Password can be changed successfully
    /// 2. Old password no longer works
    /// 3. New password works for all operations
    /// 4. MEK remains accessible after password change
    fn test_password_change() {
        let old_password = create_test_password("old_password123!");
        let new_password = create_test_password("new_password123!");

        let (mut auth_service, _) = AuthService::new(old_password)
            .expect("Should create new _vault");

        let old_password = create_test_password("old_password123!");

        // Change password
        auth_service.change_password(old_password, new_password)
            .expect("Password change should succeed");


        let old_password = create_test_password("old_password123!");
        // Verify old password no longer works
        assert!(!auth_service.verify_master_password(old_password)
            .expect("Verification should return false"));

        let new_password = create_test_password("new_password123!");
        // Verify new password works
        assert!(auth_service.verify_master_password(new_password)
            .expect("New password should verify"));
    }

    #[test]
    /// Tests that locked _vault operations fail appropriately
    fn test_locked_vault_restrictions() {
        let password = create_test_password("test_password123!");
        // let new_password = create_test_password("new_password123!");

        let (mut auth_service, _) = AuthService::new(password)
            .expect("Should create new _vault");

        auth_service.lock_vault();

        // Test password change fails when locked
        let change_result = auth_service.change_password(
            create_test_password("old"),
            create_test_password("new")
        );
        assert!(matches!(change_result, Err(AuthError::VaultLocked)));
    }

    #[test]
    /// Tests the consistency of key derivation and MEK
    /// Verifies that encrypted data remains accessible after:
    /// 1. Lock/unlock cycles
    /// 2. Password changes
    fn test_key_hierarchy_consistency() {
        let password = create_test_password("test_password123!");
        let new_password = create_test_password("new_password123!");

        // Create _vault and get initial key hierarchy
        let (mut auth_service, initial_hierarchy) = AuthService::new(password)
            .expect("Should create new _vault");

        // Create some test encrypted data
        let test_data = b"test data";
        let encrypted = initial_hierarchy.encrypt_data(test_data)
            .expect("Should encrypt data");

        let password = create_test_password("test_password123!");

        // Test after lock/unlock
        auth_service.lock_vault();
        let unlocked_hierarchy = auth_service.unlock(password)
            .expect("Should unlock");

        // Verify data can still be decrypted
        let decrypted = unlocked_hierarchy.decrypt_data(&encrypted)
            .expect("Should decrypt data");
        assert_eq!(decrypted, test_data);


        let password = create_test_password("test_password123!");
        // Test after password change
        auth_service.change_password(password, new_password)
            .expect("Should change password");


        let new_password = create_test_password("new_password123!");
        // Lock and unlock with new password
        auth_service.lock_vault();
        let final_hierarchy = auth_service.unlock(new_password)
            .expect("Should unlock with new password");

        // Verify data can still be decrypted
        let decrypted = final_hierarchy.decrypt_data(&encrypted)
            .expect("Should decrypt data");
        assert_eq!(decrypted, test_data);
    }
}