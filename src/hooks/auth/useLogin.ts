// src/hooks/useLogin.ts

import { useState } from 'react';
import { login } from '@/api/authApi';
import { AppError, getErrorMessage } from '@/api/apiErrors';
import { UseLogin } from '@/types/types';



const useLogin = (): UseLogin => {
    const [isLoggingIn, setIsLoggingIn] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const attemptLogin = async (password: string) => {
        console.log('🔑 Attempting login...');
        try {
            setIsLoggingIn(true);
            setError(null);
            await login(password);
            console.log('✅ Login successful');
        } catch (err) {
            console.error('❌ Login failed:', err);
            if (err instanceof Error) {
                setError(err.message);
            } else if (typeof err === 'object' && err !== null) {
                const appError = err as AppError;
                setError(getErrorMessage(appError));
            } else {
                setError('Failed to log in');
            }
            throw err;
        } finally {
            setIsLoggingIn(false);
        }
    };

    const clearError = () => setError(null);

    return {
        isLoggingIn,
        error,
        attemptLogin,
        clearError
    };
};

export default useLogin;