// src/hooks/useEntryData.ts

import { useState, useCallback, useMemo } from 'react';
import { EntryOverview, SearchQuery } from '@/api/apiTypes';
import { searchEntries } from '@/api/entryApi';
import { EntryDataState, SearchParameters, UseEntryDataReturn } from '@/types/dashboard.types';


export function useEntryData(): UseEntryDataReturn {
  const [state, setState] = useState<EntryDataState>({
    entries: [],
    totalCount: 0,
    maxCount: 0,
    categoryDistribution: [],
    availableCategories: [],
    isLoading: true,
    error: null,
  });

  // Optimize sorted entries memoization
  const sortedEntries = useMemo(() => 
    [...state.entries].sort((a, b) => 
        new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
    ), 
    [state.entries]
  );

  
  const fetchEntries = useCallback(async (params: SearchParameters) => {
    // Create a delayed loading state to prevent flashing loading indicators on fast fetches
    // If the fetch takes longer than the delay, the loading state will be shown, if not then the user won't notice anything
    const LOADING_DELAY_MS = 150;
    const loadingStateTimeout = setTimeout(() => {
        setState(prev => ({ ...prev, isLoading: true }));
    }, LOADING_DELAY_MS);

    try {
        const searchQuery: SearchQuery = {
            text: params.text || undefined,
            categoryName: params.categoryName,
            favoritesOnly: params.favoritesOnly
        };
        
        const results = await searchEntries(searchQuery);

        // Cancel the delayed loading state if fetch completes before timeout
        clearTimeout(loadingStateTimeout);
        setState(prev => ({
            entries: results.entries,
            totalCount: results.totalCount,
            maxCount: results.maxCount,
            categoryDistribution: results.categoryDistribution,
            availableCategories: !params.categoryName ? 
                results.categoryDistribution : 
                prev.availableCategories,
            isLoading: false,
            error: null,
        }));
    } catch (error) {
        // Also cancel the delayed loading state on error
        clearTimeout(loadingStateTimeout);
        console.error('❌ Search entries error:', error);
        setState(prev => ({
            ...prev,
            isLoading: false,
            error: error instanceof Error ? error : new Error('Failed to fetch entries'),
        }));
    }
}, []);

  const updateEntryList = useCallback((newEntries: EntryOverview[]) => {
    setState(prev => ({
      ...prev,
      entries: newEntries,
      totalCount: newEntries.length,
    }));
  }, []);

  return {
    ...state,
    entries: sortedEntries,
    fetchEntries,
    updateEntryList,
  };
}