import { useCallback, useEffect, useMemo, useState } from "react";
import { updateKeyDerivation } from "@/api/authApi";
import { getKeyDerivationSettings } from "@/api/vaultApi";
import { AppError, KeyDerivation } from "@/api/apiTypes";
import { validateKeyDerivation, KEY_DERIVATION_CONSTANTS } from "@/utils/keyDerivationValidation";
import { ChangeKeyDerivationState } from "@/types/types";



/**
 * Hook for managing key derivation parameter updates
 * Handles the update process, loading states, and error management
 */
const useChangeKeyDerivation = () => {
    const [state, setState] = useState<ChangeKeyDerivationState>({
        isUpdating: false,
        isLoading: true, // Start with loading true
        error: null,
        validationErrors: [],
        success: false,
    });

    const [formData, setFormData] = useState<KeyDerivation>({
        memoryCost: 0,  // Start with zero values
        timeCost: 0,
        parallelism: 0,
    });
    const [initialized, setInitialized] = useState(false);
    const [password, setPassword] = useState("");
    const [_open, _setOpen] = useState(false);

    const resetForm = useCallback(() => {
        setState(prev => ({
            ...prev,
            isLoading: true,  // Reset to loading state
            isUpdating: false,
            error: null,
            validationErrors: [],
            success: false
        }));
        setInitialized(false);  // Reset initialized state
        setPassword("");
    }, []);

    const setOpen = useCallback((newOpen: boolean) => {
        if (!newOpen && _open) {
            resetForm();
        }
        _setOpen(newOpen);
    }, [_open, resetForm]);

    // Modified effect to properly handle loading and setting initial values
    useEffect(() => {
        if (_open && !initialized) {
            const loadSettings = async () => {
                setState(prev => ({ ...prev, isLoading: true }));
                try {
                    const settings = await getKeyDerivationSettings();
                    console.log('Retrieved settings:', settings);
                
                    setFormData(settings);
                    setInitialized(true);
                    setState(prev => ({ ...prev, isLoading: false }));
                } catch (error) {
                    console.error('Error loading settings:', error);
                    setFormData({
                        memoryCost: KEY_DERIVATION_CONSTANTS.RECOMMENDED_MEMORY_COST,
                        timeCost: KEY_DERIVATION_CONSTANTS.RECOMMENDED_TIME_COST,
                        parallelism: 1,
                    });
                    setState(prev => ({ 
                        ...prev,
                        isLoading: false,
                        error: error as AppError 
                    }));
                    setInitialized(true);
                }
            };

            loadSettings();
        }
    }, [_open, initialized]);

    const updateFormField = useCallback((field: keyof KeyDerivation, value: number) => {
        console.log('Updating form field:', field, value);
        setFormData(prev => ({
            ...prev,
            [field]: value
        }));
    }, []);

    const updateParams = useCallback(async (masterPassword: string) => {
        const validationErrors = validateKeyDerivation(formData);
        if (validationErrors.length > 0) {
            setState(prev => ({ ...prev, validationErrors, error: null }));
            return false;
        }

        setState(prev => ({ 
            ...prev,
            isUpdating: true, 
            error: null, 
            validationErrors: [], 
            success: false 
        }));

        try {
            console.log('Updating params with formData:', formData);
            await updateKeyDerivation(masterPassword, formData);
            setState(prev => ({ 
                ...prev,
                isUpdating: false, 
                error: null, 
                validationErrors: [], 
                success: true 
            }));
            return true;
        } catch (error) {
            setState(prev => ({ 
                ...prev,
                isUpdating: false, 
                error: error as AppError, 
                validationErrors: [], 
                success: false 
            }));
            return false;
        }
    }, [formData]);

    return useMemo(() => ({
        isUpdating: state.isUpdating,
        isLoading: state.isLoading || !initialized,
        error: state.error,
        validationErrors: state.validationErrors,
        success: state.success,
        formData,
        password,
        open: _open,
        initialized,
        updateParams,
        updateFormField,
        resetForm,
        setPassword,
        setOpen,
    }), [
        state.isUpdating,
        state.isLoading,
        state.error,
        state.validationErrors,
        state.success,
        formData,
        password,
        _open,
        updateParams,
        updateFormField,
        resetForm,
        initialized,
    ]);
};

export default useChangeKeyDerivation;