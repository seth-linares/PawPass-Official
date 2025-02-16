// src/hooks/useVaultCheck.ts

import { useState, useEffect } from 'react';
import { checkVaultExists } from '@/api/vaultApi';
import { VaultCheckState } from '@/types/types';



const useVaultCheck = (): VaultCheckState => {
    const [isChecking, setIsChecking] = useState(true);
    const [vaultExists, setVaultExists] = useState(false);
    const [error, setError] = useState<Error | null>(null);

    const checkVault = async () => {
        console.log('🔍 Starting vault existence check...');
        try {
            setIsChecking(true);
            setError(null);
            const exists = await checkVaultExists();
            console.log(`✅ Vault check complete. Vault exists: ${exists}`);
            setVaultExists(exists);
        } catch (err) {
            const errorMessage = err instanceof Error ? err.message : 'Failed to check vault existence';
            console.error('❌ Vault check failed:', errorMessage);
            setError(err instanceof Error ? err : new Error(errorMessage));
        } finally {
            setIsChecking(false);
            console.log('🏁 Vault check process finished');
        }
    };

    // Check vault existence when the hook is first mounted
    useEffect(() => {
        checkVault();
    }, []);

    return {
        isChecking,
        vaultExists,
        error,
        recheckVault: checkVault
    };
};

export default useVaultCheck;