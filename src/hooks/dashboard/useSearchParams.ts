// src/hooks/useSearchParams.ts

import { useState, useCallback, useRef } from 'react';
import { useDebounce } from '../useDebounce';
import { SearchParameters, UseSearchParamsReturn } from '@/types/dashboard.types';


export function useSearchParams(): UseSearchParamsReturn {
  // Maintain search parameter state with default values
  const [searchParams, setSearchParams] = useState<SearchParameters>({
    text: undefined,
    categoryName: undefined,
    favoritesOnly: false,
  });

  // Debounce the search text to prevent too many API calls
  const debouncedSearchText = useDebounce(searchParams.text, 300);

  // Debounce loading state to prevent flickering
  const loadingTimeoutRef = useRef<NodeJS.Timeout>();

  // Handler functions for updating search parameters
  const setSearchText = useCallback((text: string) => {
    // Clear any existing loading timeout
    if (loadingTimeoutRef.current) {
      clearTimeout(loadingTimeoutRef.current);
    }

    setSearchParams(prev => ({
      ...prev,
      text: text || undefined,
    }));
  }, []);

  const setCategoryName = useCallback((category?: string) => {
    setSearchParams(prev => ({
      ...prev,
      categoryName: category,
      text: prev.text,
      favoritesOnly: prev.favoritesOnly,
    }));
  }, []);

  const setFavoritesOnly = useCallback((favorites: boolean) => {
    setSearchParams(prev => ({
      ...prev,
      favoritesOnly: favorites,
    }));
  }, []);

  // Function to reset all search parameters to their default values
  const resetSearch = useCallback(() => {
    setSearchParams({
      text: undefined,
      categoryName: undefined,
      favoritesOnly: false,
    });
  }, []);

  return {
    searchParams,
    debouncedSearchText,
    setSearchText,
    setCategoryName,
    setFavoritesOnly,
    resetSearch,
  };
}