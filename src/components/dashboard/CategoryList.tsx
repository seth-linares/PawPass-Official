import { Check, MoreVertical, Pencil, Trash2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from '@/components/ui/input';
import LoadingSpinner from '@/components/LoadingSpinner';
import { useDashboardContext } from '@/contexts/DashboardContext';
import { CategoryCount } from '@/api/apiTypes';

interface CategoryListProps {
  maxCount: number;
}

export default function CategoryList({ maxCount }: CategoryListProps) {
  const {
    searchParams,
    availableCategories,
    setCategoryName,
    handleDeleteCategory,
    isDeletingCategory,
    isRenamingCategory,
    isRenameDialogOpen,
    setIsRenameDialogOpen,
    newName,
    setNewName,
    onRenameSubmit,
    onRenameCancel,
    setSelectedCategoryId,
    setSelectedCategoryName,
  } = useDashboardContext();

  // FORCE REFRESH ON RENAME -- IMPORTANT 
  const handleRename = async () => {
    await onRenameSubmit();
    setCategoryName(undefined);
  };

  // Handler for category selection
  const handleCategoryClick = (categoryName?: string) => {
    if (categoryName === searchParams.categoryName) {
      setCategoryName(undefined);
    } else {
      setCategoryName(categoryName);
    }
  };

  // Too many classNames to fit -- need styles and classes for the category list items
  const categoryItemClasses = (isActive: boolean) => `
    flex items-center w-full px-2.5 py-2 rounded-md text-sm min-h-[2.25rem]
    transition-all duration-200 cursor-pointer
    ${isActive 
      ? 'bg-info/30 text-base-content font-medium' 
      : 'hover:bg-base-200/80 bg-base-300/80 text-base-content/70 hover:text-base-content'
    }
  `;

  const rightContentStyles = "flex items-center gap-1.5 flex-shrink-0 w-[4.5rem] justify-end";
  const countBadgeStyles = (isActive: boolean) => `
    inline-flex items-center justify-center min-w-[1.5rem] h-7 px-1.5
    rounded text-xs font-medium
    ${isActive ? 'bg-base-content/10' : 'bg-base-content/5'}
  `;

  const renderCategoryItem = (category: CategoryCount) => {
    const isActive = searchParams.categoryName === category.name;
    
    return (
      <li key={category.id} className="relative">
        <div className="group">
          <div 
            className={categoryItemClasses(isActive)}
            onClick={() => handleCategoryClick(category.name)}
          >
            {/* Category name and check icon */}
            <div className="flex items-center gap-2 flex-1 min-w-0">
              <Check className={`h-3.5 w-3.5 flex-shrink-0 transition-opacity
                ${isActive ? 'opacity-100' : 'opacity-0'}`} 
              />
              <span className="truncate">{category.name}</span>
            </div>

            {/* Right side with count and actions */}
            <div className={rightContentStyles}>
              <span className={countBadgeStyles(isActive)}>
                {category.entryCount}
              </span>

              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button 
                    variant="ghost" 
                    size="sm"
                    className="h-6 w-6 p-0 opacity-0 group-hover:opacity-100 transition-opacity duration-200
                             hover:bg-base-300"
                    disabled={isDeletingCategory(category.id) || isRenamingCategory(category.id)}
                  >
                    {isDeletingCategory(category.id) || isRenamingCategory(category.id) ? (
                      <LoadingSpinner size="xs" />
                    ) : (
                      <MoreVertical className="h-3.5 w-3.5" />
                    )}
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" className="w-40">
                  <DropdownMenuItem 
                    onClick={() => {
                      setSelectedCategoryId(category.id);    // Set the selected category ID
                      setSelectedCategoryName(category.name); // Set the selected category name
                      setNewName(category.name);
                      setIsRenameDialogOpen(true);
                    }}
                    className="text-base-content"
                  >
                    <Pencil className="h-4 w-4 mr-2" />
                    Rename
                  </DropdownMenuItem>
                  <DropdownMenuItem 
                    onClick={() => handleDeleteCategory(category.id)}
                    className="text-destructive focus:text-destructive"
                  >
                    <Trash2 className="h-4 w-4 mr-2" />
                    Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>
        </div>
      </li>
    );
  };

  return (
    <div className="space-y-2">
      <h3 className="font-medium text-sm text-base-content/50 px-2">Categories</h3>
      
      <ul className="space-y-0.5">
        {/* "All Categories" option */}
        <li className="relative menu-item">
          <div
            className={categoryItemClasses(!searchParams.categoryName)}
            onClick={() => handleCategoryClick(undefined)}
          >
            <div className="flex items-center gap-2 flex-1">
              <Check className={`h-3.5 w-3.5 transition-opacity
                ${!searchParams.categoryName ? 'opacity-100' : 'opacity-0'}`} 
              />
              <span>All Categories</span>
            </div>
            <div className={rightContentStyles}>
              <span className={countBadgeStyles(!searchParams.categoryName)}>
                {maxCount}
              </span>
              <div className="w-6" />
            </div>
          </div>
        </li>

        {/* Category list */}
        {availableCategories.map(renderCategoryItem)}
      </ul>

      {/* Rename Dialog */}
      <Dialog open={isRenameDialogOpen} onOpenChange={setIsRenameDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Rename Category</DialogTitle>
            <DialogDescription>
              Enter a new name for this category.
            </DialogDescription>
          </DialogHeader>
          <Input
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            placeholder="Category name"
            className="mt-4"
          />
          <DialogFooter className="mt-4">
            <Button variant="outline" onClick={onRenameCancel}>
              Cancel
            </Button>
            <Button
              onClick={handleRename}
              disabled={!newName.trim() || newName === searchParams.categoryName}
            >
              Save
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}