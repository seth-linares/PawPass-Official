import useChangeMasterPassword from "@/hooks/settings/useChangeMasterPassword";
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

export function ChangeMasterPasswordDialog() {
  const { toast } = useToast();
  
  const {
    isOpen,
    setOpen,
    oldPassword,
    newPassword,
    confirmPassword,
    setOldPassword,
    setNewPassword,
    setConfirmPassword,
    changePassword,
    isChanging,
    error,
    reset
  } = useChangeMasterPassword();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const success = await changePassword();
    
    if (success) {
      toast({
        title: "Success!",
        description: "Your master password has been successfully changed.",
        variant: "success",
        duration: 1500, // 1.5 seconds
      });
      setOpen(false); // Close the dialog
      reset(); // Reset the form state
    } else if (error) {
      toast({
        title: "Error",
        description: error.payload.message,
        variant: "destructive",
        duration: 1500, // 1.5 seconds
      });
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button className="btn-info">Change Master Password</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Change Master Password</DialogTitle>
            <DialogDescription>
              Enter your current password and choose a new one.
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <Input
              type="password"
              placeholder="Current Password"
              value={oldPassword}
              onChange={(e) => setOldPassword(e.target.value)}
              disabled={isChanging}
            />
            <Input
              type="password"
              placeholder="New Password"
              value={newPassword}
              onChange={(e) => setNewPassword(e.target.value)}
              disabled={isChanging}
            />
            <Input
              type="password"
              placeholder="Confirm New Password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              disabled={isChanging}
            />
            {error && (
              <p className="text-sm text-error">{error.payload.message}</p>
            )}
          </div>
          <DialogFooter>
            <Button type="submit" disabled={isChanging}>
              {isChanging && (
                <span className="loading loading-spinner loading-sm mr-2" />
              )}
              {isChanging ? "Changing..." : "Change Password"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}