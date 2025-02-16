// src/components/settings/Settings.tsx
import { ChevronDown } from "lucide-react";
import ThemeSwitcher from "./ThemeSwitcher";
import ThemePreview from "./ThemePreview";
import { Button } from "@/components/ui/button";
import { ArrowLeft } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { AnimatePresence, motion } from "framer-motion";
import { ChangeMasterPasswordDialog } from "./ChangeMasterPasswordDialog";
import { ChangeKeyDerivationDialog } from "./ChangeKeyDerivationDialog";
import useBackup from "@/hooks/settings/useBackup";
import { useSettings } from "@/hooks/settings/useSettings";
import { Switch } from "@/components/ui/switch";
import { useToast } from "@/hooks/use-toast";

const Settings = () => {
  const { toast } = useToast();
  const { isPreviewOpen, togglePreview, logoPreference, toggleLogo } = useSettings();
  const navigate = useNavigate();
  const { createNewBackup, isCreating, lastBackupTime } = useBackup();

  const handleBackup = async () => {
    await createNewBackup();
    toast({
      title: "Backup Created",
      description: "Your vault backup has been created successfully.",
      variant: "success",
      duration: 1500,
    });
  };

  return (
    <div className="container max-w-4xl mx-auto py-16 overflow-y-scroll">
      {/* Header */}
      <div className="flex items-center mb-8">
        <Button
          variant="ghost"
          size="sm"
          onClick={() => navigate('/vault')}
          className="mr-4"
        >
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <div>
          <h1 className="text-2xl font-bold text-base-content">Settings</h1>
          <p className="text-base-content/70">
            Customize your vault experience
          </p>
        </div>
      </div>

      <div className="grid gap-8">
        {/* Theme Settings */}
        <div className="card bg-base-200 p-6 rounded-lg">
          <h3 className="text-lg font-semibold mb-4 text-base-content">Appearance</h3>
          <ThemeSwitcher />
          
          <div className="flex items-center justify-between py-4">
            <div>
              <p className="font-medium">Use Silly Kitty Logo</p>
              <p className="text-sm text-base-content/70">Switch between classic and silly kitty logo</p>
            </div>
            <Switch
              checked={logoPreference === 'silly'}
              onCheckedChange={toggleLogo}
            />
          </div>

          <button
            onClick={togglePreview}
            className="btn btn-ghost btn-sm gap-2 mt-4 w-full justify-between"
          >
            <span className="text-base-content/70">Theme Preview</span>
            <ChevronDown
              className={`h-4 w-4 transition-transform duration-200 ${
                isPreviewOpen ? "rotate-180" : ""
              }`}
            />
          </button>
          <div className="relative overflow-hidden">
            <AnimatePresence initial={false}>
              {isPreviewOpen && (
                <motion.div
                  initial={{ height: 0 }}
                  animate={{ height: "auto" }}
                  exit={{ height: 0 }}
                  transition={{ 
                    duration: 0.2, 
                    ease: [0.4, 0, 0.2, 1] 
                  }}
                  className="preserve-3d"
                >
                  <div className="mt-6">
                    <ThemePreview />
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>

        {/* Security Settings */}
        <div className="card bg-base-200 p-6 rounded-lg">
          <h3 className="text-lg font-semibold mb-4 text-base-content">Security</h3>
          <div className="space-y-4">
            <div className="flex flex-col gap-2">
              <ChangeMasterPasswordDialog />
              <ChangeKeyDerivationDialog />
              <Button 
                className="btn-success"
                onClick={handleBackup}
                disabled={isCreating}
              >
                {isCreating ? "Creating Backup..." : "Create Backup"}
              </Button>
              {lastBackupTime && (
                <p className="text-sm text-muted-foreground">
                  Last backup: {lastBackupTime.toLocaleString()}
                </p>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Settings;