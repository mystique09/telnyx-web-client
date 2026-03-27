import type { FormEvent } from "react";
import { Plus } from "lucide-react";

import { Button } from "@/components/ui/button";
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
import {
  PhoneInput,
  type PhoneValidationResult,
} from "@/components/ui/phone-input";
import type { ReactNode } from "react";

type AddPhoneNumberDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  isCreatingPhoneNumber: boolean;
  phoneNameInput: string;
  onPhoneNameInputChange: (value: string) => void;
  phoneValueInput: string;
  onPhoneValueInputChange: (value: string) => void;
  onPhoneValidationChange: (result: PhoneValidationResult) => void;
  onAddPhoneNumber: (event: FormEvent<HTMLFormElement>) => void;
  trigger?: ReactNode;
};

export function AddPhoneNumberDialog({
  open,
  onOpenChange,
  isCreatingPhoneNumber,
  phoneNameInput,
  onPhoneNameInputChange,
  phoneValueInput,
  onPhoneValueInputChange,
  onPhoneValidationChange,
  onAddPhoneNumber,
  trigger,
}: AddPhoneNumberDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogTrigger asChild>
        {trigger ?? (
          <Button
            type="button"
            size="sm"
            className="gap-2"
            disabled={isCreatingPhoneNumber}
          >
            <Plus className="size-4" />
            Add number
          </Button>
        )}
      </DialogTrigger>
      <DialogContent className="rounded-[1.75rem] border-border/80 bg-background/98 p-0 shadow-[0_36px_110px_-50px_rgba(15,23,42,0.82)]">
        <div className="space-y-6 p-6 sm:p-7">
        <DialogHeader>
          <DialogTitle className="font-display text-2xl font-semibold tracking-tight">
            Add phone number
          </DialogTitle>
          <DialogDescription>
            Create a new sending identity for this workspace. Use the linked
            number name your operators will recognize immediately.
          </DialogDescription>
        </DialogHeader>

        <form
          id="add-phone-number-form"
          onSubmit={onAddPhoneNumber}
          className="space-y-5"
        >
          <div className="space-y-2">
            <Label htmlFor="phone-name">Number name</Label>
            <Input
              id="phone-name"
              placeholder="Primary Support"
              className="h-12 rounded-2xl border-border/80 bg-background/80"
              value={phoneNameInput}
              onChange={(event) => onPhoneNameInputChange(event.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="phone-value">Phone number</Label>
            <PhoneInput
              id="phone-value"
              placeholder="+13125551234"
              value={phoneValueInput}
              onValueChange={onPhoneValueInputChange}
              onValidationChange={onPhoneValidationChange}
            />
            <p className="text-xs text-muted-foreground">
              Format numbers in E.164 so new conversations can route without
              correction.
            </p>
          </div>
        </form>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            className="rounded-2xl"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            form="add-phone-number-form"
            className="gap-2 rounded-2xl"
            disabled={isCreatingPhoneNumber}
          >
            <Plus className="size-4" />
            {isCreatingPhoneNumber ? "Saving..." : "Save Number"}
          </Button>
        </DialogFooter>
        </div>
      </DialogContent>
    </Dialog>
  );
}
