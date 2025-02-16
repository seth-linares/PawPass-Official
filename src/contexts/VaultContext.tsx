// src/contexts/VaultContext.tsx

import { createContext, ReactNode, useContext } from 'react';
import useVaultCheck from '@/hooks/auth/useVaultCheck';

interface VaultContextType {
    isChecking: boolean;
    vaultExists: boolean;
    error: Error | null;
    recheckVault: () => Promise<void>;
}

interface VaultProviderProps {
    children: ReactNode;
}

export const VaultContext = createContext<VaultContextType | null>(null);



export const useVaultContext = () => {
    const context = useContext(VaultContext);
    if (!context) {
        throw new Error('useVault must be used within a VaultProvider');
    }
    return context;
};

const VaultProvider = ({ children }: VaultProviderProps) => {
    const vaultState = useVaultCheck();

    return (
        <VaultContext.Provider value={vaultState}>
            {children}
        </VaultContext.Provider>
    );
};

export default VaultProvider;