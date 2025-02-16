import { invoke } from "@tauri-apps/api/core";
import { AppError, isAppError } from './apiErrors';

/**
 * Wraps a Tauri command invocation with proper error handling and typing.
 * @param command - The Tauri command name to invoke
 * @param args - Optional arguments for the command
 * @returns A promise that resolves with the command result or rejects with a typed AppError
 */
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    console.log(`Invoking command: ${command}`, args);
    const result = await invoke<T>(command, args);
    return result;
  } catch (error) {
    console.error(`Command ${command} failed:`, error);
    
    // If it's already an AppError, rethrow it
    if (isAppError(error)) {
      throw error;
    }
    
    // Otherwise, wrap it in an AppError
    throw {
      kind: 'unknown',
      payload: {
        code: 'UNKNOWN',
        message: error instanceof Error ? error.message : 'An unknown error occurred',
        details: String(error),
      },
    } as AppError;
  }
}