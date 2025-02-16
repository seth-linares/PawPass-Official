import { PasswordGeneratorSettings } from "@/api/apiTypes";
import { strengthLabels } from "@/lib/entry.constants";
import { ValidationResult } from "@/types/types";
import { ABSOLUTE_MIN_LENGTH, ABSOLUTE_MAX_LENGTH } from "@/lib/constants";
import { useMemo, useCallback } from 'react';



export function usePasswordValidation() {
    // Memoize the minimum length calculation
    const calculateMinimumLength = useCallback((settings: PasswordGeneratorSettings): number => {
        // First calculate required characters from enabled sets
        const requiredFromSets = (
            (settings.useLowercase ? 1 : 0) +
            (settings.useUppercase ? 1 : 0) +
            (settings.useNumbers ? settings.minNumbers : 0) +
            (settings.useSymbols ? settings.minSymbols : 0)
        );

        // Even if requirements are less than 5, we still enforce ABSOLUTE_MIN_LENGTH
        return Math.max(ABSOLUTE_MIN_LENGTH, requiredFromSets);
    }, []);

    // Memoize validation logic
    const validateSettings = useCallback((settings: PasswordGeneratorSettings): ValidationResult => {
        const minimumLength = calculateMinimumLength(settings);

        // Check absolute length bounds first
        if (settings.length < ABSOLUTE_MIN_LENGTH) {
            return {
                isValid: false,
                error: `Password length must be at least ${ABSOLUTE_MIN_LENGTH} characters`
            };
        }

        if (settings.length > ABSOLUTE_MAX_LENGTH) {
            return {
                isValid: false,
                error: `Password length cannot exceed ${ABSOLUTE_MAX_LENGTH} characters`
            };
        }

        // Ensure at least one character type is selected
        if (!settings.useLowercase && 
            !settings.useUppercase && 
            !settings.useNumbers && 
            !settings.useSymbols) {
            return {
                isValid: false,
                error: "At least one character type must be selected"
            };
        }

        // Check if length meets minimum requirements
        if (settings.length < minimumLength) {
            return {
                isValid: false,
                error: `Password length must be at least ${minimumLength} to accommodate minimum requirements`
            };
        }

        // All checks passed
        return { isValid: true, error: null };
    }, [calculateMinimumLength]);

    // Cache entropy calculation
    const calculateEntropy = useCallback((settings: PasswordGeneratorSettings): number => {
        let charsetSize = 0;

        // Calculate effective charset size based on selected options
        if (settings.useLowercase) charsetSize += 26; // a-z
        if (settings.useUppercase) charsetSize += 26; // A-Z
        if (settings.useNumbers) charsetSize += 10;   // 0-9
        if (settings.useSymbols) charsetSize += 8;    // Special characters

        // If excluding ambiguous, reduce the charset size
        if (settings.excludeAmbiguous) {
            // Fix the ambiguous groups calculation
            const ambiguousGroups = [
                '1lI'.length - 1,
                'oO0'.length - 1,
                '5S'.length - 1,
                '2Z'.length - 1,
                '8B'.length - 1
            ];
            
            charsetSize -= ambiguousGroups.reduce((acc, val) => acc + val, 0);
        }

        // Calculate entropy using the same formula as the Rust implementation
        return settings.length * Math.log2(charsetSize);
    }, []);

    // Memoize strength utilities
    const strengthUtils = useMemo(() => ({
        getStrengthColor: (entropy: number | null): string => {
            if (!entropy) return 'bg-gray-200';
            if (entropy < 50) return 'bg-error';
            if (entropy < 80) return 'bg-warning';
            return 'bg-success';
        },
        getStrengthLabel: (entropy: number | null): string => {
            if (!entropy) return strengthLabels.none;
            if (entropy < 50) return strengthLabels.weak;
            if (entropy < 80) return strengthLabels.medium;
            if (entropy < 128) return strengthLabels.strong;
            return strengthLabels.veryStrong;
        }
    }), []);

    return {
        validateSettings,
        calculateEntropy,
        calculateMinimumLength,
        ...strengthUtils
    };
}
