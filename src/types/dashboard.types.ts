import { EntryOverview, CategoryCount, VaultStatus } from "@/api/apiTypes";
import { UUID } from "@/api/apiTypes";

export interface DashboardContextValue {
    // Search parameters - now matches our SearchParameters interface
    searchParams: SearchParameters;
    debouncedSearchText: string | undefined;
    setSearchText: (text: string) => void;
    setCategoryName: (category?: string) => void;
    setFavoritesOnly: (favorites: boolean) => void;

    // Entry data - includes full search results
    entries: EntryOverview[];
    totalCount: number;
    maxCount: number;
    categoryDistribution: CategoryCount[];
    availableCategories: CategoryCount[];

    // Loading and error states
    isLoading: boolean;
    error: Error | null;

    // Entry operations
    deleteEntry: (id: UUID) => Promise<void>;
    isDeletingEntry: (id: UUID) => boolean;

    // Search management
    resetSearch: () => void;
    refreshEntries: () => Promise<void>;

    // Sidebar state
    isSidebarCollapsed: boolean;
    setIsSidebarCollapsed: (collapsed: boolean) => void;

    // Vault status
    vaultStatus: VaultStatus | null;

    // Category management
    handleDeleteCategory: (id: UUID) => Promise<void>;
    handleRenameCategory: (id: UUID, newName: string) => Promise<void>;
    isDeletingCategory: (id: UUID) => boolean;
    isRenamingCategory: (id: UUID) => boolean;
    isRenameDialogOpen: boolean;
    setIsRenameDialogOpen: (open: boolean) => void;
    newName: string;
    setNewName: (name: string) => void;
    onRenameSubmit: () => Promise<void>;
    onRenameCancel: () => void;

    setSelectedCategoryId: (id: UUID) => void;
    setSelectedCategoryName: (name: string) => void;
    selectedCategoryId: UUID;
    selectedCategoryName: string;
}

export interface SearchParameters {
    text?: string;
    categoryName?: string;
    favoritesOnly: boolean;
}

export interface UseSearchParamsReturn {
    searchParams: SearchParameters;
    debouncedSearchText: string | undefined;
    setSearchText: (text: string) => void;
    setCategoryName: (category?: string) => void;
    setFavoritesOnly: (favorites: boolean) => void;
    resetSearch: () => void;
}

// Define our state interface to match the enhanced search results
    export interface EntryDataState {
    entries: EntryOverview[];
    totalCount: number;
    maxCount: number;
    categoryDistribution: CategoryCount[];
    availableCategories: CategoryCount[];
    isLoading: boolean;
    error: Error | null;
}

export interface UseEntryDataReturn {
    entries: EntryOverview[];
    totalCount: number;
    maxCount: number;
    categoryDistribution: CategoryCount[];
    availableCategories: CategoryCount[];
    isLoading: boolean;
    error: Error | null;
    fetchEntries: (params: SearchParameters) => Promise<void>;
    updateEntryList: (entries: EntryOverview[]) => void;
}

export interface DeletionState {
    pendingDeletions: Set<UUID>;
    isDeleting: boolean;
    deletionError: Error | null;
  }
  
export interface UseEntryDeletionProps {
    entries: EntryOverview[];
    fetchEntries: (params: SearchParameters) => Promise<void>;
    updateEntryList: (entries: EntryOverview[]) => void;
    searchParams: SearchParameters;
}

export interface UseEntryDeletionReturn {
    isDeletingEntry: (id: UUID) => boolean;
    deletionError: Error | null;
    handleDeleteEntry: (id: UUID) => Promise<void>;
    cancelPendingDeletions: () => void;
}

export interface UseCategoryActionsProps {
    categoryId: UUID;
    categoryName: string;
    refreshEntries: () => Promise<void>;
    currentCategoryId?: UUID;
}

export interface UseCategoryActionsReturn {
    // Loading states
    isDeleting: boolean;
    isRenaming: boolean;
    
    // Action handlers
    handleDeleteCategory: (id: UUID) => Promise<void>;
    handleRenameCategory: (id: UUID, newName: string) => Promise<void>;
    
    // Individual category loading states
    isDeletingCategory: (id: UUID) => boolean;
    isRenamingCategory: (id: UUID) => boolean;

    // Dialog states
    isRenameDialogOpen: boolean;
    setIsRenameDialogOpen: (open: boolean) => void;
    newName: string;
    setNewName: (name: string) => void;
    
    // Dropdown state
    showDropdown: boolean;
    setShowDropdown: (show: boolean) => void;
    
    // Handlers
    onRenameSubmit: () => Promise<void>;
    onRenameCancel: () => void;
}

