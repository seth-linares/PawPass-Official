import { usePasswordGenerator } from '@/hooks/entry/usePasswordGenerator';
import { motion, AnimatePresence } from 'framer-motion';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Slider } from '@/components/ui/slider';
import { Switch } from '@/components/ui/switch';
import { Progress } from '@/components/ui/progress';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { 
    AlertCircle,
    RefreshCw, 
    Shield,
    ChevronDown
} from 'lucide-react';
import { passwordGeneratorAnimations } from '@/lib/entry.constants';
import { 
    PasswordGeneratorProps, 
    SettingsPanelProps,
    StrengthIndicatorProps 
} from '@/types/entry.types';
import { usePasswordValidation } from '@/hooks/entry/usePasswordValidation';
import { useEffect, useCallback, memo } from 'react';
import { ABSOLUTE_MAX_LENGTH, ABSOLUTE_MIN_LENGTH } from '@/lib/constants';



const StrengthIndicator = memo(({ 
    password,
    entropy,
    getStrengthLabel,
    getStrengthColor 
}: StrengthIndicatorProps) => {
    if (!password) return null;
    
    return (
        <div className="space-y-1">
            <div className="flex justify-between text-sm">
                <span>Password Strength</span>
                <span>{getStrengthLabel(entropy)}</span>
            </div>
            <Progress 
                value={entropy ? (entropy / 128) * 100 : 0}
                className={getStrengthColor(entropy)}
            />
        </div>
    );
});

StrengthIndicator.displayName = 'StrengthIndicator';

const SettingsPanel = memo(({ 
    settings,
    minLength,
    onUpdateSetting 
}: SettingsPanelProps) => {
    return (
        <motion.div
            variants={passwordGeneratorAnimations.settingsPanelVariants}
            initial="hidden"
            animate="visible"
            exit="hidden"
            className="space-y-4 pt-4 border-t"
        >
            <motion.div 
                variants={passwordGeneratorAnimations.settingsVariants} 
                className="space-y-4"
            >
                {/* Length Setting */}
                <motion.div variants={passwordGeneratorAnimations.itemVariants} className="space-y-2">
                    <div className="flex justify-between">
                        <Label>Length</Label>
                        <span className="text-sm text-muted-foreground">
                            Min: {minLength}
                        </span>
                    </div>
                    <div className="flex gap-4">
                        <Slider
                            value={[settings.length]}
                            min={minLength}
                            max={ABSOLUTE_MAX_LENGTH}
                            step={1}
                            onValueChange={([value]) => {
                                onUpdateSetting('length', value);
                            }}
                            className="flex-1"
                        />
                        <Input
                            type="number"
                            value={settings.length}
                            onChange={(e) => {
                                const value = parseInt(e.target.value);
                                if (!isNaN(value)) {
                                    // Clamp the value between min and max
                                    const clampedValue = Math.min(
                                        Math.max(value, minLength),
                                        ABSOLUTE_MAX_LENGTH
                                    );
                                    onUpdateSetting('length', clampedValue);
                                }
                            }}
                            min={minLength}
                            max={ABSOLUTE_MAX_LENGTH}
                            className="w-20"
                        />
                    </div>
                </motion.div>

                {/* Character Type Toggles */}
                <motion.div 
                    variants={passwordGeneratorAnimations.itemVariants}
                    className="grid grid-cols-2 gap-4"
                >
                    <div className="flex items-center justify-between">
                        <Label htmlFor="lowercase">Lowercase</Label>
                        <Switch
                            id="lowercase"
                            checked={settings.useLowercase}
                            onCheckedChange={(checked) => {
                                onUpdateSetting('useLowercase', checked);
                            }}
                        />
                    </div>

                    <div className="flex items-center justify-between">
                        <Label htmlFor="uppercase">Uppercase</Label>
                        <Switch
                            id="uppercase"
                            checked={settings.useUppercase}
                            onCheckedChange={(checked) => {
                                onUpdateSetting('useUppercase', checked);
                            }}
                        />
                    </div>

                    <div className="flex items-center justify-between">
                        <Label htmlFor="numbers">Numbers</Label>
                        <Switch
                            id="numbers"
                            checked={settings.useNumbers}
                            onCheckedChange={(checked) => {
                                onUpdateSetting('useNumbers', checked);
                            }}
                        />
                    </div>

                    <div className="flex items-center justify-between">
                        <Label htmlFor="symbols">Symbols</Label>
                        <Switch
                            id="symbols"
                            checked={settings.useSymbols}
                            onCheckedChange={(checked) => {
                                onUpdateSetting('useSymbols', checked);
                            }}
                        />
                    </div>
                </motion.div>

                {/* Additional Settings */}
                <motion.div 
                    variants={passwordGeneratorAnimations.itemVariants} 
                    className="space-y-4"
                >
                    {/* Minimum Numbers */}
                    {settings.useNumbers && (
                        <div className="space-y-2">
                            <Label>Minimum Numbers: {settings.minNumbers}</Label>
                            <Slider
                                value={[settings.minNumbers]}
                                min={0}
                                max={settings.length}
                                step={1}
                                onValueChange={([value]) => {
                                    onUpdateSetting('minNumbers', value);
                                }}
                            />
                        </div>
                    )}

                    {/* Minimum Symbols */}
                    {settings.useSymbols && (
                        <div className="space-y-2">
                            <Label>Minimum Symbols: {settings.minSymbols}</Label>
                            <Slider
                                value={[settings.minSymbols]}
                                min={0}
                                max={settings.length}
                                step={1}
                                onValueChange={([value]) => {
                                    onUpdateSetting('minSymbols', value);
                                }}
                            />
                        </div>
                    )}

                    {/* Ambiguous Characters Toggle */}
                    <div className="flex items-center justify-between">
                        <Label htmlFor="ambiguous">
                            Exclude Ambiguous Characters
                        </Label>
                        <Switch
                            id="ambiguous"
                            checked={settings.excludeAmbiguous}
                            onCheckedChange={(checked) => {
                                onUpdateSetting('excludeAmbiguous', checked);
                            }}
                        />
                    </div>
                </motion.div>
            </motion.div>
        </motion.div>
    );
});

SettingsPanel.displayName = 'SettingsPanel';

export const PasswordGenerator = memo(({ 
    onPasswordGenerated, 
    className 
}: PasswordGeneratorProps) => {
    const {
        settings,
        controls,
        generatePassword,
        updateSetting,
        isSettingsValid,
        clearError,
        setShowSettings,
    } = usePasswordGenerator();
    
    const { 
        calculateMinimumLength,
        getStrengthColor,
        getStrengthLabel 
    } = usePasswordValidation();
    
    const minLength = settings ? calculateMinimumLength(settings) : ABSOLUTE_MIN_LENGTH;

    useEffect(() => {
        if (settings) {
            const requiredLength = calculateMinimumLength(settings);
            if (settings.length < requiredLength) {
                updateSetting('length', requiredLength);
            }
        }
    }, [settings?.minNumbers, settings?.minSymbols, settings?.useNumbers, settings?.useSymbols, settings?.useLowercase, settings?.useUppercase]);

    const handleGenerateClick = useCallback(async () => {
        try {
            clearError();
            const newPassword = await generatePassword();
            onPasswordGenerated(newPassword);
        } catch (error) {
            console.error('Failed to generate password:', error);
        }
    }, [clearError, generatePassword, onPasswordGenerated]);

    const handleDismissError = useCallback(() => clearError(), [clearError]);

    const handleSettingsToggle = useCallback(() => {
        setShowSettings(!settings?.showSettings);
        controls.start(settings?.showSettings ? "closed" : "open");
    }, [settings?.showSettings, setShowSettings, controls]);

    if (!settings) return null;

    return (
        <motion.div
            variants={passwordGeneratorAnimations.containerVariants}
            initial="hidden"
            animate="visible"
            exit="exit"
            className={className}
        >
            <div className="space-y-4 rounded-lg border p-4 shadow-sm">
                <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                        <div className="p-2 rounded-full bg-secondary/70">
                            <Shield className="h-5 w-5 text-secondary-foreground" />
                        </div>
                        <h3 className="font-semibold">Password Generator</h3>
                    </div>
                    <Button
                        variant="ghost"
                        size="sm"
                        type="button"
                        onClick={handleSettingsToggle}
                        className="gap-2"
                    >
                        Settings
                        <motion.div
                            animate={{ rotate: settings?.showSettings ? 180 : 0 }}
                            transition={{ duration: 0.2 }}
                        >
                            <ChevronDown className="h-4 w-4" />
                        </motion.div>
                    </Button>
                </div>

                <AnimatePresence mode="wait">
                    {settings?.error && (
                        <motion.div
                            initial={{ opacity: 0, height: 0 }}
                            animate={{ opacity: 1, height: 'auto' }}
                            exit={{ opacity: 0, height: 0 }}
                        >
                            <Alert 
                                variant="destructive" 
                                className="mt-2"
                                onClick={handleDismissError}
                                role="button"
                            >
                                <AlertCircle className="h-4 w-4" />
                                <AlertDescription>{settings.error}</AlertDescription>
                            </Alert>
                        </motion.div>
                    )}
                </AnimatePresence>

                <StrengthIndicator 
                    password={settings.lastGeneratedPassword}
                    entropy={settings.entropy}
                    getStrengthLabel={getStrengthLabel}
                    getStrengthColor={getStrengthColor}
                />

                <AnimatePresence>
                    {settings?.showSettings && (
                        <SettingsPanel 
                            settings={settings}
                            minLength={minLength}
                            onUpdateSetting={updateSetting}
                        />
                    )}
                </AnimatePresence>

                <Button
                    onClick={handleGenerateClick}
                    type="button" 
                    disabled={!isSettingsValid || settings.isGenerating}
                    className="w-full"
                >
                    {settings.isGenerating ? (
                        <>
                            <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                            Generating...
                        </>
                    ) : (
                        <>
                            <RefreshCw className="mr-2 h-4 w-4" />
                            Generate Password
                        </>
                    )}
                </Button>
            </div>
        </motion.div>
    );
});

PasswordGenerator.displayName = 'PasswordGenerator';