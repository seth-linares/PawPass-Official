// src/hooks/useLogout.ts

import { useState } from 'react';
import { logout } from '@/api/authApi';
import { AppError, getErrorMessage } from '@/api/apiErrors';
import { UseLogout } from '@/types/types';



const useLogout = (): UseLogout => {
    const [isLoggingOut, setIsLoggingOut] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const attemptLogout = async () => {
        try {
            setIsLoggingOut(true);
            setError(null);
            await logout();
        } catch (err) {
            console.error('❌ Logout failed:', err);
            if (err instanceof Error) {
                setError(err.message);
            } else if (typeof err === 'object' && err !== null) {
                const appError = err as AppError;
                setError(getErrorMessage(appError));
            } else {
                setError('Failed to log out');
            }
            throw err;
        } finally {
            setIsLoggingOut(false);
        }
    };

    const clearError = () => setError(null);

    return {
        isLoggingOut,
        error,
        attemptLogout,
        clearError
    };
};

export default useLogout;