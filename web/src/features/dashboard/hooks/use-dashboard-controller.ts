import { router, useForm } from "@inertiajs/react";
import { useMemo, useState, type FormEvent } from "react";
import { toast } from "sonner";

import type { PhoneValidationResult } from "@/components/ui/phone-input";
import type { PhoneNumber } from "@/lib/mock-messaging";
import type { DashboardPageProps } from "../types";

export function useDashboardController(props: DashboardPageProps) {
  const { post: postLogout, processing: isLoggingOut } = useForm({});
  const [isCreatingPhoneNumber, setIsCreatingPhoneNumber] = useState(false);

  const phoneNumbers = useMemo<PhoneNumber[]>(
    () => {
      return props.phoneNumbers.map((item) => ({
        id: item.id,
        userId: item.userId,
        name: item.name,
        phone: item.phone,
      }));
    },
    [props.phoneNumbers],
  );
  const [phoneNameInput, setPhoneNameInput] = useState("");
  const [phoneValueInput, setPhoneValueInput] = useState("");
  const [phoneValidation, setPhoneValidation] =
    useState<PhoneValidationResult | null>(null);
  const [isAddPhoneDialogOpen, setIsAddPhoneDialogOpen] = useState(false);

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

    setIsCreatingPhoneNumber(true);
    router.post(
      "/phone-numbers",
      { name, phone },
      {
        preserveScroll: true,
        onSuccess: () => {
          openAddPhoneDialog(false);
        },
        onError: () => {
          toast.error("Unable to add phone number right now.");
        },
        onFinish: () => {
          setIsCreatingPhoneNumber(false);
        },
      },
    );
  }

  function logout() {
    postLogout("/auth/logout");
  }

  return {
    isLoggingOut,
    logout,
    totalConversations: props.analytics.totalConversations,
    totalMessages: props.analytics.totalMessages,
    totalPhoneNumbers: props.analytics.totalPhoneNumbers,
    phoneNumbers,
    isCreatingPhoneNumber,
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
