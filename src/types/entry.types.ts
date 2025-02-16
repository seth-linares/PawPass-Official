import { AnimationControls } from "framer-motion";
import { Category, DecryptedEntry, EntryData, PasswordGeneratorSettings } from "@/api/apiTypes";

export interface FormState {
    formData: EntryData;
    categories: Category[];
    isSubmitting: boolean;
    isLoadingCategories: boolean;
    error: string | null;
}

export interface EntryFormState {
    formData: Partial<EntryData> | null;
    categories: Category[];
    isLoading: boolean;
    isSubmitting: boolean;
    error: string | null;
    handleSubmit: () => Promise<void>;
    clearError: () => void;
    handleInputChange: (field: keyof EntryData, value: string | boolean) => void;
    handleCategoryCreate?: (category: string) => Promise<void>;
}

export interface EditorState {
    entry: DecryptedEntry | null;
    originalEntry: DecryptedEntry | null;
    isLoading: boolean;
    isSaving: boolean;
    error: string | null;
}

export interface UseEntryEditorReturn extends Omit<EditorState, 'originalEntry'> {
    updateField: (field: keyof EntryData, value: string | boolean) => void;
    saveEntry: () => Promise<void>;
    isDirty: boolean;
    resetEntry: () => void;
    clearError: () => void;
}

export interface PasswordGeneratorState extends PasswordGeneratorSettings {
    isOpen: boolean;
    isLoading: boolean;
    isGenerating: boolean;
    error: string | null;
    lastGeneratedPassword: string | null;
    entropy: number | null;
    showSettings: boolean;
    copied: boolean;
}

export interface UsePasswordGeneratorReturn {
    settings: PasswordGeneratorState | null;
    controls: AnimationControls;
    generatePassword: () => Promise<string>;
    updateSetting: <P extends keyof PasswordGeneratorSettings>(
        setting: P,
        value: PasswordGeneratorSettings[P]
    ) => Promise<void>;
    togglePanel: () => void;
    isSettingsValid: boolean;
    clearError: () => void;
    setShowSettings: (show: boolean) => void;
    setCopied: (copied: boolean) => void;
}

export interface PasswordGeneratorProps {
    onPasswordGenerated: (password: string) => void;
    className?: string;
}

// Define props interface for the component
export interface EntryFormFieldsProps {
    formData: Partial<EntryData> | null;
    onInputChange: (field: keyof EntryData, value: string | boolean) => void;
    categories: Category[];
    onAddCategory?: (category: string) => Promise<void>;
    isSubmitting?: boolean;
}

export interface SettingsPanelProps {
    settings: PasswordGeneratorState;
    minLength: number;
    onUpdateSetting: <K extends keyof PasswordGeneratorSettings>(
        setting: K,
        value: PasswordGeneratorSettings[K]
    ) => Promise<void>;
}

export interface StrengthIndicatorProps {
    password: string | null;
    entropy: number | null;
    getStrengthLabel: (entropy: number | null) => string;
    getStrengthColor: (entropy: number | null) => string;
}
