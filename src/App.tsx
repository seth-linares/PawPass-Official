// src/App.tsx

import { Routes, Route, Navigate } from "react-router-dom";
import FirstLaunchWrapper from "./components/auth/FirstLaunchWrapper";
import ThemeSwitcher from "./components/settings/ThemeSwitcher";
import { useVaultContext } from "./contexts/VaultContext";
import { AuthProvider, useAuthContext } from "./contexts/AuthContext";
import DashboardLayout from "./components/dashboard/DashboardLayout";
import EntryForm from "./components/entry/EntryForm";
import { DashboardProvider } from '@/contexts/DashboardContext';
import Settings from "./components/settings/Settings";
import TitleBar from "./components/TitleBar";

function App() {
  return (
    <AuthProvider>
      <div className="min-h-screen flex flex-col">
        <TitleBar />
        <div className="flex-1">
          <VaultContents />
        </div>
      </div>
    </AuthProvider>
  );
}

const VaultContents = () => {
  const { vaultExists, isChecking } = useVaultContext();
  const { isAuthenticated } = useAuthContext();
  
  console.log('📱 VaultContents Render:', { vaultExists, isChecking, isAuthenticated });

  if (isChecking) {
    return (
      <div className="min-h-screen">
        <div className="absolute top-4 right-4">
          <ThemeSwitcher />
        </div>
        <div className="flex items-center justify-center min-h-screen">
          <div>Checking vault status...</div>
        </div>
      </div>
    );
  }

  return (
    <Routes>
      <Route path="/auth" element={<FirstLaunchWrapper />} />
      
      {/* Authenticated routes wrapped in DashboardProvider */}
      <Route
        path="/vault/*"
        element={
          isAuthenticated ? (
            <DashboardProvider>
              <DashboardLayout />
            </DashboardProvider>
          ) : (
            <Navigate to="/auth" replace />
          )
        }
      />
      
      <Route
        path="/entry/*"
        element={
          isAuthenticated ? (
            <DashboardProvider>
              <Routes>
                <Route path="new" element={<EntryForm />} />
                <Route path=":id" element={<EntryForm />} />
              </Routes>
            </DashboardProvider>
          ) : (
            <Navigate to="/auth" replace />
          )
        }
      />
      
      <Route
        path="/settings"
        element={
          isAuthenticated ? (
            <DashboardProvider>
              <Settings />
            </DashboardProvider>
          ) : (
            <Navigate to="/auth" replace />
          )
        }
      />

      <Route 
        path="*" 
        element={<Navigate to={vaultExists ? "/vault" : "/auth"} replace />} 
      />
    </Routes>
  );
};

export default App;