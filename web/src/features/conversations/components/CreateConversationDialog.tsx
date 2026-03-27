import type { FormEvent } from "react";
import { PenSquare } from "lucide-react";

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
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { PhoneNumber } from "../types";
import type { ReactNode } from "react";

type CreateConversationDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  isCreatingConversation: boolean;
  phoneNumbers: PhoneNumber[];
  fromPhoneNumberId: string;
  onFromPhoneNumberIdChange: (phoneNumberId: string) => void;
  recipientPhoneInput: string;
  onRecipientPhoneInputChange: (recipient: string) => void;
  onCreateConversation: (event: FormEvent<HTMLFormElement>) => void;
  trigger?: ReactNode;
};

export function CreateConversationDialog({
  open,
  onOpenChange,
  isCreatingConversation,
  phoneNumbers,
  fromPhoneNumberId,
  onFromPhoneNumberIdChange,
  recipientPhoneInput,
  onRecipientPhoneInputChange,
  onCreateConversation,
  trigger,
}: CreateConversationDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogTrigger asChild>
        {trigger ?? (
          <Button
            type="button"
            size="icon"
            variant="ghost"
            className="size-7"
            disabled={isCreatingConversation}
          >
            <PenSquare className="size-4" />
            <span className="sr-only">New conversation</span>
          </Button>
        )}
      </DialogTrigger>

      <DialogContent className="rounded-[1.75rem] border-border/80 bg-background/98 p-0 shadow-[0_36px_110px_-50px_rgba(15,23,42,0.82)]">
        <div className="space-y-6 p-6 sm:p-7">
        <DialogHeader>
          <DialogTitle className="font-display text-2xl font-semibold tracking-tight">
            Open a new conversation
          </DialogTitle>
          <DialogDescription>
            Pick the number you want to send from, then route the first outbound
            thread to the recipient.
          </DialogDescription>
        </DialogHeader>

        <form
          id="new-conversation-form"
          onSubmit={onCreateConversation}
          className="space-y-5"
        >
          <div className="space-y-2">
            <Label htmlFor="from-number">From phone number</Label>
            <Select
              value={fromPhoneNumberId}
              onValueChange={onFromPhoneNumberIdChange}
            >
              <SelectTrigger
                id="from-number"
                className="h-12 w-full rounded-2xl border-border/80 bg-background/80"
              >
                <SelectValue placeholder="Select a phone number" />
              </SelectTrigger>
              <SelectContent>
                {phoneNumbers.map((phoneNumber) => (
                  <SelectItem key={phoneNumber.id} value={phoneNumber.id}>
                    {phoneNumber.name} ({phoneNumber.phone})
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <Label htmlFor="recipient-number">Recipient phone number</Label>
            <Input
              id="recipient-number"
              placeholder="+14155551234"
              className="h-12 rounded-2xl border-border/80 bg-background/80"
              value={recipientPhoneInput}
              onChange={(event) => onRecipientPhoneInputChange(event.target.value)}
            />
            <p className="text-xs text-muted-foreground">
              Use an E.164 number so the new thread is immediately valid.
            </p>
          </div>
        </form>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            className="rounded-2xl"
            onClick={() => onOpenChange(false)}
            disabled={isCreatingConversation}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            form="new-conversation-form"
            className="rounded-2xl"
            disabled={isCreatingConversation}
          >
            {isCreatingConversation ? "Opening..." : "Open Chat"}
          </Button>
        </DialogFooter>
        </div>
      </DialogContent>
    </Dialog>
  );
}
