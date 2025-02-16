import { invokeCommand } from './commandUtils';
import { KeyDerivation } from './apiTypes';

export async function login(password: string): Promise<void> {
    return invokeCommand<void>('login', { password });
}

export async function logout(): Promise<void> {
    return invokeCommand<void>('logout');
}

export async function changeMasterPassword(
    oldPassword: string,
    newPassword: string,
    confirmPassword: string,
): Promise<void> {
    return invokeCommand<void>('change_master_password', { 
        oldPassword, 
        newPassword, 
        confirmPassword 
    });
}

export async function updateKeyDerivation(
    masterPassword: string,
    settings: KeyDerivation
): Promise<void> {
    return invokeCommand<void>('update_key_derivation', { 
        masterPassword, 
        settings 
    });
}