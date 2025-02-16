// src/components/entry/EntryFormFields.tsx

import { useRef, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Switch } from '@/components/ui/switch';
import { ComboBox } from '@/components/ComboBox';
import { 
    ChevronDown,
    Star, 
    Link2, 
    User, 
    FileText, 
    Key,
    Eye,
    EyeOff,
    Copy,
    Check,
    RefreshCw
} from 'lucide-react';
import { PasswordGenerator } from './PasswordGenerator';
import { cn } from '@/lib/utils';
import { entryFormFieldAnimations } from '@/lib/entry.constants';
import { EntryFormFieldsProps } from '@/types/entry.types';
import { usePasswordGenerator } from '@/hooks/entry/usePasswordGenerator';


export default function EntryFormFields({
    formData = {},
    onInputChange,
    categories,
    onAddCategory,
    isSubmitting = false
}: EntryFormFieldsProps) {
    // Local state
    const [showPassword, setShowPassword] = useState(false);
    const [isPasswordGeneratorOpen, setIsPasswordGeneratorOpen] = useState(false);
    const [copied, setCopied] = useState(false);
    
    // References
    const passwordInputRef = useRef<HTMLInputElement>(null);
    
    // Hooks
    const passwordGenerator = usePasswordGenerator();

    // Handle password generation
    const handlePasswordGenerated = async () => {
        try {
            const password = await passwordGenerator.generatePassword();
            onInputChange('password', password);
            // Focus the password input after generation
            passwordInputRef.current?.focus();
        } catch (error) {
            console.error('Failed to generate password:', error);
        }
    };

    // Handle copy functionality
    const handleCopy = async () => {
        if (formData?.password) {
            await navigator.clipboard.writeText(formData.password);
            setCopied(true);
            setTimeout(() => setCopied(false), 2000);
        }
    };

    // Handle category selection or creation
    const handleCategoryChange = async (value: string) => {
        // Just pass the value directly to onInputChange
        onInputChange('categoryName', value);
    };

    // Toggle password generator panel
    const togglePasswordGenerator = () => {
        setIsPasswordGeneratorOpen(!isPasswordGeneratorOpen);
    };

    const categoryNames = categories.map(cat => cat.name);

    return (
        <div className="space-y-8">
            {/* Title Field */}
            <motion.div 
                className="space-y-2"
                variants={entryFormFieldAnimations.fieldVariants}
                initial="hidden"
                animate="visible"
                custom={0}
            >
                <Label htmlFor="title" className="required">Title</Label>
                <Input
                    id="title"
                    value={formData?.title ?? ''}
                    onChange={e => onInputChange('title', e.target.value)}
                    placeholder="Entry title"
                    disabled={isSubmitting}
                    required
                />
            </motion.div>

            {/* Username Field */}
            <motion.div 
                className="space-y-2"
                variants={entryFormFieldAnimations.fieldVariants}
                initial="hidden"
                animate="visible"
                custom={1}
            >
                <Label htmlFor="username">Username</Label>
                <div className="relative">
                    <User className="absolute left-2.5 top-3.5 h-5 w-5 text-muted-foreground" />
                    <Input
                        id="username"
                        className="pl-9"
                        value={formData?.username ?? ''}
                        onChange={e => onInputChange('username', e.target.value)}
                        placeholder="Username or email"
                        disabled={isSubmitting}
                    />
                </div>
            </motion.div>

            {/* Password Field */}
            <motion.div 
                className="space-y-2"
                variants={entryFormFieldAnimations.fieldVariants}
                initial="hidden"
                animate="visible"
                custom={2}
            >
                <Label htmlFor="password">Password</Label>
                <div className="relative">
                    <Key className="absolute left-2.5 top-3.5 h-5 w-5 text-muted-foreground" />
                    <Input
                        id="password"
                        ref={passwordInputRef}
                        type={showPassword ? 'text' : 'password'}
                        className="pl-9 pr-28"
                        value={formData?.password ?? ''}
                        onChange={e => onInputChange('password', e.target.value)}
                        placeholder="Password"
                        disabled={isSubmitting}
                    />
                    <div className="absolute right-3 top-3 flex items-center gap-2">
                        <button
                            type="button"
                            onClick={handleCopy}
                            className="text-muted-foreground hover:text-foreground"
                            disabled={isSubmitting || !formData?.password}
                        >
                            {copied ? (
                                <Check className="h-6 w-6 text-green-500" />
                            ) : (
                                <Copy className="h-6 w-6" />
                            )}
                        </button>
                        <button
                            type="button"
                            onClick={handlePasswordGenerated}
                            className="text-muted-foreground hover:text-foreground"
                            disabled={isSubmitting}
                            title="Generate new password"
                        >
                            <RefreshCw className="h-6 w-6" />
                        </button>
                        <button
                            type="button"
                            onClick={() => setShowPassword(!showPassword)}
                            className="text-muted-foreground hover:text-foreground"
                            disabled={isSubmitting}
                        >
                            {showPassword ? (
                                <EyeOff className="h-6 w-6" />
                            ) : (
                                <Eye className="h-6 w-6" />
                            )}
                        </button>
                        <button
                            type="button"
                            onClick={togglePasswordGenerator}
                            className={cn(
                                "text-muted-foreground hover:text-foreground transition-transform",
                                isPasswordGeneratorOpen && "rotate-180"
                            )}
                            disabled={isSubmitting}
                        >
                            <ChevronDown className="h-6 w-6" />
                        </button>
                    </div>
                </div>

                {/* Password Generator Panel */}
                <AnimatePresence>
                    {isPasswordGeneratorOpen && (
                        <motion.div
                            initial={{ opacity: 0, height: 0 }}
                            animate={{ opacity: 1, height: 'auto' }}
                            exit={{ opacity: 0, height: 0 }}
                            transition={{ duration: 0.2 }}
                        >
                            <PasswordGenerator 
                                onPasswordGenerated={handlePasswordGenerated}
                                className="mt-4"
                            />
                        </motion.div>
                    )}
                </AnimatePresence>
            </motion.div>

            {/* URL Field */}
            <motion.div 
                className="space-y-2"
                variants={entryFormFieldAnimations.fieldVariants}
                initial="hidden"
                animate="visible"
                custom={3}
            >
                <Label htmlFor="url">URL</Label>
                <div className="relative">
                    <Link2 className="absolute left-2 top-4 h-5 w-5 text-muted-foreground" />
                    <Input
                        id="url"
                        className="pl-9"
                        value={formData?.url ?? ''}
                        onChange={e => onInputChange('url', e.target.value)}
                        placeholder="https://example.com"
                        type="url"
                        disabled={isSubmitting}
                    />
                </div>
            </motion.div>

            {/* Notes Field */}
            <motion.div 
                className="space-y-2"
                variants={entryFormFieldAnimations.fieldVariants}
                initial="hidden"
                animate="visible"
                custom={4}
            >
                <Label htmlFor="notes">Notes</Label>
                <div className="relative">
                    <FileText className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                    <Textarea
                        id="notes"
                        className="pl-9 min-h-[100px]"
                        value={formData?.notes ?? ''}
                        onChange={e => onInputChange('notes', e.target.value)}
                        placeholder="Additional notes"
                        disabled={isSubmitting}
                    />
                </div>
            </motion.div>

            {/* Category Selection */}
            <motion.div 
                className="space-y-2"
                variants={entryFormFieldAnimations.fieldVariants}
                initial="hidden"
                animate="visible"
                custom={5}
            >
                <Label htmlFor="category">Category</Label>
                <ComboBox
                    items={categoryNames as readonly string[]}
                    defaultValue={formData?.categoryName || ''}
                    onValueChange={handleCategoryChange}
                    placeholder="Select a category"
                    searchPlaceholder="Search categories..."
                    emptyMessage="No categories found."
                    onAddItem={onAddCategory}  // Pass the onAddCategory prop directly
                    className="w-full"
                />
            </motion.div>

            {/* Favorite Toggle */}
            <motion.div 
                className="flex items-center justify-between"
                variants={entryFormFieldAnimations.fieldVariants}
                initial="hidden"
                animate="visible"
                custom={6}
            >
                <div className="space-y-0.5">
                    <Label htmlFor="favorite">Favorite</Label>
                    <p className="text-sm text-muted-foreground">
                        Mark this entry as a favorite for quick access
                    </p>
                </div>
                <div className="flex items-center gap-2">
                    <Star className="h-4 w-4 text-warning" />
                    <Switch
                        id="favorite"
                        checked={formData?.favorite ?? false}
                        onCheckedChange={checked => onInputChange('favorite', checked)}
                        disabled={isSubmitting}
                    />
                </div>
            </motion.div>
        </div>
    );
}