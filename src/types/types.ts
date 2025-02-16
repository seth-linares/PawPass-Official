import { AppError, KeyDerivation } from "@/api/apiTypes";
import { ValidationError } from "@/utils/keyDerivationValidation";

// Authentication Interfaces
export interface UseLogin {
    isLoggingIn: boolean;
    error: string | null;
    attemptLogin: (password: string) => Promise<void>;
    clearError: () => void;
}

export interface UseLogout {
    isLoggingOut: boolean;
    error: string | null;
    attemptLogout: () => Promise<void>;
    clearError: () => void;
}

export interface UseVaultInitialization {
    isInitializing: boolean;
    error: string | null;
    handleInitialization: (password: string, confirmPassword: string) => Promise<void>;
    clearError: () => void;
    showPassword: boolean;
    showConfirmPassword: boolean;
    togglePasswordVisibility: (field: 'password' | 'confirmPassword') => void;
}

// Vault Management Interfaces
export interface VaultCheckState {
    isChecking: boolean;
    vaultExists: boolean;
    error: Error | null;
    recheckVault: () => Promise<void>;
}

export interface BackupState {
    isCreating: boolean;
    error: AppError | null;
    lastBackupTime: Date | null;
}

export interface RestoreBackupState {
    isRestoring: boolean;
    error: AppError | null;
    success: boolean;
}

export interface RestoreBackupReturn {
    filePath: string | undefined;
    selectBackupFile: (path: string) => void;
    restoreBackup: () => Promise<void>;
    resetState: () => void;
    isRestoring: boolean;
    error: AppError | null;
    success: boolean;
}

// Security and Validation Interfaces
export interface ChangeMasterPasswordState {
    isChanging: boolean;
    error: AppError | null;
    success: boolean;
    oldPassword: string;
    newPassword: string;
    confirmPassword: string;
    isOpen: boolean;
}

export interface ChangeKeyDerivationState {
    isUpdating: boolean;
    isLoading: boolean;
    error: AppError | null;
    validationErrors: ValidationError[];
    success: boolean;
}

export interface KeyDerivationParams {
    masterPassword: string;
    keyDerivation: KeyDerivation;
}

export interface ValidationResult {
    isValid: boolean;
    error: string | null;
}

export interface StrengthValidation {
    getStrengthColor: (entropy: number | null) => string;
    getStrengthLabel: (entropy: number | null) => string;
}

// UI Utility Interfaces
export interface UseComboBoxProps<T extends readonly string[]> {
    items: T;
    defaultValue: string;
    onValueChange: (value: T[number]) => void;
    capitalizeItems?: boolean;
}

// Combo Box Interface
export interface ComboBoxProps<T extends readonly string[]> {
    items: T;
    defaultValue: string;
    onValueChange: (value: T[number]) => void;
    label?: string;
    showLabel?: boolean;
    description?: string;
    placeholder?: string;
    searchPlaceholder?: string;
    emptyMessage?: string;
    onAddItem?: (newItem: string) => void;
    capitalizeItems?: boolean;
    className?: string;
}

// Loading Spinner Interface
export interface LoadingSpinnerProps {
    size?: 'xs' | 'sm' | 'md' | 'lg';
    label?: string;
    className?: string;
    inline?: boolean;
}


