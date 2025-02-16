// src/hooks/usePasswordGenerator.ts

import { useState, useCallback, useEffect, useMemo } from 'react';
import { useAnimationControls } from 'framer-motion';
import { 
    getPasswordGeneratorSettings, 
    updatePasswordGeneratorSettings,
    generatePassword as generatePasswordApi,
    calculatePasswordEntropy as calculateEntropyApi 
} from '@/api/vaultApi';
import { PasswordGeneratorSettings, AppError } from '@/api/apiTypes';
import { useToast } from '@/hooks/use-toast';
import { UsePasswordGeneratorReturn, PasswordGeneratorState } from "@/types/entry.types";
import { initialPasswordSettings } from '@/lib/entry.constants';
import { usePasswordValidation } from './usePasswordValidation';

export function usePasswordGenerator(): UsePasswordGeneratorReturn {
    // Initialize state with UI-specific properties
    const [state, setState] = useState<PasswordGeneratorState | null>(null);
    
    // Animation controls for the settings panel
    const controls = useAnimationControls();
    const { toast } = useToast();
    const { validateSettings: validatePasswordSettings, calculateMinimumLength } = usePasswordValidation();

    // Memoize settings validation
    const isSettingsValid = useMemo(() => {
        if (!state) return false;
        const result = validatePasswordSettings(state);
        if (!result.isValid && result.error !== null) {
            setState(prev => prev ? { ...prev, error: result.error } : null);
        }
        return result.isValid;
    }, [state, validatePasswordSettings]);

    // Load initial settings
    useEffect(() => {
        let isMounted = true;

        const loadSettings = async () => {
            try {
                setState(prev => prev ? { ...prev, isLoading: true } : null);
                const savedSettings = await getPasswordGeneratorSettings();
                
                if (isMounted) {
                    setState({
                        ...savedSettings,
                        isOpen: false,
                        isLoading: false,
                        isGenerating: false,
                        error: null,
                        lastGeneratedPassword: null,
                        entropy: null,
                        showSettings: false,
                        copied: false
                    });
                }
            } catch (err) {
                console.error('❌ Failed to load password settings:', err);
                if (isMounted) {
                    const errorMessage = err instanceof Error ? err.message :
                                       (err as AppError)?.payload?.message ?? 'Failed to load settings';
                    
                    setState({
                        ...initialPasswordSettings,
                        isOpen: false,
                        isLoading: false,
                        isGenerating: false,
                        error: errorMessage,
                        lastGeneratedPassword: null,
                        entropy: null,
                        showSettings: false,
                        copied: false
                    });
                }
            }
        };

        loadSettings();
        return () => { isMounted = false; };
    }, []);

    // Memoize update functions
    const updateSetting = useCallback(async <K extends keyof PasswordGeneratorSettings>(
        setting: K,
        value: PasswordGeneratorSettings[K]
    ): Promise<void> => {
        if (!state) return;

        try {
            const updatedSettings = {
                ...state,
                [setting]: value
            };

            // Calculate new minimum length after the setting change
            const requiredLength = calculateMinimumLength(updatedSettings);
            
            // If the new setting would make the current length invalid,
            // adjust the length to meet the new minimum
            if (updatedSettings.length < requiredLength) {
                updatedSettings.length = requiredLength;
            }

            // Update backend with potentially adjusted settings
            await updatePasswordGeneratorSettings(updatedSettings);

            // Update local state with all changes
            setState(prev => prev ? {
                ...prev,
                ...updatedSettings,
                error: null
            } : null);

        } catch (err) {
            console.error('❌ Failed to update setting:', err);
            const errorMessage = err instanceof Error ? err.message :
                               (err as AppError)?.payload?.message ?? 'Failed to update setting';
            
            setState(prev => prev ? { ...prev, error: errorMessage } : null);
            throw err;
        }
    }, [state, calculateMinimumLength]);

    // Generate a new password
    const generatePassword = useCallback(async (): Promise<string> => {
        if (!state) throw new Error('Password generator not initialized');

        setState(prev => prev ? { ...prev, isGenerating: true, error: null } : null);

        try {
            // Generate password and calculate entropy in parallel
            const [newPassword, entropyValue] = await Promise.all([
                generatePasswordApi(),
                calculateEntropyApi()
            ]);

            setState(prev => prev ? {
                ...prev,
                isGenerating: false,
                lastGeneratedPassword: newPassword,
                entropy: entropyValue,
                error: null
            } : null);

            toast({
                title: "Password Generated",
                description: `Entropy: ${Math.floor(entropyValue)} bits`,
                variant: "default",
            });

            return newPassword;
        } catch (err) {
            console.error('❌ Failed to generate password:', err);
            const errorMessage = err instanceof Error ? err.message :
                               (err as AppError)?.payload?.message ?? 'Failed to generate password';
            
            setState(prev => prev ? {
                ...prev,
                isGenerating: false,
                error: errorMessage
            } : null);
            throw err;
        }
    }, [state, toast]);

    // Toggle settings panel with animation
    const togglePanel = useCallback(() => {
        if (!state) return;

        setState(prev => prev ? {
            ...prev,
            isOpen: !prev.isOpen
        } : null);

        // Animate the panel
        controls.start(state.isOpen ? "closed" : "open");
    }, [state, controls]);

    // Clear error state
    const clearError = useCallback(() => {
        setState(prev => prev ? { ...prev, error: null } : null);
    }, []);

    const setShowSettings = useCallback((show: boolean) => {
        setState(prev => prev ? { ...prev, showSettings: show } : null);
    }, []);

    const setCopied = useCallback((copied: boolean) => {
        setState(prev => prev ? { ...prev, copied: copied } : null);
    }, []);

    return useMemo(() => ({
        settings: state,
        controls,
        generatePassword,
        updateSetting,
        togglePanel,
        isSettingsValid,
        clearError,
        setShowSettings,
        setCopied
    }), [state, controls, generatePassword, updateSetting, isSettingsValid]);
}