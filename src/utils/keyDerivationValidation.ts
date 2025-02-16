export const KEY_DERIVATION_CONSTANTS = {
    RECOMMENDED_MEMORY_COST: 46_080,
    RECOMMENDED_TIME_COST: 1,
    MIN_KEY_LENGTH: 16,
    MAX_KEY_LENGTH: 64,
    MAX_MEMORY_COST: 8_388_608,
    MAX_TIME_COST: 50,
    MAX_PARALLELISM: 16,
    MIN_PARALLELISM: 1,
} as const;

export interface ValidationError {
    field: string;
    message: string;
}

export function validateKeyDerivation(params: {
    memoryCost: number;
    timeCost: number;
    parallelism: number;
}): ValidationError[] {
    const errors: ValidationError[] = [];

    // Memory cost validation
    if (params.memoryCost < 8 * params.parallelism) {
        errors.push({
            field: 'memoryCost',
            message: `Memory cost must be at least ${8 * params.parallelism} KiB (8 * parallelism)`,
        });
    }
    if (params.memoryCost > KEY_DERIVATION_CONSTANTS.MAX_MEMORY_COST) {
        errors.push({
            field: 'memoryCost',
            message: `Memory cost cannot exceed ${KEY_DERIVATION_CONSTANTS.MAX_MEMORY_COST} KiB`,
        });
    }

    // Time cost validation
    if (params.timeCost < 1 || params.timeCost > KEY_DERIVATION_CONSTANTS.MAX_TIME_COST) {
        errors.push({
            field: 'timeCost',
            message: `Time cost must be between 1 and ${KEY_DERIVATION_CONSTANTS.MAX_TIME_COST}`,
        });
    }

    // Parallelism validation
    if (params.parallelism < KEY_DERIVATION_CONSTANTS.MIN_PARALLELISM || 
        params.parallelism > KEY_DERIVATION_CONSTANTS.MAX_PARALLELISM) {
        errors.push({
            field: 'parallelism',
            message: `Parallelism must be between ${KEY_DERIVATION_CONSTANTS.MIN_PARALLELISM} and ${KEY_DERIVATION_CONSTANTS.MAX_PARALLELISM}`,
        });
    }

    return errors;
}
