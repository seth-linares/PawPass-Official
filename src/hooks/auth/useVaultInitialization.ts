// src/hooks/useVaultInitialization.ts

import { useState } from 'react';
import { initializeApp } from '@/api/vaultApi';
import { useVaultContext } from '@/contexts/VaultContext';
import { useAuthContext } from '@/contexts/AuthContext';
import { useNavigate } from 'react-router-dom';
import { AppError, getErrorMessage } from '@/api/apiErrors';
import { UseVaultInitialization } from '@/types/types';



const useVaultInitialization = (): UseVaultInitialization => {
    const [isInitializing, setIsInitializing] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [showPassword, setShowPassword] = useState(false);
    const [showConfirmPassword, setShowConfirmPassword] = useState(false);
    const { recheckVault } = useVaultContext();
    const { setAuthenticated } = useAuthContext();
    const navigate = useNavigate();

    const handleInitialization = async (password: string, confirmPassword: string) => {
        try {
            console.log('🚀 Starting vault initialization submission...');
            setIsInitializing(true);
            setError(null);
            
            await initializeApp(password, confirmPassword);
            await recheckVault();
            
            console.log('✅ Vault initialized, setting authenticated state...');
            setAuthenticated(true);
            
            console.log('🔄 Attempting navigation to /vault...');
            navigate('/vault');
            
            console.log('✨ Initialization flow complete');
        } catch (err) {
            console.error('❌ Initialization error:', err);
            if (err instanceof Error) {
                setError(err.message);
            } else if (typeof err === 'object' && err !== null) {
                const appError = err as AppError;
                setError(getErrorMessage(appError));
            } else {
                setError('Failed to initialize vault');
            }
            throw err;
        } finally {
            setIsInitializing(false);
        }
    };

    const clearError = () => setError(null);

    const togglePasswordVisibility = (field: 'password' | 'confirmPassword') => {
        if (field === 'password') {
            setShowPassword(!showPassword);
        } else {
            setShowConfirmPassword(!showConfirmPassword);
        }
    };

    return {
        isInitializing,
        error,
        handleInitialization,
        clearError,
        showPassword,
        showConfirmPassword,
        togglePasswordVisibility
    };
};

export default useVaultInitialization;