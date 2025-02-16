import { invokeCommand } from './commandUtils';
import { Category, UUID } from './apiTypes';

/**
 * Creates a new category.
 */
export async function createCategory(name: string): Promise<void> {
    return invokeCommand<void>('create_category', { name });
}

/**
 * Renames an existing category.
 */
export async function renameCategory(id: UUID, newName: string): Promise<void> {
    console.log('renameCategory', id, newName);
    return invokeCommand<void>('rename_category', { id, newName });
}

/**
 * Deletes a category.
 */
export async function deleteCategory(id: UUID): Promise<void> {
    return invokeCommand<void>('delete_category', { id });
}

/**
 * Searches categories by name for autocomplete.
 * Returns all categories if query is empty.
 */
export async function searchCategories(query: string): Promise<Category[]> {
    return invokeCommand<Category[]>('search_categories', { query });
}
