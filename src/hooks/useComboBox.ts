// src/hooks/useComboBox.ts

import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { z } from "zod"
import { useState, useMemo, useCallback } from "react"
import { UseComboBoxProps } from "@/types/types";

// Create a Zod schema for form validation
const createFormSchema = () => z.object({
    value: z.string({
        required_error: "Please select a value.",
    }),
});

// Helper function to capitalize strings consistently
export function capitalizeFirstLetter(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
}

const useComboBox = <T extends readonly string[]>({ 
    items, 
    defaultValue, 
    onValueChange,
    capitalizeItems = false 
}: UseComboBoxProps<T>) => {
    // State management with stable initialization
    const [inputValue, setInputValue] = useState("");
    const [open, setOpen] = useState(false);

    // Initialize form with Zod schema
    const FormSchema = useMemo(() => createFormSchema(), []);
    
    const form = useForm<z.infer<typeof FormSchema>>({
        resolver: zodResolver(FormSchema),
        defaultValues: {
            value: defaultValue
        }
    });

    // Memoized dropdown state handler to prevent recreating on each render
    const handleOpenChange = useCallback((isOpen: boolean) => {
        setOpen(isOpen);
        if (!isOpen) {
            setInputValue("");  // Clear input when closing dropdown
        }
    }, []);

    // Memoized filtering logic for dropdown items
    const filteredItems = useMemo(() => {
        const searchInput = inputValue.toLowerCase();
        if (!searchInput) return items;
        return items.filter(item => 
            item.toLowerCase().includes(searchInput)
        );
    }, [inputValue, items]);

    // Memoized to know when to show the add option
    const shouldShowAddOption = useMemo(() => {
        if (!inputValue) return false;
        
        // Important: case-sensitive comparison for flexibility
        return !items.includes(inputValue);
    }, [inputValue, items]);

    const onSubmit = useCallback((data: z.infer<typeof FormSchema>) => {
        onValueChange(data.value as T[number]);
    }, [onValueChange]);

    // Set the new value
    const setValue = useCallback((value: string) => {
        form.setValue("value", value);
        onSubmit(form.getValues());
    }, [form, onSubmit]);

    // Memoized input handler without trimming
    const handleInputChange = useCallback((value: string) => {
        setInputValue(value);
    }, []);

    // Memoized display text formatter
    const formatDisplayText = useCallback((text: string): string => {
        if (!text) return "";
        return capitalizeItems ? capitalizeFirstLetter(text) : text;
    }, [capitalizeItems]);

    // Return stable references to all needed values and functions
    return {
        form,
        onSubmit,
        items,
        inputValue,
        setInputValue: handleInputChange,
        open,
        handleOpenChange,
        filteredItems,
        shouldShowAddOption,
        formatDisplayText,
        setValue
    };
};

export default useComboBox;