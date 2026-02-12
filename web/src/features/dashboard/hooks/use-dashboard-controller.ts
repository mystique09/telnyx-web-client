import { useForm } from "@inertiajs/react";
import { useMemo, useState, type FormEvent } from "react";
import { toast } from "sonner";

import type { PhoneValidationResult } from "@/components/ui/phone-input";
import {
  createClientId,
  seedConversations,
  seedPhoneNumbers,
  type PhoneNumber,
  USER_ID,
} from "@/lib/mock-messaging";

export function useDashboardController() {
  const { post: postLogout, processing: isLoggingOut } = useForm({});

  const [phoneNumbers, setPhoneNumbers] =
    useState<PhoneNumber[]>(seedPhoneNumbers);
  const [phoneNameInput, setPhoneNameInput] = useState("");
  const [phoneValueInput, setPhoneValueInput] = useState("");
  const [phoneValidation, setPhoneValidation] =
    useState<PhoneValidationResult | null>(null);
  const [isAddPhoneDialogOpen, setIsAddPhoneDialogOpen] = useState(false);

  const totalMessages = useMemo(() => {
    return seedConversations.reduce(
      (total, conversation) => total + conversation.messages.length,
      0,
    );
  }, []);

  function resetAddPhoneForm() {
    setPhoneNameInput("");
    setPhoneValueInput("");
    setPhoneValidation(null);
  }

  function openAddPhoneDialog(open: boolean) {
    setIsAddPhoneDialogOpen(open);
    if (!open) {
      resetAddPhoneForm();
    }
  }

  function addPhoneNumber(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const name = phoneNameInput.trim();
    const phone = phoneValidation?.e164;

    if (!name || !phoneValueInput.trim()) {
      toast.error("Phone name and number are required.");
      return;
    }

    if (!phoneValidation?.isValid || !phone) {
      toast.error("Use a valid phone format, for example +13125551234.");
      return;
    }

    if (phoneNumbers.some((item) => item.phone === phone)) {
      toast.error("That phone number already exists.");
      return;
    }

    setPhoneNumbers((prev) => [
      ...prev,
      {
        id: createClientId("phone"),
        userId: USER_ID,
        name,
        phone,
      },
    ]);

    openAddPhoneDialog(false);
    toast.success("Phone number added.");
  }

  function logout() {
    postLogout("/auth/logout");
  }

  return {
    isLoggingOut,
    logout,
    totalConversations: seedConversations.length,
    totalMessages,
    phoneNumbers,
    isAddPhoneDialogOpen,
    openAddPhoneDialog,
    phoneNameInput,
    setPhoneNameInput,
    phoneValueInput,
    setPhoneValueInput,
    setPhoneValidation,
    addPhoneNumber,
  };
}
