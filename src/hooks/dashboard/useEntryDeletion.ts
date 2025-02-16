// src/hooks/useEntryDeletion.ts

import { useState, useCallback } from 'react';
import { UUID } from '@/api/apiTypes';
import { deleteEntry } from '@/api/entryApi';
import { useToast } from '../use-toast';
import { UseEntryDeletionProps, UseEntryDeletionReturn, DeletionState } from '@/types/dashboard.types';


export function useEntryDeletion({
  entries,
  fetchEntries,
  updateEntryList,
  searchParams,
}: UseEntryDeletionProps): UseEntryDeletionReturn {
  const { toast } = useToast();
  const [deletionState, setDeletionState] = useState<DeletionState>({
    pendingDeletions: new Set<UUID>(),
    isDeleting: false,
    deletionError: null,
  });

  const handleDeleteEntry = useCallback(async (id: UUID) => {
    setDeletionState(prev => ({
      ...prev,
      pendingDeletions: new Set([...prev.pendingDeletions, id]),
      isDeleting: true,
      deletionError: null,
    }));

    try {
      // Optimistic update
      const updatedEntries = entries.filter(entry => entry.id !== id);
      updateEntryList(updatedEntries);

      // Perform deletion
      await deleteEntry(id);

      toast({
        title: "Entry Deleted",
        description: "The entry was successfully deleted.",
        variant: "default",
      });
    } catch (error) {
      console.error('Failed to delete entry:', error);
      
      // Rollback on error
      await fetchEntries(searchParams);
      
      toast({
        title: "Delete Failed",
        description: "Failed to delete the entry. Please try again.",
        variant: "destructive",
      });

      setDeletionState(prev => ({
        ...prev,
        deletionError: error instanceof Error ? error : new Error('Failed to delete entry'),
      }));
    } finally {
      setDeletionState(prev => ({
        ...prev,
        pendingDeletions: new Set([...prev.pendingDeletions].filter(x => x !== id)),
        isDeleting: false,
      }));
    }
  }, [entries, fetchEntries, searchParams, toast, updateEntryList]);

  const isDeletingEntry = useCallback((id: UUID) => {
    return deletionState.pendingDeletions.has(id);
  }, [deletionState.pendingDeletions]);

  const cancelPendingDeletions = useCallback(() => {
    setDeletionState({
      pendingDeletions: new Set(),
      isDeleting: false,
      deletionError: null,
    });
    fetchEntries(searchParams);
  }, [fetchEntries, searchParams]);

  return {
    isDeletingEntry,
    deletionError: deletionState.deletionError,
    handleDeleteEntry,
    cancelPendingDeletions,
  };
}