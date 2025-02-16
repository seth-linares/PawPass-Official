import { memo } from "react"
import { Check, ChevronsUpDown, Plus } from "lucide-react"
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from "@/components/ui/command"
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form"
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import useComboBox from "@/hooks/useComboBox"
import { ComboBoxProps } from "@/types/types"


// Memoized component since it doesn't need to rerender unless props change
export const ComboBox = memo(<T extends readonly string[]>({ 
    items,
    defaultValue,
    onValueChange,
    label,
    showLabel = false,
    description,
    placeholder = "Select an option",
    searchPlaceholder = "Search...",
    emptyMessage = "No option found.",
    onAddItem,
    capitalizeItems = false,
    className
}: ComboBoxProps<T>) => {
    const { 
        form, 
        onSubmit, 
        inputValue,
        setInputValue,
        open,
        handleOpenChange,
        filteredItems,
        shouldShowAddOption,
        formatDisplayText,
        setValue
    } = useComboBox({ 
        items, 
        defaultValue, 
        onValueChange,
        capitalizeItems 
    });

    // Only show add option if onAddItem is provided and shouldShowAddOption is true
    const showAddOption = onAddItem && shouldShowAddOption;

    return (
        <Form {...form}>
            <FormField
                control={form.control}
                name="value"
                render={({ field }) => (
                    <FormItem className={cn("flex flex-col", className)}>
                        {/* Conditional label rendering */}
                        {(showLabel && label) && (
                            <FormLabel className="text-base-content/80">
                                {label}
                            </FormLabel>
                        )}
                        
                        {/* Popover for dropdown functionality */}
                        <Popover open={open} onOpenChange={handleOpenChange}>
                            {/* Trigger button for the dropdown */}
                            <PopoverTrigger asChild>
                                <FormControl>
                                    <Button
                                        type="button"
                                        variant="outline"
                                        role="combobox"
                                        className={cn(
                                            "w-full justify-between",
                                            "bg-base-100 text-base-content border-base-300",
                                            "hover:bg-base-200 hover:text-base-content",
                                            !field.value && "text-base-content/60"
                                        )}
                                    >
                                        {field.value 
                                            ? formatDisplayText(field.value) 
                                            : placeholder}
                                        <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                                    </Button>
                                </FormControl>
                            </PopoverTrigger>

                            {/* Dropdown content */}
                            <PopoverContent className="w-[var(--radix-popover-trigger-width)] p-0 bg-base-100 border-base-300">
                                <Command className="bg-base-100">
                                    {/* Search input */}
                                    <CommandInput 
                                        placeholder={searchPlaceholder} 
                                        className="bg-base-100 text-base-content"
                                        value={inputValue}
                                        onValueChange={setInputValue}
                                    />
                                    
                                    <CommandList className="bg-base-100">
                                        {/* Empty state message */}
                                        {filteredItems.length === 0 && !showAddOption && (
                                            <CommandEmpty className="text-base-content text-center">
                                                {emptyMessage}
                                            </CommandEmpty>
                                        )}
                                        
                                        {/* List of items */}
                                        <CommandGroup>
                                            {/* Filtered items */}
                                            {filteredItems.map((item) => (
                                                <CommandItem
                                                    value={item}
                                                    key={item}
                                                    onSelect={() => {
                                                        form.setValue("value", item);
                                                        onSubmit(form.getValues());
                                                    }}
                                                    className="text-base-content hover:bg-base-200"
                                                >
                                                    {formatDisplayText(item)}
                                                    <Check
                                                        className={cn(
                                                            "ml-auto text-primary",
                                                            item === field.value
                                                                ? "opacity-100"
                                                                : "opacity-0"
                                                        )}
                                                    />
                                                </CommandItem>
                                            ))}
                                            
                                            {showAddOption && inputValue && (
                                                <CommandItem
                                                    value={`add-${inputValue}`}
                                                    className="text-base-content hover:bg-base-200 flex items-center gap-2"
                                                    onSelect={async () => {
                                                        if (onAddItem) {
                                                            try {
                                                                await onAddItem(inputValue);
                                                                setValue(inputValue); // Immediately set the value after successful creation
                                                                handleOpenChange(false);
                                                            } catch (error) {
                                                                console.error('Failed to add item:', error);
                                                            }
                                                        }
                                                    }}
                                                >
                                                    <Plus className="h-4 w-4" />
                                                    Add "{formatDisplayText(inputValue)}"
                                                </CommandItem>
                                            )}
                                        </CommandGroup>
                                    </CommandList>
                                </Command>
                            </PopoverContent>
                        </Popover>
                        
                        {/* Optional description */}
                        {description && (
                            <FormDescription className="text-base-content/60">
                                {description}
                            </FormDescription>
                        )}
                        <FormMessage />
                    </FormItem>
                )}
            />
        </Form>
    );
});

ComboBox.displayName = "ComboBox";

export default ComboBox;