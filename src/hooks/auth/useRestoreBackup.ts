import { useState, useCallback, useMemo } from "react";
import { restoreFromBackup } from "@/api/vaultApi";
import { AppError } from "@/api/apiTypes";
import { RestoreBackupReturn, RestoreBackupState } from "@/types/types";



const useRestoreBackup = (): RestoreBackupReturn => {
    const [filePath, setFilePath] = useState<string | undefined>(undefined);
    const [state, setState] = useState<RestoreBackupState>({
        isRestoring: false,
        error: null,
        success: false,
    });

    const selectBackupFile = useCallback((path: string) => {
        setFilePath(path);
    }, []);

    const restoreBackup = useCallback(async () => {
        if (!filePath) {
            return;
        }

        setState({ isRestoring: true, error: null, success: false });

        try {
            await restoreFromBackup(filePath);
            setState({ isRestoring: false, error: null, success: true });
        } catch (error) {
            setState({ isRestoring: false, error: error as AppError, success: false });
        }
    }, [filePath]);

    const resetState = useCallback(() => {
        setState({ isRestoring: false, error: null, success: false });
        setFilePath(undefined);
    }, []);

    return useMemo(() => ({
        filePath,
        selectBackupFile,
        restoreBackup,
        resetState,
        isRestoring: state.isRestoring,
        error: state.error,
        success: state.success
    }), [
        filePath,
        selectBackupFile,
        restoreBackup,
        resetState,
        state.isRestoring,
        state.error,
        state.success
    ]);
};

export default useRestoreBackup;