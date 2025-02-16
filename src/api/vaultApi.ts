// vaultApi.ts
import { VaultPaths, KeyDerivation, PasswordGeneratorSettings, VaultStatus } from './apiTypes';
import { invokeCommand } from './commandUtils';
/**
 * Retrieves the paths used by the vault for storage and operations.
 * This includes the main vault path, backup directory, and temporary directory.
 */
export async function getVaultPaths(): Promise<VaultPaths> {
    return invokeCommand<VaultPaths>('get_vault_paths');
}

/**
 * Initializes the password manager application with a new master password.
 * This should only be called once when setting up the vault for the first time.
 *
 * @param password - The desired master password
 * @param confirmPassword - Password confirmation to prevent typos
 * @throws Will throw an error if the vault already exists or passwords don't match
 */
export async function initializeApp(
    password: string,
    confirmPassword: string
): Promise<void> {
    return invokeCommand<void>('initialize_app', { password, confirmPassword });
}

/**
 * Retrieves the current key derivation settings used for password hashing.
 * These settings affect the security and performance of password operations.
 */
export async function getKeyDerivationSettings(): Promise<KeyDerivation> {
    return invokeCommand<KeyDerivation>('get_key_derivation_settings');;
}

/**
 * Creates a backup of the current vault state.
 * This should be called periodically or before major operations.
 */
export async function createBackup(): Promise<void> {
    return invokeCommand<void>('create_backup');
}

/**
 * Restores the vault from a backup file.
 * This will replace the current vault state with the backup's contents.
 *
 * @param backupPath - The file system path to the backup file
 */
export async function restoreFromBackup(backupPath: string): Promise<void> {
    return invokeCommand<void>('restore_from_backup', { backupPath });
}

/**
 * Checks if a vault exists at the expected location.
 * Used to determine whether to show initialization or login screen.
 * 
 * @returns Promise<boolean> True if a vault exists, false otherwise
 */
export async function checkVaultExists(): Promise<boolean> {
    return invokeCommand<boolean>('check_vault_exists');
}

/**
 * Gets the current status of the vault, including session and key hierarchy state.
 * Useful for debugging authentication and lock states.
 */
export async function getVaultStatus(): Promise<VaultStatus> {
    return invokeCommand<VaultStatus>('get_vault_status');
}

// Password Generator Functions

/**
 * Retrieves the current password generation settings.
 * These settings control the complexity and format of generated passwords.
 */
export async function getPasswordGeneratorSettings(): Promise<PasswordGeneratorSettings> {
    return invokeCommand<PasswordGeneratorSettings>('get_password_generator_settings');
}

/**
 * Updates the password generator settings with new configuration.
 *
 * @param settings - The new password generator settings to apply
 */
export async function updatePasswordGeneratorSettings(
    settings: PasswordGeneratorSettings
): Promise<void> {
    return invokeCommand<void>('update_password_generator_settings', { settings });
}

/**
 * Generates a new password using the current password generator settings.
 * @returns A string containing the generated password
 */
export async function generatePassword(): Promise<string> {
    return invokeCommand<string>('generate_password');
}

/**
 * Calculates the entropy (randomness) of passwords generated with current settings.
 * Higher entropy indicates stronger passwords.
 *
 * @returns A number representing the bits of entropy
 */
export async function calculatePasswordEntropy(): Promise<number> {
    return invokeCommand<number>('calculate_password_entropy');
}