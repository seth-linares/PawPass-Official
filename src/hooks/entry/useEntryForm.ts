// src/hooks/useEntryForm.ts

import { useState, useCallback, useRef, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { createEntry } from '@/api/entryApi';
import { searchCategories, createCategory } from '@/api/categoryApi';
import { EntryData, AppError } from '@/api/apiTypes';
import { useDashboardContext } from '@/contexts/DashboardContext';
import { useToast } from '@/hooks/use-toast';
import { FormState } from '@/types/entry.types';  // Update import
import { initialFormData } from '@/lib/constants';


export function useEntryForm() {
    const [state, setState] = useState<FormState>({
        formData: initialFormData,
        categories: [],
        isSubmitting: false,
        isLoadingCategories: false,
        error: null
    });

    const navigate = useNavigate();
    const { refreshEntries } = useDashboardContext();
    const { toast } = useToast();
    
    // Use ref for category search timer
    const categorySearchTimer = useRef<NodeJS.Timeout>();
    const isCategoriesLoaded = useRef(false);
    

    const updateField = useCallback((field: keyof EntryData, value: string | boolean) => {
        setState(prev => ({
            ...prev,
            formData: {
                ...prev.formData,
                [field]: value,
                updatedAt: new Date().toISOString()
            },
            error: null
        }));
    }, []);

    const loadCategories = useCallback(async () => {
        if (isCategoriesLoaded.current) return;

        setState(prev => ({ ...prev, isLoadingCategories: true }));
        try {
            const loadedCategories = await searchCategories('');
            setState(prev => ({
                ...prev,
                categories: loadedCategories,
                isLoadingCategories: false
            }));
            isCategoriesLoaded.current = true;
        } catch (err) {
            console.error('❌ Failed to load categories:', err);
            const errorMessage = err instanceof Error ? err.message :
                               (err as AppError)?.payload?.message ?? 'Failed to load categories';
            setState(prev => ({
                ...prev,
                error: errorMessage,
                isLoadingCategories: false
            }));
        }
    }, []);

    const handleCategoryCreate = useCallback(async (categoryName: string) => {
        try {
            await createCategory(categoryName);
            const updatedCategories = await searchCategories('');
            setState(prev => ({
                ...prev,
                categories: updatedCategories,
                formData: {
                    ...prev.formData,
                    categoryName
                }
            }));

            toast({
                title: "Success",
                description: "Category created successfully",
                variant: "default",
            });
        } catch (err) {
            console.error('❌ Failed to create category:', err);
            const errorMessage = err instanceof Error ? err.message :
                               (err as AppError)?.payload?.message ?? 'Failed to create category';
            setState(prev => ({ ...prev, error: errorMessage }));
            throw err;
        }
    }, [toast]);

    const handleAddCategory = async (newCategory: string) => {
        try {
            await createCategory(newCategory);
            // Refresh categories list after adding new one
            const updatedCategories = await searchCategories('');
            setState(prev => ({
                ...prev,
                categories: updatedCategories,
                formData: {
                    ...prev.formData,
                    categoryName: newCategory
                }
            }));
        } catch (err) {
            console.error('Failed to create category:', err);
            if (err instanceof Error) {
                setState(prev => ({ ...prev, error: err.message }));
            } else {
                const appError = err as AppError;
                setState(prev => ({ ...prev, error: appError.payload?.message ?? 'Failed to create category' }));
            }
        }
    };

    const handleSubmit = useCallback(async () => {
        setState(prev => ({ ...prev, isSubmitting: true, error: null }));

        try {
            if (!state.formData.title.trim()) {
                throw new Error('Title is required');
            }

            await createEntry(state.formData);
            await refreshEntries();

            toast({
                title: "Success",
                description: "Entry created successfully",
                variant: "default",
            });

            navigate('/vault');
        } catch (err) {
            console.error('❌ Failed to create entry:', err);
            const errorMessage = err instanceof Error ? err.message :
                               (err as AppError)?.payload?.message ?? 'Failed to create entry';
            setState(prev => ({
                ...prev,
                error: errorMessage,
                isSubmitting: false
            }));
            throw err;
        }
    }, [state.formData, navigate, refreshEntries, toast]);

    const resetForm = useCallback(() => {
        setState(prev => ({
            ...prev,
            formData: initialFormData,
            error: null
        }));
    }, []);

    // Move the effect into the hook
    useEffect(() => {
        loadCategories();
    }, []); // Depends on loadCategories which is memoized

    // Cleanup effect
    useEffect(() => {
        return () => {
            if (categorySearchTimer.current) {
                clearTimeout(categorySearchTimer.current);
            }
            isCategoriesLoaded.current = false;
        };
    }, []);

    return {
        formData: state.formData,
        categories: state.categories,
        isSubmitting: state.isSubmitting,
        isLoadingCategories: state.isLoadingCategories,
        error: state.error,
        updateField,
        handleSubmit,
        resetForm,
        loadCategories,
        handleCategoryCreate,
        handleAddCategory  // Make sure to include this in the return object
    };
}