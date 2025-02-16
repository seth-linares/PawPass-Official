/**
 * Error codes organized by category -- ported from the Rust backend
 * 
 */
export const ErrorCodes = {
  Auth: {
    INVALID_PASSWORD: 'AUTH001',
    VAULT_LOCKED: 'AUTH002',
    VAULT_UNLOCKED: 'AUTH003',
    KEY_DERIVATION_FAILED: 'AUTH004',
    OPERATION_FAILED: 'AUTH005',
    UNKNOWN: 'AUTH999',
  },
  Crypto: {
    KEY_DERIVATION_FAILED: 'CRYPTO001',
    INVALID_KEY_LENGTH: 'CRYPTO002',
    EMPTY_PASSWORD: 'CRYPTO003',
    INVALID_PARAMETERS: 'CRYPTO004',
    UNKNOWN: 'CRYPTO999',
  },
  Category: {
    INVALID_NAME: 'CAT001',
    DUPLICATE_NAME: 'CAT002',
    NOT_FOUND: 'CAT003',
    NOT_FOUND_BY_NAME: 'CAT004',
    UNKNOWN: 'CAT999',
  },
  Entry: {
    INVALID_PASSWORD: 'ENTRY001',
    INVALID_TITLE: 'ENTRY002',
    NOT_FOUND: 'ENTRY003',
    UNKNOWN: 'ENTRY999',
  },
  Io: {
    OPERATION_FAILED: 'IO001',
    PATH_NOT_FOUND: 'IO002',
    INSUFFICIENT_PERMISSIONS: 'IO003',
    DIRECTORY_CREATE_FAILED: 'IO004',
    DIRECTORY_READ_FAILED: 'IO005',
    UNKNOWN: 'IO999',
  },
  PasswordGeneration: {
    GENERATION_FAILED: 'PWD001',
    PASSWORD_TOO_SHORT: 'PWD002',
    PASSWORD_TOO_LONG: 'PWD003',
    PASSWORD_TOO_WEAK: 'PWD004',
    UNKNOWN: 'PWD999',
  },
  Validation: {
    PASSWORDS_DO_NOT_MATCH: 'VAL001',
    UNKNOWN: 'VAL999',
  },
  Backup: {
    CREATION_FAILED: 'BACKUP001',
    RESTORE_FAILED: 'BACKUP002',
    DELETION_FAILED: 'BACKUP003',
    UNKNOWN: 'BACKUP999',
  },
  Data: {
    CORRUPTION_DETECTED: 'DATA001',
    SERIALIZATION_FAILED: 'DATA002',
    DESERIALIZATION_FAILED: 'DATA003',
    UNKNOWN: 'DATA999',
  },
  Vault: {
    OPERATION_FAILED: 'VAULT001',
    LOCKED: 'VAULT002',
    NOT_INITIALIZED: 'VAULT003',
    ALREADY_EXISTS: 'VAULT004',
    NOT_FOUND: 'VAULT005',
    SETTINGS_OUT_OF_SYNC: 'VAULT006',
    UNKNOWN: 'VAULT999',
  },
} as const;

export type ErrorCode = typeof ErrorCodes[keyof typeof ErrorCodes][keyof typeof ErrorCodes[keyof typeof ErrorCodes]];


interface BaseErrorPayload {
  code: string;
  message: string;
}


interface AuthErrorPayload extends BaseErrorPayload {
  details?: string;
}


interface CryptoErrorPayload extends BaseErrorPayload {
  details?: string;
}


interface CategoryErrorPayload extends BaseErrorPayload {
  categoryId?: string;
  categoryName?: string;
}


interface EntryErrorPayload extends BaseErrorPayload {
  entryId?: string;
  fieldName?: string;
}

interface PasswordGenerationErrorPayload extends BaseErrorPayload {
  length?: number;
  details?: string;
}


interface VaultErrorPayload extends BaseErrorPayload {
  path?: string;
  details?: string;
}


interface IoErrorPayload extends BaseErrorPayload {
  path?: string;
  operation?: string;
}


interface DataErrorPayload extends BaseErrorPayload {
  details?: string;
}


interface ValidationErrorPayload extends BaseErrorPayload {
  field?: string;
  reason?: string;
}


interface UnknownErrorPayload extends BaseErrorPayload {
  details?: string;
}

/**
 * The main error type that corresponds to our Rust SerializableError
 * This is what we'll receive from Tauri commands
 */
export type AppError =
  | { kind: 'auth';               payload: AuthErrorPayload }
  | { kind: 'crypto';             payload: CryptoErrorPayload }
  | { kind: 'category';           payload: CategoryErrorPayload }
  | { kind: 'entry';              payload: EntryErrorPayload }
  | { kind: 'passwordGeneration'; payload: PasswordGenerationErrorPayload }
  | { kind: 'vault';              payload: VaultErrorPayload }
  | { kind: 'io';                 payload: IoErrorPayload }
  | { kind: 'data';               payload: DataErrorPayload }
  | { kind: 'validation';         payload: ValidationErrorPayload }
  | { kind: 'unknown';            payload: UnknownErrorPayload };

/**
 * Type guard to check if an unknown error is an AppError
 * @param error - The error to check
 * @returns True if the error matches the AppError structure
 */
export function isAppError(error: unknown): error is AppError {
  if (typeof error !== 'object' || error === null) return false;
  
  const err = error as Record<string, unknown>;
  return 'kind' in err && 
         'payload' in err && 
         typeof err.payload === 'object' &&
         err.payload !== null &&
         'code' in err.payload &&
         'message' in err.payload;
}

/**
 * Creates a user-friendly error message by combining the base message with relevant context
 * from the error payload. The function examines the error kind and available payload details
 * to construct the most informative message possible.
 * 
 * @param error - The AppError to generate a message for
 * @returns A formatted error message string with relevant context
 * @example
 * const error: AppError = {
 *   kind: 'validation',
 *   payload: {
 *     code: 'VAL001',
 *     message: 'Validation failed',
 *     field: 'email',
 *     reason: 'Invalid format'
 *   }
 * };
 * const message = getErrorMessage(error);
 * // Returns: "Validation failed (Field: email, Reason: Invalid format)"
 */
export function getErrorMessage(error: AppError): string {
  const { kind, payload } = error;
  
  switch (kind) {
    case 'auth':
      if (payload.details) {
        return `${payload.message} (${payload.details})`;
      }
      break;

    case 'crypto':
      // Only expose non-sensitive cryptographic error details
      if (payload.details) {
        return `${payload.message} (${payload.details})`;
      }
      break;

    case 'category':
      // Prioritize category name over ID for better user experience
      if (payload.categoryName) {
        return `${payload.message} (Category: ${payload.categoryName})`;
      }
      if (payload.categoryId) {
        return `${payload.message} (Category ID: ${payload.categoryId})`;
      }
      break;

    case 'entry':
      // Combine entry ID and field name if both are available
      if (payload.entryId && payload.fieldName) {
        return `${payload.message} (Entry ID: ${payload.entryId}, Field: ${payload.fieldName})`;
      }
      if (payload.fieldName) {
        return `${payload.message} (Field: ${payload.fieldName})`;
      }
      if (payload.entryId) {
        return `${payload.message} (Entry ID: ${payload.entryId})`;
      }
      break;

    case 'passwordGeneration':
      if (payload.length) {
        return `${payload.message} (Length: ${payload.length})`;
      }
      if (payload.details) {
        return `${payload.message} (${payload.details})`;
      }
      break;

    case 'vault':
      if (payload.path && payload.details) {
        return `${payload.message} (Path: ${payload.path}, ${payload.details})`;
      }
      if (payload.path) {
        return `${payload.message} (Path: ${payload.path})`;
      }
      if (payload.details) {
        return `${payload.message} (${payload.details})`;
      }
      break;

    case 'io':
      // Include both operation and path details for better error tracking
      if (payload.path && payload.operation) {
        return `${payload.message} (Path: ${payload.path}, Operation: ${payload.operation})`;
      }
      if (payload.path) {
        return `${payload.message} (Path: ${payload.path})`;
      }
      if (payload.operation) {
        return `${payload.message} (Operation: ${payload.operation})`;
      }
      break;

    case 'validation':
      if (payload.field && payload.reason) {
        return `${payload.message} (Field: ${payload.field}, Reason: ${payload.reason})`;
      }
      if (payload.field) {
        return `${payload.message} (Field: ${payload.field})`;
      }
      if (payload.reason) {
        return `${payload.message} (Reason: ${payload.reason})`;
      }
      break;

    case 'data':
      if (payload.details) {
        return `${payload.message} (${payload.details})`;
      }
      break;
  }
  
  return payload.message;
}

/**
 * Check if an error matches a specific error code
 * @param error - The AppError to check
 * @param code - The error code to match against
 * @returns True if the error matches the code
 */
export function isErrorCode(error: AppError, code: ErrorCode): boolean {
  return error.payload.code === code;
}

export class CommandError extends Error {
  constructor(public error: AppError) {
    super(error.payload.message);
    this.name = 'CommandError';
  }
}

export function handleError(error: unknown): Error {
  if (isAppError(error)) {
    return new CommandError(error);
  }
  return error instanceof Error ? error : new Error('An unknown error occurred');
}