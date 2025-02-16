import { EntryData, PasswordGeneratorSettings } from "@/api/apiTypes";

export const initialFormData: EntryData = {
    title: '',
    username: '',
    password: '',
    url: '',
    notes: '',
    categoryName: undefined,
    favorite: false,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
};

export const initialPasswordSettings: PasswordGeneratorSettings = {
    length: 16,
    useLowercase: true,
    useUppercase: true,
    useNumbers: true,
    useSymbols: true,
    minNumbers: 2,
    minSymbols: 2,
    excludeAmbiguous: true
};

export const entryFormAnimations = {
    pageVariants: {
        initial: { opacity: 0, y: 20 },
        animate: { opacity: 1, y: 0 },
        exit: { opacity: 0, y: -20 }
    }
} as const;

export const passwordGeneratorAnimations = {
    settingsPanelVariants: {
        hidden: { opacity: 0, height: 0 },
        visible: { 
            opacity: 1, 
            height: "auto",
            transition: { 
                duration: 0.3,
                when: "beforeChildren",
                staggerChildren: 0.1 
            }
        }
    },
    containerVariants: {
        hidden: { opacity: 0, height: 0 },
        visible: { 
            opacity: 1, 
            height: 'auto',
            transition: { duration: 0.3 }
        },
        exit: { 
            opacity: 0, 
            height: 0,
            transition: { duration: 0.2 }
        }
    },
    settingsVariants: {
        hidden: { opacity: 0, y: -20 },
        visible: { 
            opacity: 1, 
            y: 0,
            transition: { 
                duration: 0.3,
                staggerChildren: 0.1 
            }
        }
    },
    itemVariants: {
        hidden: { opacity: 0, x: -20 },
        visible: { opacity: 1, x: 0 }
    }
} as const;

export const entryFormFieldAnimations = {
    fieldVariants: {
        hidden: { opacity: 0, y: 20 },
        visible: (i: number) => ({
            opacity: 1,
            y: 0,
            transition: {
                delay: i * 0.1,
                duration: 0.3
            }
        })
    }
} as const;

export const relevantEntryFields: (keyof EntryData)[] = [
    'title', 
    'username', 
    'password', 
    'url', 
    'notes', 
    'categoryName', 
    'favorite'
] as const;

export const strengthLabels = {
    none: 'No Password',
    weak: 'Weak',
    medium: 'Medium',
    strong: 'Strong',
    veryStrong: 'Very Strong'
} as const;

export const strengthColors = {
    none: 'bg-gray-200',
    weak: 'bg-red-500',
    medium: 'bg-yellow-500',
    strong: 'bg-green-500'
} as const;
