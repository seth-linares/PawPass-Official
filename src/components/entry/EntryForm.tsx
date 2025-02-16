// src/components/entry/EntryForm.tsx

import { useParams, useNavigate } from 'react-router-dom';
import { useEntryForm } from '@/hooks/entry/useEntryForm';
import { useEntryEditor } from '@/hooks/entry/useEntryEditor';
import { EntryFormState } from '@/types/entry.types';  // Update import
import { motion, AnimatePresence } from 'framer-motion';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { AlertCircle, ArrowLeft, Save } from 'lucide-react';
import EntryFormFields from './EntryFormFields';
import LoadingSpinner from '@/components/LoadingSpinner';
import { useToast } from '@/hooks/use-toast';
import { entryFormAnimations } from '@/lib/entry.constants';

export default function EntryForm() {
    // Get entry ID from URL if we're in edit mode
    const { id: entryId } = useParams();
    const navigate = useNavigate();
    const { toast } = useToast();
    
    // Determine if we're in edit mode
    const isEditMode = Boolean(entryId);
    
    const createForm = useEntryForm();
    const editForm = isEditMode && entryId ? useEntryEditor(entryId) : null;

    // Extract the appropriate state and handlers based on mode
    const {
        formData,
        categories,
        isLoading,
        isSubmitting,
        error,
        handleSubmit,
        clearError,
        handleInputChange,
        handleCategoryCreate,
    }: EntryFormState = isEditMode && editForm ? {
        // Edit mode values
        formData: editForm.entry,
        categories: createForm.categories,
        isLoading: editForm.isLoading,
        isSubmitting: editForm.isSaving,
        error: editForm.error,
        handleSubmit: editForm.saveEntry,
        clearError: editForm.clearError,
        handleInputChange: editForm.updateField,
        handleCategoryCreate: createForm.handleCategoryCreate,
    } : {
        // Create mode values
        formData: createForm.formData,
        categories: createForm.categories,
        isLoading: createForm.isLoadingCategories,
        isSubmitting: createForm.isSubmitting,
        error: createForm.error,
        handleSubmit: createForm.handleSubmit,
        clearError: createForm.resetForm,
        handleInputChange: createForm.updateField,
        handleCategoryCreate: createForm.handleCategoryCreate,
    };

    // Handle form submission
    const handleFormSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        clearError();

        try {
            await handleSubmit();
            
            // Success notification
            toast({
                title: isEditMode ? "Entry Updated" : "Entry Created",
                description: isEditMode 
                    ? "Your entry has been successfully updated"
                    : "Your new entry has been created",
                variant: "default",
            });

            // Navigate back to vault
            navigate('/vault');
        } catch (err) {
            console.error('Form submission failed:', err);
            // Error is handled by the form state
        }
    };

    // Show loading state while fetching entry data in edit mode
    if (isEditMode && isLoading) {
        return (
            <div className="flex items-center justify-center min-h-[400px]">
                <LoadingSpinner size="lg" label="Loading entry..." />
            </div>
        );
    }

    return (
        <AnimatePresence mode="wait">
            <motion.div
                initial="initial"
                animate="animate"
                exit="exit"
                variants={entryFormAnimations.pageVariants}
                transition={{ duration: 0.3 }}
                className="container max-w-2xl mx-auto py-11"
            >
                <form onSubmit={handleFormSubmit} className="space-y-6">
                    {/* Header */}
                    <div className="flex items-center justify-between">
                        <Button
                            type="button"
                            variant="ghost"
                            size="icon"
                            onClick={() => navigate('/vault')}
                            className="hover:bg-secondary/80"
                        >
                            <ArrowLeft className="h-6 w-6" />
                        </Button>
                        <h1 className="text-2xl font-bold">
                            {isEditMode ? 'Edit Entry' : 'New Entry'}
                        </h1>
                        <div className="w-10" /> {/* Spacer for alignment */}
                    </div>

                    {/* Description */}
                    <p className="text-muted-foreground text-center">
                        {isEditMode 
                            ? 'Update your existing entry details'
                            : 'Create a new secure entry in your vault'
                        }
                    </p>

                    {/* Error Display */}
                    <AnimatePresence mode="wait">
                        {error && (
                            <motion.div
                                initial={{ opacity: 0, height: 0 }}
                                animate={{ opacity: 1, height: 'auto' }}
                                exit={{ opacity: 0, height: 0 }}
                            >
                                <Alert variant="destructive">
                                    <AlertCircle className="h-4 w-4" />
                                    <AlertDescription>{error}</AlertDescription>
                                </Alert>
                            </motion.div>
                        )}
                    </AnimatePresence>

                    {/* Form Fields */}
                    <EntryFormFields
                        formData={formData}
                        onInputChange={handleInputChange}
                        categories={categories}
                        onAddCategory={handleCategoryCreate}  // Allow category creation in both modes
                        isSubmitting={isSubmitting}
                    />

                    {/* Form Actions */}
                    <div className="flex justify-end gap-4 pt-6">
                        <Button
                            type="button"
                            variant="outline"
                            onClick={() => navigate('/vault')}
                            disabled={isSubmitting}
                        >
                            Cancel
                        </Button>
                        <Button
                            type="submit"
                            disabled={isSubmitting}
                            className="min-w-[120px]"
                        >
                            {isSubmitting ? (
                                <div className="flex items-center gap-2">
                                    <LoadingSpinner size="sm" />
                                    <span>
                                        {isEditMode ? 'Saving...' : 'Creating...'}
                                    </span>
                                </div>
                            ) : (
                                <div className="flex items-center gap-2">
                                    <Save className="h-4 w-4" />
                                    <span>
                                        {isEditMode ? 'Save Changes' : 'Create Entry'}
                                    </span>
                                </div>
                            )}
                        </Button>
                    </div>
                </form>
            </motion.div>
        </AnimatePresence>
    );
}