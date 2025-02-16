// src/components/auth/LoginView.tsx

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { Card, CardContent, CardHeader, CardTitle, CardDescription, CardFooter } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { AlertCircle, Info } from 'lucide-react';
import LoadingSpinner from '../LoadingSpinner';
import useLogin from '@/hooks/auth/useLogin';
import { useNavigate } from 'react-router-dom';
import { useAuthContext } from '@/contexts/AuthContext';
// Switch between the two logos
import betterPawPassLogo from '@/assets/silly_kitty.png'; // 518x482
import pawPassLogo from '@/assets/PawPass_Logo.png'; // w-32 h-40
import FileOpener from '../FileOpener';
import useRestoreBackup from '@/hooks/auth/useRestoreBackup';
import { useSettings } from '@/hooks/settings/useSettings';

const getFileName = (path: string) => {
    return path.split('\\').pop() || path.split('/').pop() || path;
};

interface LoginFormData {
    password: string;
}

const LoginView = () => {
    const navigate = useNavigate();
    const { setAuthenticated } = useAuthContext();
    const [showPassword, setShowPassword] = useState(false);
    const { logoPreference } = useSettings();
    
    const { 
        register, 
        handleSubmit, 
        formState: { errors },
        reset 
    } = useForm<LoginFormData>();

    const { 
        isLoggingIn, 
        error, 
        attemptLogin, 
        clearError 
    } = useLogin();

    const { 
        selectBackupFile, 
        restoreBackup, 
        isRestoring, 
        error: restoreError, 
        success: restoreSuccess,
        filePath 
    } = useRestoreBackup();

    const handleBackupFileSelect = (path: string) => {
        console.log('🔄 LoginView: Backup file selected:', path);
        selectBackupFile(path);
    };

    const handleRestore = async () => {
        console.log('🔄 LoginView: Starting restore with file:', filePath);
        await restoreBackup();
    };

    const onSubmit = async (data: LoginFormData) => {
        try {
            clearError();
            await attemptLogin(data.password);
            setAuthenticated(true);
            navigate('/vault');
            reset();
        } catch (err) {
            console.error('Form submission failed');
        }
    };

    return (
        <div className="flex items-center justify-center min-h-[calc(100vh-2rem)] p-4">
            <Card className="w-full max-w-md">
                <CardHeader className="text-center space-y-4">
                    <img
                        src={logoPreference === 'silly' ? betterPawPassLogo : pawPassLogo}
                        alt="PawPass Logo"
                        className={`mx-auto animate-fade-in-down ${
                            logoPreference === 'silly' ? 'w-32 h-32' : 'w-32 h-40'}`}
                    />
                    <div>
                        <CardTitle>Welcome Back</CardTitle>
                        <CardDescription>
                            Enter your master password to access your vault
                        </CardDescription>
                    </div>
                </CardHeader>

                <CardContent>
                    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
                        <div className="space-y-2">
                            <div className="relative">
                                <Input
                                    type={showPassword ? "text" : "password"}
                                    placeholder="Master Password"
                                    {...register("password", {
                                        required: "Password is required"
                                    })}
                                    disabled={isLoggingIn}
                                    className="pr-24"
                                />
                                <Button
                                    type="button"
                                    variant="ghost"
                                    size="sm"
                                    className="absolute right-2 top-1/2 -translate-y-1/2"
                                    onClick={() => setShowPassword(!showPassword)}
                                >
                                    {showPassword ? 'Hide' : 'Show'}
                                </Button>
                            </div>
                            {errors.password && (
                                <p className="text-sm text-error">{errors.password.message}</p>
                            )}
                        </div>

                        {error && (
                            <Alert variant="destructive" className="mt-4">
                                <AlertCircle className="h-4 w-4" />
                                <AlertDescription>{error}</AlertDescription>
                            </Alert>
                        )}

                        <Button 
                            type="submit" 
                            className="w-full" 
                            disabled={isLoggingIn}
                        >
                            {isLoggingIn ? (
                                <LoadingSpinner inline size="sm" />
                            ) : (
                                'Unlock Vault'
                            )}
                        </Button>
                    </form>
                </CardContent>

                <CardFooter className="flex flex-col gap-2">
                    <div className="text-sm text-base-content/70 text-center">
                        Forgot your master password? Unfortunately, it can't be recovered.
                        You'll need to restore from a backup or reset your vault.
                    </div>
                    {restoreError && (
                        <Alert variant="destructive" className="mt-2">
                            <AlertCircle className="h-4 w-4" />
                            <AlertDescription>
                                {restoreError.payload.message}
                            </AlertDescription>
                        </Alert>
                    )}
                    {restoreSuccess && (
                        <Alert className="mt-2">
                            <AlertDescription>
                                Backup restored successfully! Please login with your backup's password.
                            </AlertDescription>
                        </Alert>
                    )}
                    <div className="flex flex-col gap-2 w-full">
                        <div className="flex gap-2">
                            <FileOpener onFileSelect={handleBackupFileSelect} />
                            <Button 
                                variant="outline" 
                                className="flex-1 bg-base-300"
                                onClick={handleRestore}
                                disabled={isRestoring || !filePath}
                            >
                                {isRestoring ? (
                                    <LoadingSpinner inline size="sm" />
                                ) : (
                                    'Restore from Backup'
                                )}
                            </Button>
                        </div>
                        {filePath && (
                            <div role="alert" className="alert alert-warning text-sm">
                                <Info className="h-4 w-4 stroke-current" />
                                <div className="break-all">
                                    <span className="opacity-75">Selected backup: </span>
                                    <div 
                                        className="tooltip tooltip-top font-medium" 
                                        data-tip={filePath}
                                    >
                                        {getFileName(filePath)}
                                    </div>
                                </div>
                            </div>
                        )}
                    </div>
                </CardFooter>
            </Card>
        </div>
    );
};

export default LoginView;