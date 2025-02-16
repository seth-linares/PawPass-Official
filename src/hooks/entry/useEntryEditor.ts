// src/hooks/useEntryEditor.ts

import { useState, useEffect, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { getDecryptedEntry, updateEntry } from '@/api/entryApi';
import { DecryptedEntry, EntryData, UUID, AppError } from '@/api/apiTypes';
import { useDashboardContext } from '@/contexts/DashboardContext';
import { useToast } from '@/hooks/use-toast';
import { EditorState, UseEntryEditorReturn } from '@/types/entry.types';  // Update import


/**
 * Custom hook for managing entry editing functionality
 * @param entryId - UUID of the entry to edit
 */
export function useEntryEditor(entryId: UUID): UseEntryEditorReturn {
    // Initialize state
    const [state, setState] = useState<EditorState>({
        entry: null,
        originalEntry: null,
        isLoading: true,
        isSaving: false,
        error: null
    });

    // Hooks
    const navigate = useNavigate();
    const { refreshEntries } = useDashboardContext();
    const { toast } = useToast();

    // Load entry data
    useEffect(() => {
        let isMounted = true;

        const loadEntry = async () => {
            try {
                const entryData = await getDecryptedEntry(entryId);
                
                if (isMounted) {
                    setState(prev => ({
                        ...prev,
                        entry: entryData,
                        originalEntry: entryData,
                        isLoading: false
                    }));
                }
            } catch (err) {
                console.error('❌ Failed to load entry:', err);
                if (isMounted) {
                    if (err instanceof Error) {
                        setState(prev => ({
                            ...prev,
                            error: err.message,
                            isLoading: false
                        }));
                    } else if (typeof err === 'object' && err !== null) {
                        const appError = err as AppError;
                        setState(prev => ({
                            ...prev,
                            error: appError.payload.message,
                            isLoading: false
                        }));
                    } else {
                        setState(prev => ({
                            ...prev,
                            error: 'Failed to load entry',
                            isLoading: false
                        }));
                    }
                }
            }
        };

        loadEntry();

        return () => {
            isMounted = false;
        };
    }, [entryId]);

    // Handle field updates
    const updateField = useCallback((field: keyof EntryData, value: string | boolean) => {
        setState(prev => {
            if (!prev.entry) return prev;

            return {
                ...prev,
                entry: {
                    ...prev.entry,
                    [field]: value,
                    updatedAt: new Date().toISOString()
                },
                error: null
            };
        });
    }, []);

    // Save entry changes
    const saveEntry = useCallback(async () => {
        if (!state.entry) return;

        setState(prev => ({ ...prev, isSaving: true, error: null }));

        try {
            // Validate required fields
            if (!state.entry.title.trim()) {
                throw new Error('Title is required');
            }

            // Prepare entry data for update
            const entryData: EntryData = {
                title: state.entry.title,
                username: state.entry.username,
                password: state.entry.password,
                url: state.entry.url,
                notes: state.entry.notes,
                categoryName: state.entry.categoryName,
                favorite: state.entry.favorite,
                createdAt: state.entry.createdAt,
                updatedAt: new Date().toISOString()
            };

            // Update entry
            await updateEntry(entryData, entryId);
            
            // Refresh dashboard entries
            await refreshEntries();

            toast({
                title: "Success",
                description: "Entry updated successfully",
                variant: "default",
            });

            // Navigate back to vault
            navigate('/vault');
        } catch (err) {
            console.error('❌ Failed to save entry:', err);
            
            if (err instanceof Error) {
                setState(prev => ({
                    ...prev,
                    error: err.message,
                    isSaving: false
                }));
            } else if (typeof err === 'object' && err !== null) {
                const appError = err as AppError;
                setState(prev => ({
                    ...prev,
                    error: appError.payload.message,
                    isSaving: false
                }));
            } else {
                setState(prev => ({
                    ...prev,
                    error: 'Failed to save entry',
                    isSaving: false
                }));
            }
        }
    }, [state.entry, entryId, navigate, refreshEntries, toast]);

    // Reset entry to original state
    const resetEntry = useCallback(() => {
        setState(prev => ({
            ...prev,
            entry: prev.originalEntry ? { ...prev.originalEntry } : null,
            error: null
        }));
    }, []);

    // Clear error state
    const clearError = useCallback(() => {
        setState(prev => ({ ...prev, error: null }));
    }, []);

    // Calculate if form is dirty (has unsaved changes)
    const isDirty = useCallback(() => {
        if (!state.entry || !state.originalEntry) return false;
        
        // Type assertion after null check
        const entry = state.entry as DecryptedEntry;
        const originalEntry = state.originalEntry as DecryptedEntry;
    
        const relevantFields: (keyof EntryData)[] = [
            'title', 'username', 'password', 'url', 
            'notes', 'categoryName', 'favorite'
        ];
    
        return relevantFields.some(field => 
            entry[field] !== originalEntry[field]
        );
    }, [state.entry, state.originalEntry]);

    return {
        entry: state.entry,
        isLoading: state.isLoading,
        isSaving: state.isSaving,
        error: state.error,
        updateField,
        saveEntry,
        isDirty: isDirty(),
        resetEntry,
        clearError
    };
}