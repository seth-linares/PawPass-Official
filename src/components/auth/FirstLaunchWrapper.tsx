// src/components/auth/FirstLaunchWrapper.tsx

import { useVaultContext } from '@/contexts/VaultContext';
import InitializationView from './InitializationView';
import LoginView from './LoginView';
import LoadingSpinner from '../LoadingSpinner';

const FirstLaunchWrapper: React.FC = () => {
  const { isChecking, vaultExists, error } = useVaultContext();
  
  console.log('🔍 FirstLaunchWrapper State:', { isChecking, vaultExists, error });

  if (isChecking) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <LoadingSpinner size="lg" label="Checking vault status..." />
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-error">
          Failed to check vault status: {error.message}
        </div>
      </div>
    );
  }

  // The whole point of this component is to decide if we should show the login view or the initialization view
  return vaultExists ? <LoginView /> : <InitializationView />;
};

export default FirstLaunchWrapper;