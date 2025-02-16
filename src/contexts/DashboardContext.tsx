// src/contexts/DashboardContext.tsx

import { createContext, useContext, useEffect, useState, useCallback } from 'react';
import { useSearchParams } from '@/hooks/dashboard/useSearchParams';
import { useEntryData } from '@/hooks/dashboard/useEntryData';
import { UUID, VaultStatus } from '@/api/apiTypes';
import { deleteEntry as deleteEntryApi } from '@/api/entryApi';
import { Toaster } from '@/components/ui/toaster';
import { useToast } from '@/hooks/use-toast';
import { getVaultStatus } from '@/api/vaultApi';
import { DashboardContextValue } from '@/types/dashboard.types';
import { useCategoryList } from '@/hooks/dashboard/useCategoryList';

const DashboardContext = createContext<DashboardContextValue | undefined>(undefined);

export function DashboardProvider({ children }: { children: React.ReactNode }) {
  const DashboardContent: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const { toast } = useToast();
    const {
      searchParams,
      debouncedSearchText,
      setSearchText,
      setCategoryName,
      setFavoritesOnly,
      resetSearch
    } = useSearchParams();

    const {
      entries,
      totalCount,
      maxCount,
      categoryDistribution,
      availableCategories,
      isLoading,
      error,
      fetchEntries,
      updateEntryList
    } = useEntryData();

    // State for tracking entries being deleted
    const [pendingDeletions, setPendingDeletions] = useState<Set<UUID>>(new Set());

    // State for sidebar collapse
    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);

    // State for vault status
    const [vaultStatus, setVaultStatus] = useState<VaultStatus | null>(null);

    // State for tracking which category is being modified
    const [selectedCategoryId, setSelectedCategoryId] = useState<UUID>('');
    const [selectedCategoryName, setSelectedCategoryName] = useState('');

    // Initialize category actions with the selected category
    const {
      handleDeleteCategory,
      handleRenameCategory,
      isDeletingCategory,
      isRenamingCategory,
      isRenameDialogOpen,
      setIsRenameDialogOpen,
      newName,
      setNewName,
      onRenameSubmit,
      onRenameCancel
    } = useCategoryList({
      categoryId: selectedCategoryId,
      categoryName: selectedCategoryName,
      refreshEntries: async () => {
        await fetchEntries(searchParams);
      },
      currentCategoryId: searchParams.categoryName
    });

    // Refresh on mount
    useEffect(() => {
      console.log('🔄 DashboardProvider mounted - fetching initial data');
      const fetchData = async () => {
        try {
          await fetchEntries({
            text: searchParams.text,
            categoryName: searchParams.categoryName,
            favoritesOnly: searchParams.favoritesOnly
          });
        } catch (error) {
          console.error('Failed to fetch initial data:', error);
        }
      };
      fetchData();
    }, []); // Only run on mount

    // Update search parameters effect to include favorites
    useEffect(() => {
      console.log('🔍 Search parameters changed:', searchParams);
      const fetchWithDelay = setTimeout(() => {
        fetchEntries({
          text: searchParams.text,
          categoryName: searchParams.categoryName,
          favoritesOnly: searchParams.favoritesOnly
        });
      }, searchParams.text ? 300 : 0);

      return () => clearTimeout(fetchWithDelay);
    }, [searchParams, fetchEntries]);

    // Function to manually refresh entries while keeping current filters
    const refreshEntries = useCallback(async () => {
      await fetchEntries(searchParams);
    }, [searchParams, fetchEntries]);

    // Enhanced delete handler with optimistic updates and category distribution refresh
    const deleteEntry = useCallback(async (id: UUID) => {
      setPendingDeletions(prev => new Set([...prev, id]));
      const previousEntries = entries;
      
      // Optimistic update for entries
      const updatedEntries = entries.filter(entry => entry.id !== id);
      updateEntryList(updatedEntries);

      try {
        await deleteEntryApi(id);
        
        toast({
          title: "Entry Deleted",
          description: "The entry was successfully deleted.",
          variant: "default",
        });
        
        // Refresh to update category distribution
        await refreshEntries();
      } catch (error) {
        // Revert optimistic update on error
        updateEntryList(previousEntries);
        
        toast({
          title: "Delete Failed",
          description: "Failed to delete the entry. Please try again.",
          variant: "destructive",
        });
        
        console.error('Failed to delete entry:', error);
      } finally {
        setPendingDeletions(prev => {
          const next = new Set(prev);
          next.delete(id);
          return next;
        });
      }
    }, [entries, refreshEntries, updateEntryList, toast]);

    const isDeletingEntry = useCallback((id: UUID) => {
      return pendingDeletions.has(id);
    }, [pendingDeletions]);

    // Vault status check
    useEffect(() => {
      const checkVaultStatus = async () => {
        try {
          const status = await getVaultStatus();
          setVaultStatus(status);
        } catch (error) {
          console.error('Failed to get vault status:', error);
        }
      };

      checkVaultStatus();
      const interval = setInterval(checkVaultStatus, 30000);
      return () => clearInterval(interval);
    }, []);

    const value: DashboardContextValue = {
      // Search state
      searchParams,
      debouncedSearchText,
      setSearchText,
      setCategoryName,
      setFavoritesOnly,
      
      // Entry data from search
      entries,
      totalCount,
      maxCount,
      categoryDistribution,
      availableCategories,
      
      // Loading states
      isLoading,
      error,
      
      // Entry operations
      deleteEntry,
      isDeletingEntry,
      
      // Search management
      resetSearch,
      refreshEntries,

      // Sidebar state
      isSidebarCollapsed,
      setIsSidebarCollapsed,

      // Vault status
      vaultStatus,

      // Category management
      handleDeleteCategory,
      handleRenameCategory,
      isDeletingCategory,
      isRenamingCategory,
      isRenameDialogOpen,
      setIsRenameDialogOpen,
      newName,
      setNewName,
      onRenameSubmit,
      onRenameCancel,

      setSelectedCategoryId,
      setSelectedCategoryName,
      selectedCategoryId,
      selectedCategoryName,
    };

    return (
      <DashboardContext.Provider value={value}>
        {children}
      </DashboardContext.Provider>
    );
  };

  return (
    <>
      <Toaster />
      <DashboardContent children={children} />
    </>
  );
}

export function useDashboardContext() {
  const context = useContext(DashboardContext);
  if (context === undefined) {
    throw new Error('useDashboard must be used within a DashboardProvider');
  }
  return context;
}