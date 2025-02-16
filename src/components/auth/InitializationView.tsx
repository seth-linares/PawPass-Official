// src/components/auth/InitializationView.tsx

import React from 'react';
import { useForm } from 'react-hook-form';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { AlertCircle, KeyRound, CatIcon } from 'lucide-react';
import LoadingSpinner from '../LoadingSpinner';
import useVaultInitialization from '@/hooks/auth/useVaultInitialization';

interface InitializationFormData {
    password: string;
    confirmPassword: string;
}

const InitializationView: React.FC = () => {
    const { 
        register, 
        handleSubmit, 
        formState: { errors },
        reset 
    } = useForm<InitializationFormData>();

    const { 
        isInitializing, 
        error, 
        handleInitialization, 
        clearError,
        showPassword,
        showConfirmPassword,
        togglePasswordVisibility
    } = useVaultInitialization();

    const onSubmit = async (data: InitializationFormData) => {
        try {
            clearError();
            await handleInitialization(data.password, data.confirmPassword);
            reset();
        } catch (err) {
            // error should now be handled by the hook, but I'll just log it here for good measure
            console.error('Form submission failed');
        }
    };

    return (
        <div className="flex items-center justify-center min-h-[calc(100vh-2rem)] p-4">
            <Card className="w-full max-w-md">
                <CardHeader className="text-center space-y-4">
                    <div className="mx-auto p-4 rounded-full bg-secondary/70">
                        <CatIcon className="w-16 h-16 text-secondary-foreground" />
                    </div>
                    <div>
                        <CardTitle className="text-2xl">Create Your Vault</CardTitle>
                        <CardDescription className="mt-2 text-base">
                            Secure your digital life with a strong master password. 
                            This will be your key to accessing all your passwords.
                        </CardDescription>
                    </div>
                </CardHeader>

                <CardContent>
                    <div className="bg-secondary/70 p-4 rounded-lg mb-6 relative">
                        <KeyRound className="w-6 h-6 text-secondary-foreground absolute top-3 right-3" />
                        <h3 className="text-sm font-medium mb-2 text-secondary-foreground">Password Requirements:</h3>
                        <ul className="text-sm list-disc list-inside space-y-1 text-secondary-foreground/80">
                            <li>At least 8 characters long</li>
                            <li>The max length is 256 characters</li>
                            <li>Mix of uppercase and lowercase letters</li>
                            <li>Include numbers and special characters</li>
                            <li>Something memorable but hard to guess</li>
                        </ul>
                    </div>

                    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
                        <div className="space-y-2">
                            <div className="relative">
                                <Input
                                    type={showPassword ? "text" : "password"}
                                    placeholder="Master Password"
                                    {...register("password", {
                                        required: "Password is required",
                                        minLength: {
                                            value: 8,
                                            message: "Password must be at least 8 characters"
                                        }
                                    })}
                                    disabled={isInitializing}
                                    className="pr-24"
                                />
                                <Button
                                    type="button"
                                    variant="ghost"
                                    size="sm"
                                    className="absolute right-2 top-1/2 -translate-y-1/2"
                                    onClick={() => togglePasswordVisibility('password')}
                                >
                                    {showPassword ? 'Hide' : 'Show'}
                                </Button>
                            </div>
                            {errors.password && (
                                <p className="text-sm text-error">{errors.password.message}</p>
                            )}
                        </div>

                        <div className="space-y-2">
                            <div className="relative">
                                <Input
                                    type={showConfirmPassword ? "text" : "password"}
                                    placeholder="Confirm Password"
                                    {...register("confirmPassword", {
                                        required: "Please confirm your password",
                                        validate: (value, formValues) => 
                                            value === formValues.password || "Passwords do not match"
                                    })}
                                    disabled={isInitializing}
                                    className="pr-24"
                                />
                                <Button
                                    type="button"
                                    variant="ghost"
                                    size="sm"
                                    className="absolute right-2 top-1/2 -translate-y-1/2"
                                    onClick={() => togglePasswordVisibility('confirmPassword')}
                                >
                                    {showConfirmPassword ? 'Hide' : 'Show'}
                                </Button>
                            </div>
                            {errors.confirmPassword && (
                                <p className="text-sm text-error">{errors.confirmPassword.message}</p>
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
                            disabled={isInitializing}
                        >
                            {isInitializing ? (
                                <LoadingSpinner inline size="sm" />
                            ) : (
                                'Create Vault'
                            )}
                        </Button>
                    </form>
                </CardContent>
            </Card>
        </div>
    );
};

export default InitializationView;