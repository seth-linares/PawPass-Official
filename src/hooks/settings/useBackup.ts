import { useState, useCallback, useMemo } from 'react';
import { createBackup } from '@/api/vaultApi';
import { AppError } from '@/api/apiTypes';
import { BackupState } from '@/types/types';


const useBackup = () => {
    const [state, setState] = useState<BackupState>(() => {
        const savedBackupTime = localStorage.getItem('lastBackupTime');
        return {
            isCreating: false,
            error: null,
            lastBackupTime: savedBackupTime ? new Date(savedBackupTime) : null,
        };
    });

    const createNewBackup = useCallback(async () => {
        setState(prev => ({ ...prev, isCreating: true, error: null }));
        
        try {
            await createBackup();
            setState(prev => ({
                ...prev,
                isCreating: false,
                lastBackupTime: new Date(),
            }));
            localStorage.setItem('lastBackupTime', new Date().toISOString());
        } catch (error) {
            setState(prev => ({
                ...prev,
                isCreating: false,
                error: error as AppError,
            }));
        }
    }, []);

    const clearError = useCallback(() => {
        setState(prev => ({ ...prev, error: null }));
    }, []);

    return useMemo(() => ({
        isCreating: state.isCreating,
        error: state.error,
        lastBackupTime: state.lastBackupTime,
        createNewBackup,
        clearError,
    }), [
        state.isCreating,
        state.error,
        state.lastBackupTime,
        createNewBackup,
        clearError
    ]);
};

export default useBackup;