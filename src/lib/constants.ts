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

export const initialSettings: PasswordGeneratorSettings = {
    length: 16,
    useLowercase: true,
    useUppercase: true,
    useNumbers: true,
    useSymbols: true,
    minNumbers: 1,
    minSymbols: 1,
    excludeAmbiguous: true
};

export const ABSOLUTE_MIN_LENGTH = 5;
export const ABSOLUTE_MAX_LENGTH = 256;