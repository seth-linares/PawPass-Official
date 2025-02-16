import useChangeKeyDerivation from "@/hooks/settings/useChangeKeyDerivation";
import { Button } from "@/components/ui/button";
import { useToast } from "@/hooks/use-toast";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { KEY_DERIVATION_CONSTANTS } from "@/utils/keyDerivationValidation";

export function ChangeKeyDerivationDialog() {
  const { toast } = useToast();
  
  const {
    formData,
    password,
    open,
    isUpdating,
    isLoading,
    error,
    validationErrors,
    updateParams,
    updateFormField,
    resetForm,
    setPassword,
    setOpen,
    initialized,
  } = useChangeKeyDerivation();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const success = await updateParams(password);
    
    if (success) {
      toast({
        title: "Success!",
        description: "Key derivation parameters have been successfully updated.",
        variant: "success",
        duration: 1500,
      });
      setOpen(false);
      resetForm();
    } else if (validationErrors.length > 0) {
      toast({
        title: "Validation Error",
        description: validationErrors.map(e => `${e.field}: ${e.message}`).join('\n'),
        variant: "destructive",
        duration: 1500,
      });
    } else if (error) {
      toast({
        title: "Error",
        description: error.payload.message,
        variant: "destructive",
        duration: 1500,
      });
    }
  };

  const handleDialogClose = (newOpen: boolean) => {
    setOpen(newOpen);
    if (!newOpen) {
      resetForm();
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleDialogClose}>
      <DialogTrigger asChild>
        <Button className="btn-primary">
          Change Key Derivation Parameters
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Update Key Derivation</DialogTitle>
          <DialogDescription>
            {!initialized || isLoading 
              ? "Loading current settings..."
              : "Adjust the key derivation parameters. Higher values increase security but slow down operations."
            }
          </DialogDescription>
        </DialogHeader>

        {(!initialized || isLoading) ? (
          <div className="flex justify-center py-8">
            <span className="loading loading-spinner loading-lg text-primary"></span>
          </div>
        ) : (
          <form onSubmit={handleSubmit}>
            <div className="grid gap-4 py-4">
              <div className="grid gap-2">
                <Label htmlFor="memoryCost">Memory Cost (KiB)</Label>
                <Input
                  id="memoryCost"
                  type="number"
                  value={formData.memoryCost || KEY_DERIVATION_CONSTANTS.RECOMMENDED_MEMORY_COST}
                  onChange={(e) => {
                    const value = parseInt(e.target.value, 10);
                    if (!isNaN(value)) {
                      updateFormField('memoryCost', value);
                    }
                  }}
                  min={8}
                  max={KEY_DERIVATION_CONSTANTS.MAX_MEMORY_COST}
                  disabled={isLoading}
                />
                <p className="text-sm text-base-content/70">
                  Recommended: {KEY_DERIVATION_CONSTANTS.RECOMMENDED_MEMORY_COST} KiB
                </p>
              </div>

              <div className="grid gap-2">
                <Label htmlFor="timeCost">Time Cost (Iterations)</Label>
                <Input
                  id="timeCost"
                  type="number"
                  value={formData.timeCost || KEY_DERIVATION_CONSTANTS.RECOMMENDED_TIME_COST}
                  onChange={(e) => {
                    const value = parseInt(e.target.value, 10);
                    if (!isNaN(value)) {
                      updateFormField('timeCost', value);
                    }
                  }}
                  min={1}
                  max={KEY_DERIVATION_CONSTANTS.MAX_TIME_COST}
                  disabled={isLoading}
                />
                <p className="text-sm text-base-content/70">
                  Recommended: {KEY_DERIVATION_CONSTANTS.RECOMMENDED_TIME_COST} iterations
                </p>
              </div>

              <div className="grid gap-2">
                <Label htmlFor="parallelism">Parallelism</Label>
                <Input
                  id="parallelism"
                  type="number"
                  value={formData.parallelism || 1}
                  onChange={(e) => {
                    const value = parseInt(e.target.value, 10);
                    if (!isNaN(value)) {
                      updateFormField('parallelism', value);
                    }
                  }}
                  min={1}
                  max={KEY_DERIVATION_CONSTANTS.MAX_PARALLELISM}
                  disabled={isLoading}
                />
                <p className="text-sm text-base-content/70">
                  Recommended: 1 thread
                </p>
              </div>

              
              <div className="grid gap-2">
                <Label htmlFor="password">Master Password</Label>
                <Input
                  id="password"
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Enter your master password"
                  disabled={isLoading}
                />
              </div>
              
              {validationErrors.map((error, index) => (
                <p key={index} className="text-sm text-error">
                  {error.field}: {error.message}
                </p>
              ))}
              
              {error && (
                <p className="text-sm text-error">{error.payload.message}</p>
              )}
            </div>
            <DialogFooter>
              <Button type="submit" disabled={isUpdating || isLoading}>
                {(isLoading || isUpdating) && (
                  <span className="loading loading-spinner loading-sm mr-2" />
                )}
                {isLoading ? "Loading..." : isUpdating ? "Updating..." : "Update Parameters"}
              </Button>
            </DialogFooter>
          </form>
        )}
      </DialogContent>
    </Dialog>
  );
}