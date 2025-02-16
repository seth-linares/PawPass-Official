import { invokeCommand } from './commandUtils';
import { 
    DecryptedEntry, 
    EntryData, 
    EnhancedSearchResults,
    SearchQuery, 
    UUID 
} from './apiTypes';

/**
 * Creates a new entry in the vault.
 */
export async function createEntry(entry: EntryData): Promise<void> {
    return invokeCommand<void>('create_entry', { entry });
}

/**
 * Updates an existing entry in the vault.
 */
export async function updateEntry(entry: EntryData, id: UUID): Promise<void> {
    return invokeCommand<void>('update_entry', { entry, id });
}

/**
 * Deletes an entry from the vault.
 */
export async function deleteEntry(id: UUID): Promise<void> {
    return invokeCommand<void>('delete_entry', { id });
}

/**
 * Retrieves a decrypted entry from the vault.
 */
export async function getDecryptedEntry(id: UUID): Promise<DecryptedEntry> {
    return invokeCommand<DecryptedEntry>('get_decrypted_entry', { id });
}

/**
 * Primary function for retrieving entries with optional filtering.
 * When called with an empty SearchQuery, returns all entries.
 * Also returns category distribution matching the current filters.
 * @param query - Search parameters (all optional)
 * @returns Filtered entries, total count, and matching category distribution
 */
export async function searchEntries(query: SearchQuery): Promise<EnhancedSearchResults> {
    console.log('📡 API searchEntries called with:', query);
    try {
        const results = await invokeCommand<EnhancedSearchResults>('search_entries', { query });
        console.log('📡 API searchEntries results:', results);
        return results;
    } catch (error) {
        console.error('📡 API searchEntries error:', error);
        throw error;
    }
}
