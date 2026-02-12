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

type AddPhoneNumberDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  phoneNameInput: string;
  onPhoneNameInputChange: (value: string) => void;
  phoneValueInput: string;
  onPhoneValueInputChange: (value: string) => void;
  onPhoneValidationChange: (result: PhoneValidationResult) => void;
  onAddPhoneNumber: (event: FormEvent<HTMLFormElement>) => void;
};

export function AddPhoneNumberDialog({
  open,
  onOpenChange,
  phoneNameInput,
  onPhoneNameInputChange,
  phoneValueInput,
  onPhoneValueInputChange,
  onPhoneValidationChange,
  onAddPhoneNumber,
}: AddPhoneNumberDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogTrigger asChild>
        <Button type="button" size="sm" className="gap-2">
          <Plus className="size-4" />
          Add number
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add Phone Number</DialogTitle>
          <DialogDescription>
            Create a new phone number entry linked to this user.
          </DialogDescription>
        </DialogHeader>

        <form
          id="add-phone-number-form"
          onSubmit={onAddPhoneNumber}
          className="space-y-4"
        >
          <div className="space-y-2">
            <Label htmlFor="phone-name">Number Name</Label>
            <Input
              id="phone-name"
              placeholder="Primary Support"
              value={phoneNameInput}
              onChange={(event) => onPhoneNameInputChange(event.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="phone-value">Phone Number</Label>
            <PhoneInput
              id="phone-value"
              placeholder="+13125551234"
              value={phoneValueInput}
              onValueChange={onPhoneValueInputChange}
              onValidationChange={onPhoneValidationChange}
            />
          </div>
        </form>

        <DialogFooter>
          <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button type="submit" form="add-phone-number-form" className="gap-2">
            <Plus className="size-4" />
            Save Number
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
