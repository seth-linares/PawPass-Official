export type UUID = string;
export type ISO_8601 = string;

export interface EntryOverview {
    id: UUID;
    title: string;
    username?: string;
    url?: string;
    categoryId?: UUID;
    categoryName?: string;
    favorite: boolean;
    createdAt: ISO_8601;  
    updatedAt: ISO_8601;
}

export interface DecryptedEntry {
    id: UUID;
    title: string;
    username?: string;
    url?: string;
    categoryId?: UUID;
    categoryName?: string;
    favorite: boolean;
    createdAt: ISO_8601;
    updatedAt: ISO_8601;
    password?: string;
    notes?: string;
}

export interface EntryData {
    title: string;
    username?: string;
    password?: string;
    url?: string;
    notes?: string;
    categoryName?: string;
    favorite: boolean;
    createdAt: ISO_8601;
    updatedAt: ISO_8601;
}

export interface Category {
    id: UUID;
    name: string;
    entryCount: number;
}

export interface SearchQuery {
    text?: string;
    categoryName?: string;
    favoritesOnly: boolean;
}

export interface KeyDerivation {
    memoryCost: number;
    timeCost: number;
    parallelism: number;
}

export interface VaultPaths {
    vaultPath: string;
    backupDir: string;
    tempDir: string;
}

export interface PasswordGeneratorSettings {
    length: number;
    useLowercase: boolean;
    useUppercase: boolean;
    useNumbers: boolean;
    useSymbols: boolean;
    minNumbers: number;
    minSymbols: number;
    excludeAmbiguous: boolean;
}

export interface VaultStatus {
    sessionActive: boolean;
    keyHierarchyPresent: boolean;
    vaultManagerPresent: boolean;
}

export interface AppErrorPayload {
  code: string;
  message: string;
  details?: string;
}

export interface AppError {
  kind: string;
  payload: AppErrorPayload;
}

export interface CategoryCount {
    id: UUID;
    name: string;
    entryCount: number;
}

export interface EnhancedSearchResults {
    entries: EntryOverview[];
    totalCount: number;
    maxCount: number; 
    categoryDistribution: CategoryCount[];
}