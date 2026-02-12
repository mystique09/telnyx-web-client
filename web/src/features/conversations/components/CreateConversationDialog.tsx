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
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { PhoneNumber } from "@/lib/mock-messaging";

type CreateConversationDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  phoneNumbers: PhoneNumber[];
  fromPhoneNumberId: string;
  onFromPhoneNumberIdChange: (phoneNumberId: string) => void;
  conversationNameInput: string;
  onConversationNameInputChange: (name: string) => void;
  recipientPhoneInput: string;
  onRecipientPhoneInputChange: (recipient: string) => void;
  onCreateConversation: (event: FormEvent<HTMLFormElement>) => void;
};

export function CreateConversationDialog({
  open,
  onOpenChange,
  phoneNumbers,
  fromPhoneNumberId,
  onFromPhoneNumberIdChange,
  conversationNameInput,
  onConversationNameInputChange,
  recipientPhoneInput,
  onRecipientPhoneInputChange,
  onCreateConversation,
}: CreateConversationDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogTrigger asChild>
        <Button type="button" size="icon" variant="ghost" className="size-7">
          <PenSquare className="size-4" />
          <span className="sr-only">New conversation</span>
        </Button>
      </DialogTrigger>

      <DialogContent>
        <DialogHeader>
          <DialogTitle>New Conversation</DialogTitle>
          <DialogDescription>
            Add a name, pick a sending number, then enter recipient phone number.
          </DialogDescription>
        </DialogHeader>

        <form
          id="new-conversation-form"
          onSubmit={onCreateConversation}
          className="space-y-4"
        >
          <div className="space-y-2">
            <Label htmlFor="conversation-name">Conversation Name</Label>
            <Input
              id="conversation-name"
              placeholder="Acme Logistics"
              value={conversationNameInput}
              onChange={(event) =>
                onConversationNameInputChange(event.target.value)
              }
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="from-number">From Phone Number</Label>
            <Select
              value={fromPhoneNumberId}
              onValueChange={onFromPhoneNumberIdChange}
            >
              <SelectTrigger id="from-number" className="w-full">
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
            <Label htmlFor="recipient-number">Recipient Phone Number</Label>
            <Input
              id="recipient-number"
              placeholder="+14155551234"
              value={recipientPhoneInput}
              onChange={(event) => onRecipientPhoneInputChange(event.target.value)}
            />
          </div>
        </form>

        <DialogFooter>
          <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button type="submit" form="new-conversation-form">
            Open Chat
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
