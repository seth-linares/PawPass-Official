import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthContext } from '@/contexts/AuthContext';
import { getVaultStatus } from '@/api/vaultApi';
import { VaultStatus } from '@/api/apiTypes';

export const useDashboard = () => {
  const { logout } = useAuthContext();
  const navigate = useNavigate();
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
  const [vaultStatus, setVaultStatus] = useState<VaultStatus | null>(null);
  const [isInitialLoad, setIsInitialLoad] = useState(true);

  const handleLogout = async () => {
    try {
      await logout();
    } catch (error) {
      console.error('Failed to logout:', error);
    }
  };

  useEffect(() => {
    const checkVaultStatus = async () => {
      try {
        const status = await getVaultStatus();
        setVaultStatus(status);
        if (isInitialLoad) {
          setIsInitialLoad(false);
        }
      } catch (error) {
        console.error('Failed to get vault status:', error);
      }
    };

    // Initial check
    checkVaultStatus();

    // Only set up polling if not initial load
    const interval = !isInitialLoad ? 
      setInterval(checkVaultStatus, 30000) : 
      null;

    return () => {
      if (interval) clearInterval(interval);
    };
  }, [isInitialLoad]);

  return {
    isSidebarCollapsed,
    setIsSidebarCollapsed,
    vaultStatus,
    handleLogout,
    navigate
  };
};