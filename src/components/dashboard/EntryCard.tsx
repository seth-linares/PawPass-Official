import { memo } from 'react';
import { MoreVertical, Trash2, Edit, User, Globe, Star } from 'lucide-react';
import { EntryOverview } from '@/api/apiTypes';
import LoadingSpinner from '@/components/LoadingSpinner';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from '@/components/ui/button';
import { useNavigate } from 'react-router-dom';
import { formatDistanceToNow } from 'date-fns';

interface EntryCardProps {
  entry: EntryOverview;
  isDeleting: boolean;
  onDeleteClick: (id: string) => void;
}

const EntryCard: React.FC<EntryCardProps> = ({
  entry,
  isDeleting,
  onDeleteClick,
}: {
  entry: EntryOverview;
  isDeleting: boolean;
  onDeleteClick: (id: string) => void;
}) => {
  const navigate = useNavigate();

  const handleEditClick = () => {
    navigate(`/entry/${entry.id}`);
  };

  const formattedDate = formatDistanceToNow(new Date(entry.updatedAt), { 
    addSuffix: true 
  });

  return (
    <div 
      className={`card bg-base-100 shadow hover:shadow-lg transition-all duration-300 border border-base-200
        ${isDeleting ? 'opacity-50' : ''}
        ${entry.favorite ? 'border-warning/50' : ''}`}
    >
      <div className="card-body p-4 flex flex-col min-h-[200px] relative">
        {/* Header */}
        <div className="flex justify-between items-start">
          <h3 className="card-title text-lg font-semibold truncate pr-8">
            {entry.title}
            {entry.favorite && (
              <Star className="h-4 w-4 fill-warning stroke-warning ml-2" />
            )}
          </h3>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button 
                variant="ghost" 
                size="sm"
                className="absolute top-3 right-3"
                disabled={isDeleting}
              >
                <MoreVertical className="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-40">
              <DropdownMenuItem onClick={handleEditClick}>
                <Edit className="h-4 w-4 mr-2" />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem 
                onClick={() => onDeleteClick(entry.id)}
                className="text-destructive focus:text-destructive"
              >
                <Trash2 className="h-4 w-4 mr-2" />
                Delete
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>

        {/* Content */}
        <div className="space-y-3 mt-3 flex-1">
          {entry.username && (
            <div className="flex items-center gap-2 text-sm">
              <User className="h-4 w-4 text-muted-foreground flex-shrink-0" />
              <span className="truncate">{entry.username}</span>
            </div>
          )}
          
          {entry.url && (
            <div className="flex items-center gap-2 text-sm">
              <Globe className="h-4 w-4 text-muted-foreground flex-shrink-0" />
              <a 
                href={entry.url}
                target="_blank"
                rel="noopener noreferrer"
                className="link link-info truncate"
              >
                {entry.url}
              </a>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="flex justify-between items-center pt-2 border-t border-base-200">
          {entry.categoryName && (
            <div className="badge badge-outline badge-secondary">{entry.categoryName}</div>
          )}
          <div className="text-xs text-muted-foreground">
            {formattedDate}
          </div>
        </div>

        {/* Loading Overlay */}
        {isDeleting && (
          <div className="absolute inset-0 bg-background/50 flex items-center justify-center rounded-lg">
            <LoadingSpinner size="sm" />
          </div>
        )}
      </div>
    </div>
  );
};

export default memo(EntryCard);