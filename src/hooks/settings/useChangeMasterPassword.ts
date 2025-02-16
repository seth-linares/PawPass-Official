import { useCallback, useState, useMemo } from "react";
import { changeMasterPassword } from "@/api/authApi";
import { AppError } from "@/api/apiTypes";
import { ChangeMasterPasswordState } from "@/types/types";



const useChangeMasterPassword = () => {
    const [state, setState] = useState<ChangeMasterPasswordState>({
        isChanging: false,
        error: null,
        success: false,
        oldPassword: "",
        newPassword: "",
        confirmPassword: "",
        isOpen: false
    });

    const setOpen = useCallback((open: boolean) => {
        setState(prev => ({ ...prev, isOpen: open }));
        if (!open) {
            reset();
        }
    }, []);

    const setOldPassword = useCallback((password: string) => {
        setState(prev => ({ ...prev, oldPassword: password }));
    }, []);

    const setNewPassword = useCallback((password: string) => {
        setState(prev => ({ ...prev, newPassword: password }));
    }, []);

    const setConfirmPassword = useCallback((password: string) => {
        setState(prev => ({ ...prev, confirmPassword: password }));
    }, []);

    const changePassword = useCallback(async () => {
        setState(prev => ({ ...prev, isChanging: true, error: null, success: false }));

        try {
            await changeMasterPassword(state.oldPassword, state.newPassword, state.confirmPassword);
            setState(prev => ({ ...prev, isChanging: false, error: null, success: true }));
            return true;
        } catch (error) {
            setState(prev => ({ ...prev, isChanging: false, error: error as AppError, success: false }));
            return false;
        }
    }, [state.oldPassword, state.newPassword, state.confirmPassword]);

    const reset = useCallback(() => {
        setState(prev => ({
            ...prev,
            isChanging: false,
            error: null,
            success: false,
            oldPassword: "",
            newPassword: "",
            confirmPassword: "",
        }));
    }, []);

    return useMemo(() => ({
        ...state,
        setOpen,
        setOldPassword,
        setNewPassword,
        setConfirmPassword,
        changePassword,
        reset,
    }), [
        state,
        setOpen,
        setOldPassword,
        setNewPassword,
        setConfirmPassword,
        changePassword,
        reset
    ]);
};

export default useChangeMasterPassword;