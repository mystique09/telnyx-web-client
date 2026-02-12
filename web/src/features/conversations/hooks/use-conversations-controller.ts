import { router, useForm, usePage } from "@inertiajs/react";
import {
  useEffect,
  useMemo,
  useState,
  type FormEvent,
  type KeyboardEvent,
} from "react";
import { toast } from "sonner";

import {
  compareMessagesAsc,
  createClientId,
  getLatestMessage,
  paginateMessages,
  type Conversation,
  type Message,
  type MessageWindow,
  type PhoneNumber,
  type SentMediaItem,
  USER_ID,
} from "@/lib/mock-messaging";
import type { ConversationsPageProps } from "../types";
import {
  conversationIdFromPath,
  replaceConversationPath,
} from "../utils/conversation-route";

const E164_PHONE_PATTERN = /^\+?[1-9]\d{6,14}$/;

export function useConversationsController() {
  const { post: postLogout, processing: isLoggingOut } = useForm({});
  const { props, url } = usePage<ConversationsPageProps>();
  const [isCreatingConversation, setIsCreatingConversation] = useState(false);

  const phoneNumbers = useMemo<PhoneNumber[]>(
    () =>
      (props.phoneNumbers ?? []).map((item) => ({
        id: item.id,
        userId: item.userId,
        name: item.name,
        phone: item.phone,
      })),
    [props.phoneNumbers],
  );

  const conversationsFromProps = useMemo<Conversation[]>(() => {
    const phoneById = new Map(phoneNumbers.map((item) => [item.id, item]));

    return (props.conversations ?? []).map((item) => {
      const phoneNumber = phoneById.get(item.phoneNumberId);

      return {
        id: item.id,
        phoneNumberId: item.phoneNumberId,
        userId: item.userId,
        title: phoneNumber?.name ?? `Conversation ${item.id.slice(0, 8)}`,
        counterpartyNumber: phoneNumber?.phone ?? "Unknown",
        messages: [],
      };
    });
  }, [phoneNumbers, props.conversations]);

  const [conversations, setConversations] =
    useState<Conversation[]>(conversationsFromProps);
  const [messageWindows, setMessageWindows] = useState<
    Record<string, MessageWindow>
  >({});
  const [messageDraft, setMessageDraft] = useState("");
  const [isCreateConversationDialogOpen, setIsCreateConversationDialogOpen] =
    useState(false);
  const [fromPhoneNumberId, setFromPhoneNumberId] = useState<string>(
    phoneNumbers[0]?.id ?? "",
  );
  const [conversationNameInput, setConversationNameInput] = useState("");
  const [recipientPhoneInput, setRecipientPhoneInput] = useState("");
  const [selectedConversationId, setSelectedConversationId] = useState<
    string | null
  >(conversationIdFromPath(url));

  useEffect(() => {
    setConversations(conversationsFromProps);
  }, [conversationsFromProps]);

  useEffect(() => {
    if (!phoneNumbers.some((item) => item.id === fromPhoneNumberId)) {
      setFromPhoneNumberId(phoneNumbers[0]?.id ?? "");
    }
  }, [fromPhoneNumberId, phoneNumbers]);

  useEffect(() => {
    setSelectedConversationId(conversationIdFromPath(url));
  }, [url]);

  const sortedConversations = useMemo(() => {
    return [...conversations].sort((a, b) => {
      const latestA = getLatestMessage(a);
      const latestB = getLatestMessage(b);

      if (!latestA && !latestB) {
        return 0;
      }

      if (!latestA) {
        return 1;
      }

      if (!latestB) {
        return -1;
      }

      return (
        new Date(latestB.createdAt).getTime() -
        new Date(latestA.createdAt).getTime()
      );
    });
  }, [conversations]);

  useEffect(() => {
    const handlePopState = () => {
      setSelectedConversationId(conversationIdFromPath(window.location.pathname));
    };

    window.addEventListener("popstate", handlePopState);
    return () => window.removeEventListener("popstate", handlePopState);
  }, []);

  const selectedConversation = useMemo(() => {
    if (!selectedConversationId) {
      return null;
    }

    return (
      sortedConversations.find(
        (conversation) => conversation.id === selectedConversationId,
      ) ?? null
    );
  }, [selectedConversationId, sortedConversations]);

  const selectedPhoneNumber = useMemo(() => {
    if (!selectedConversation) {
      return null;
    }

    return (
      phoneNumbers.find((item) => item.id === selectedConversation.phoneNumberId) ??
      null
    );
  }, [phoneNumbers, selectedConversation]);

  const sentMedia = useMemo<SentMediaItem[]>(() => {
    if (!selectedConversation) {
      return [];
    }

    return selectedConversation.messages
      .filter(
        (message) =>
          message.messageType === "OUTBOUND" &&
          (message.mediaFiles?.length ?? 0) > 0,
      )
      .flatMap((message) =>
        (message.mediaFiles ?? []).map((mediaFile) => ({
          ...mediaFile,
          messageId: message.id,
          sentAt: message.createdAt,
          status: message.status,
        })),
      )
      .sort((a, b) => new Date(b.sentAt).getTime() - new Date(a.sentAt).getTime());
  }, [selectedConversation]);

  const selectedWindow = selectedConversationId
    ? messageWindows[selectedConversationId]
    : undefined;

  const fallbackMessageWindow = useMemo(() => {
    if (!selectedConversation) {
      return null;
    }

    const { page, nextCursor } = paginateMessages(selectedConversation.messages);
    return { messages: page, nextCursor };
  }, [selectedConversation]);

  const visibleMessages = selectedWindow?.messages ?? fallbackMessageWindow?.messages ?? [];
  const nextCursor = selectedWindow?.nextCursor ?? fallbackMessageWindow?.nextCursor ?? null;

  function loadOlderMessages() {
    if (!selectedConversationId || !nextCursor) {
      return;
    }

    const conversation = conversations.find((item) => item.id === selectedConversationId);
    if (!conversation) {
      return;
    }

    setMessageWindows((prev) => {
      const currentWindow = prev[selectedConversationId] ?? fallbackMessageWindow;
      if (!currentWindow || !currentWindow.nextCursor) {
        return prev;
      }

      const { page, nextCursor: cursor } = paginateMessages(
        conversation.messages,
        currentWindow.nextCursor,
      );

      return {
        ...prev,
        [selectedConversationId]: {
          messages: [...page, ...currentWindow.messages],
          nextCursor: cursor,
        },
      };
    });
  }

  function submitMessage() {
    if (!selectedConversationId) {
      return;
    }

    const content = messageDraft.trim();
    if (!content) {
      return;
    }

    const activeConversation = conversations.find(
      (item) => item.id === selectedConversationId,
    );
    if (!activeConversation) {
      return;
    }

    const outboundNumber =
      phoneNumbers.find((item) => item.id === activeConversation.phoneNumberId)?.phone ??
      "unknown";

    const newMessage: Message = {
      id: createClientId("msg"),
      conversationId: selectedConversationId,
      userId: USER_ID,
      messageType: "OUTBOUND",
      status: "pending",
      fromNumber: outboundNumber,
      content,
      createdAt: new Date().toISOString(),
    };

    setConversations((prev) =>
      prev.map((conversation) => {
        if (conversation.id !== selectedConversationId) {
          return conversation;
        }

        return {
          ...conversation,
          messages: [...conversation.messages, newMessage],
        };
      }),
    );

    setMessageWindows((prev) => {
      const currentWindow = prev[selectedConversationId] ?? fallbackMessageWindow;
      if (!currentWindow) {
        return prev;
      }

      return {
        ...prev,
        [selectedConversationId]: {
          ...currentWindow,
          messages: [...currentWindow.messages, newMessage].sort(compareMessagesAsc),
        },
      };
    });

    setMessageDraft("");
  }

  function sendMessage(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    submitMessage();
  }

  function composerKeyDown(event: KeyboardEvent<HTMLTextAreaElement>) {
    if (event.key === "Enter" && event.ctrlKey) {
      event.preventDefault();
      submitMessage();
    }
  }

  function selectConversation(conversationId: string) {
    setSelectedConversationId(conversationId);
    replaceConversationPath(conversationId);
  }

  function openCreateConversationDialog(open: boolean) {
    setIsCreateConversationDialogOpen(open);

    if (!open) {
      setFromPhoneNumberId(phoneNumbers[0]?.id ?? "");
      setConversationNameInput("");
      setRecipientPhoneInput("");
    }
  }

  function createConversation(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const recipient = recipientPhoneInput.trim();
    const selectedPhone = phoneNumbers.find((item) => item.id === fromPhoneNumberId);

    if (!selectedPhone) {
      toast.error("Select a phone number to send from.");
      return;
    }

    if (!E164_PHONE_PATTERN.test(recipient)) {
      toast.error("Use a valid recipient phone format, for example +14155551234.");
      return;
    }

    setIsCreatingConversation(true);
    router.post(
      "/conversations",
      {
        phoneNumberId: selectedPhone.id,
      },
      {
        preserveScroll: true,
        onSuccess: () => {
          openCreateConversationDialog(false);
        },
        onError: () => {
          toast.error("Unable to create conversation right now.");
        },
        onFinish: () => {
          setIsCreatingConversation(false);
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
    phoneNumbers,
    sortedConversations,
    selectedConversationId,
    selectConversation,
    isCreateConversationDialogOpen,
    openCreateConversationDialog,
    fromPhoneNumberId,
    setFromPhoneNumberId,
    conversationNameInput,
    setConversationNameInput,
    recipientPhoneInput,
    setRecipientPhoneInput,
    createConversation,
    isCreatingConversation,
    hasConversations: sortedConversations.length > 0,
    selectedConversation,
    selectedPhoneNumber,
    visibleMessages,
    nextCursor,
    loadOlderMessages,
    messageDraft,
    setMessageDraft,
    sendMessage,
    composerKeyDown,
    sentMedia,
  };
}
