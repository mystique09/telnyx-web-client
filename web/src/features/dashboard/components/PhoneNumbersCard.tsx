import type { FormEvent } from "react";
import { Phone } from "lucide-react";

import type { PhoneValidationResult } from "@/components/ui/phone-input";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { PhoneNumber } from "@/lib/mock-messaging";
import { AddPhoneNumberDialog } from "./AddPhoneNumberDialog";

type PhoneNumbersCardProps = {
  phoneNumbers: PhoneNumber[];
  isCreatingPhoneNumber: boolean;
  isAddPhoneDialogOpen: boolean;
  onOpenAddPhoneDialog: (open: boolean) => void;
  phoneNameInput: string;
  onPhoneNameInputChange: (value: string) => void;
  phoneValueInput: string;
  onPhoneValueInputChange: (value: string) => void;
  onPhoneValidationChange: (result: PhoneValidationResult) => void;
  onAddPhoneNumber: (event: FormEvent<HTMLFormElement>) => void;
};

export function PhoneNumbersCard({
  phoneNumbers,
  isCreatingPhoneNumber,
  isAddPhoneDialogOpen,
  onOpenAddPhoneDialog,
  phoneNameInput,
  onPhoneNameInputChange,
  phoneValueInput,
  onPhoneValueInputChange,
  onPhoneValidationChange,
  onAddPhoneNumber,
}: PhoneNumbersCardProps) {
  return (
    <Card>
      <CardHeader className="gap-3">
        <div className="flex flex-wrap items-center justify-between gap-2">
          <CardTitle className="flex items-center gap-2">
            <Phone className="size-4" />
            Manage Phone Numbers
          </CardTitle>

          <AddPhoneNumberDialog
            open={isAddPhoneDialogOpen}
            onOpenChange={onOpenAddPhoneDialog}
            isCreatingPhoneNumber={isCreatingPhoneNumber}
            phoneNameInput={phoneNameInput}
            onPhoneNameInputChange={onPhoneNameInputChange}
            phoneValueInput={phoneValueInput}
            onPhoneValueInputChange={onPhoneValueInputChange}
            onPhoneValidationChange={onPhoneValidationChange}
            onAddPhoneNumber={onAddPhoneNumber}
          />
        </div>
        <CardDescription>
          Add additional phone numbers for this user. Stored with `user_id`.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-2">
        {phoneNumbers.map((phoneNumber) => (
          <div
            key={phoneNumber.id}
            className="flex items-center justify-between rounded-lg border px-3 py-2"
          >
            <div>
              <p className="text-sm font-medium">{phoneNumber.name}</p>
              <p className="text-xs text-muted-foreground">{phoneNumber.phone}</p>
            </div>
            <Badge variant="outline">user_id linked</Badge>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
