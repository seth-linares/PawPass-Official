// src/components/dashboard/EntryOverview.tsx

import { useState, useMemo } from 'react';
import { useDashboardContext } from '@/contexts/DashboardContext';
import LoadingSpinner from '@/components/LoadingSpinner';
import { AlertCircle, Info } from 'lucide-react';
import { Alert, AlertDescription} from '@/components/ui/alert';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog';
import { UUID, EntryOverview as EntryOverviewType } from '@/api/apiTypes';
import EntryCard from './EntryCard';
import { memo } from 'react';

const EntryOverview: React.FC = () => {
  const [entryToDelete, setEntryToDelete] = useState<UUID | null>(null);

  const {
    entries,
    totalCount,
    isLoading,
    error,
    searchParams,
    isDeletingEntry,
    deleteEntry,
  } = useDashboardContext();

  // Handler for initiating delete
  const handleDeleteClick = (id: UUID) => {
    setEntryToDelete(id);
  };

  // Handler for confirming delete
  const handleConfirmDelete = async () => {
    if (entryToDelete) {
      await deleteEntry(entryToDelete);
      setEntryToDelete(null);
    }
  };

  // Handler for canceling delete
  const handleCancelDelete = () => {
    setEntryToDelete(null);
  };

  // Memoize the entry cards to prevent unnecessary re-renders
  const entryCards = useMemo(() => {
    return entries.map((entry: EntryOverviewType) => (
      <EntryCard
        key={entry.id}
        entry={entry}
        isDeleting={isDeletingEntry(entry.id)}
        onDeleteClick={handleDeleteClick}
      />
    ));
  }, [entries, isDeletingEntry]); // Only re-render when entries or deletion status changes

  if (isLoading) {
    // Prevent loading indicator flash for quick updates
    const isQuickUpdate = entries.length > 0;
    return isQuickUpdate ? (
      <div className="opacity-50 pointer-events-none">
        {entryCards}
      </div>
    ) : (
      <div className="flex items-center justify-center p-8">
        <LoadingSpinner size="lg" label="Loading entries..." />
      </div>
    );
  }

  if (error) {
    return (
      <Alert variant="destructive">
        <AlertCircle className="h-5 w-5" />
        <AlertDescription>{error.message}</AlertDescription>
      </Alert>
    );
  }

  if (entries.length === 0) {
    return (
      <div className="alert alert-info shadow-lg mx-4 mt-4 flex justify-center">
        <div className="flex flex-col items-center text-center">
          <div className="flex items-center gap-2">
            <Info className="h-6 w-6" />
            <h3 className="font-bold">No entries found</h3>
          </div>
          <div className="text-sm">
            {searchParams.text || searchParams.categoryName || searchParams.favoritesOnly
              ? "Try adjusting your search criteria"
              : "Create your first entry to get started"}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <div className="p-4 border-b border-base-200">
        <div className="badge badge-lg badge-primary">
          {totalCount} {totalCount === 1 ? 'entry' : 'entries'} found
        </div>
      </div>
      
      <div className="overflow-y-auto p-4">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {entryCards}
        </div>
      </div>

      <AlertDialog open={!!entryToDelete} onOpenChange={() => setEntryToDelete(null)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Entry</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this entry? This action cannot be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel onClick={handleCancelDelete}>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={handleConfirmDelete} className="bg-destructive text-destructive-foreground hover:bg-destructive/90">
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
};

// Memoize the entire component
export default memo(EntryOverview);