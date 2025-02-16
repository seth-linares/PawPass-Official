// src/contexts/AuthContext.tsx

import { createContext, useContext, ReactNode, useState } from 'react';
import { logout } from '@/api/authApi';

interface AuthContextType {
    isAuthenticated: boolean;
    setAuthenticated: (value: boolean) => void;
    logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const useAuthContext = () => {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error('useAuth must be used within an AuthProvider');
    }
    return context;
};

export const AuthProvider = ({ children }: { children: ReactNode }) => {
    const [isAuthenticated, setIsAuthenticated] = useState(false);

    const handleLogout = async () => {
        try {
            await logout();
            setIsAuthenticated(false);
        } catch (error) {
            console.error('Logout failed:', error);
            throw error;
        }
    };

    return (
        <AuthContext.Provider value={{ 
            isAuthenticated, 
            setAuthenticated: setIsAuthenticated,
            logout: handleLogout 
        }}>
            {children}
        </AuthContext.Provider>
    );
};
