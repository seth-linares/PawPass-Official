// src/hooks/useCategoryActions.ts

import { useState, useCallback } from 'react';
import { AppError, UUID } from '@/api/apiTypes';
import { deleteCategory, renameCategory } from '@/api/categoryApi';
import { useToast } from '@/hooks/use-toast';
import { UseCategoryActionsProps, UseCategoryActionsReturn } from '@/types/dashboard.types';

export function useCategoryList({
  categoryId,
  categoryName,
  refreshEntries,
  currentCategoryId
}: UseCategoryActionsProps): UseCategoryActionsReturn {
  // Track categories being modified
  const [categoriesBeingDeleted, setCategoriesBeingDeleted] = useState<Set<UUID>>(new Set());
  const [categoriesBeingRenamed, setCategoriesBeingRenamed] = useState<Set<UUID>>(new Set());
  
  const { toast } = useToast();

  // Helper to check if a specific category is being deleted
  const isDeletingCategory = useCallback((id: UUID) => {
    return categoriesBeingDeleted.has(id);
  }, [categoriesBeingDeleted]);

  // Helper to check if a specific category is being renamed
  const isRenamingCategory = useCallback((id: UUID) => {
    return categoriesBeingRenamed.has(id);
  }, [categoriesBeingRenamed]);

  // Handler for deleting a category
  const handleDeleteCategory = useCallback(async (id: UUID) => {
    // Don't allow multiple deletion attempts
    if (isDeletingCategory(id)) return;

    // Category for deletion set
    setCategoriesBeingDeleted(prev => new Set([...prev, id]));

    try {
      await deleteCategory(id);
      
      // If we deleted the current category, we should reset the filter
      if (currentCategoryId === id) {
        // This will be handled by the component
        toast({
          title: "Category Deleted",
          description: "Category filter has been reset.",
        });
      } else {
        toast({
          title: "Category Deleted",
          description: "The category was successfully deleted.",
        });
      }

      // Refresh the entries to update the category list
      await refreshEntries();
    } catch (error) {
      console.error('Failed to delete category:', error);
      toast({
        title: "Delete Failed",
        description: "Failed to delete the category. Please try again.",
        variant: "destructive",
      });
    } finally {
      // Remove category from deletion set
      setCategoriesBeingDeleted(prev => {
        const next = new Set(prev);
        next.delete(id);
        return next;
      });
    }
  }, [currentCategoryId, isDeletingCategory, refreshEntries, toast]);

  // Handler for renaming a category
  const handleRenameCategory = useCallback(async (id: UUID, newName: string) => {
    // Don't allow multiple rename attempts
    if (isRenamingCategory(id)) return;

    // Validate new name
    if (!newName.trim()) {
      toast({
        title: "Invalid Name",
        description: "Category name cannot be empty.",
        variant: "destructive",
      });
      return;
    }

    // Category for renaming set
    setCategoriesBeingRenamed(prev => new Set([...prev, id]));

    try {
      console.log('Renaming category:', id, newName);
      await renameCategory(id, newName);
      
      toast({
        title: "Category Renamed",
        description: "The category was successfully renamed.",
      });

      // Refresh the entries to update the category list
      await refreshEntries();
    } catch (error) {
      console.error('Failed to rename category:', error);

      toast({
        title: "Rename Failed",
        description: `${(error as AppError).payload?.message}`,
        variant: "destructive",
      });
    } finally {
      // Remove category from renaming set
      setCategoriesBeingRenamed(prev => {
        const next = new Set(prev);
        next.delete(id);
        return next;
      });
    }
  }, [isRenamingCategory, refreshEntries, toast]);

  // Dialog states
  const [isRenameDialogOpen, setIsRenameDialogOpen] = useState(false);
  const [newName, setNewName] = useState(categoryName);
  const [showDropdown, setShowDropdown] = useState(false);

  const onRenameSubmit = useCallback(async () => {
    console.log('onRenameSubmit:', categoryId, newName);
    await handleRenameCategory(categoryId, newName);
    setIsRenameDialogOpen(false);
    setNewName(categoryName);
  }, [categoryId, categoryName, handleRenameCategory, newName]);

  const onRenameCancel = useCallback(() => {
    setIsRenameDialogOpen(false);
    setNewName(categoryName);
  }, [categoryName]);

  return {
    // Overall loading states
    isDeleting: categoriesBeingDeleted.size > 0,
    isRenaming: categoriesBeingRenamed.size > 0,
    
    // Action handlers
    handleDeleteCategory,
    handleRenameCategory,
    
    // Individual category loading states
    isDeletingCategory,
    isRenamingCategory,

    // Dialog states
    isRenameDialogOpen,
    setIsRenameDialogOpen,
    newName,
    setNewName,
    showDropdown,
    setShowDropdown,
    onRenameSubmit,
    onRenameCancel
  };
}