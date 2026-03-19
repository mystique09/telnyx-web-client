import { router, useForm, usePage } from "@inertiajs/react";
import {
  startTransition,
  useEffect,
  useEffectEvent,
  useMemo,
  useState,
  type FormEvent,
  type KeyboardEvent,
} from "react";
import { toast } from "sonner";

import {
  compareMessagesAsc,
  paginateMessages,
} from "../utils/message-utils";
import type {
  Conversation,
  ConversationsPageProps,
  Message,
  MessageRecord,
  MessageWindow,
  PhoneNumber,
  RealtimeMessageEvent,
  RealtimeMessageEventType,
  SentMediaItem,
} from "../types";
import { conversationIdFromPath } from "../utils/conversation-route";

const E164_PHONE_PATTERN = /^\+?[1-9]\d{6,14}$/;
type ConversationRecordFromProps = NonNullable<
  ConversationsPageProps["conversations"]
>[number];

function mapMessageRecord(record: MessageRecord): Message {
  return {
    ...record,
    mediaFiles: [],
  };
}

function mapConversationRecord(
  record: ConversationRecordFromProps,
  messages: Message[] = [],
): Conversation {
  return {
    id: record.id,
    phoneNumberId: record.phoneNumberId,
    userId: record.userId,
    recipientPhoneNumber: record.recipientPhoneNumber ?? null,
    lastMessageAt: record.lastMessageAt,
    createdAt: record.createdAt,
    updatedAt: record.updatedAt,
    messages,
  };
}

function sameMessage(a: Message, b: Message): boolean {
  return (
    a.id === b.id ||
    (!!a.providerMessageId &&
      !!b.providerMessageId &&
      a.providerMessageId === b.providerMessageId)
  );
}

function upsertMessages(messages: Message[], incomingMessage: Message): Message[] {
  const hasExisting = messages.some((message) => sameMessage(message, incomingMessage));
  const nextMessages = hasExisting
    ? messages.map((message) => (sameMessage(message, incomingMessage) ? incomingMessage : message))
    : [...messages, incomingMessage];

  return [...nextMessages].sort(compareMessagesAsc);
}

export function useConversationsController() {
  const { post: postLogout, processing: isLoggingOut } = useForm({});
  const { props, url } = usePage<ConversationsPageProps>();
  const [isCreatingConversation, setIsCreatingConversation] = useState(false);
  const [isSendingMessage, setIsSendingMessage] = useState(false);

  const phoneNumbers = useMemo<PhoneNumber[]>(
    () => (props.phoneNumbers ?? []).map((item) => ({ ...item })),
    [props.phoneNumbers],
  );

  const selectedConversationMessages = useMemo(
    () => (props.messages ?? []).map(mapMessageRecord),
    [props.messages],
  );

  const conversationsFromProps = useMemo<Conversation[]>(() => {
    return (props.conversations ?? []).map((item) => ({
      ...mapConversationRecord(
        item,
        item.id === props.conversation?.id ? selectedConversationMessages : [],
      ),
    }));
  }, [props.conversation?.id, props.conversations, selectedConversationMessages]);

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
  const [recipientPhoneInput, setRecipientPhoneInput] = useState("");
  const [selectedConversationId, setSelectedConversationId] = useState<
    string | null
  >(conversationIdFromPath(url));

  useEffect(() => {
    setConversations(conversationsFromProps);
    setMessageWindows({});
  }, [conversationsFromProps]);

  useEffect(() => {
    if (!phoneNumbers.some((item) => item.id === fromPhoneNumberId)) {
      setFromPhoneNumberId(phoneNumbers[0]?.id ?? "");
    }
  }, [fromPhoneNumberId, phoneNumbers]);

  useEffect(() => {
    setSelectedConversationId(conversationIdFromPath(url));
  }, [url]);

  const applyRealtimeEvent = useEffectEvent(
    (_eventType: RealtimeMessageEventType, payload: RealtimeMessageEvent) => {
      const incomingMessage = mapMessageRecord(payload.message);

      startTransition(() => {
        setConversations((prev) => {
          let foundConversation = false;
          const nextConversations = prev.map((conversation) => {
            if (conversation.id !== payload.conversation.id) {
              return conversation;
            }

            foundConversation = true;
            return {
              ...conversation,
              ...mapConversationRecord(payload.conversation),
              messages: upsertMessages(conversation.messages, incomingMessage),
            };
          });

          if (foundConversation) {
            return nextConversations;
          }

          return [
            mapConversationRecord(payload.conversation, [incomingMessage]),
            ...prev,
          ];
        });

        setMessageWindows((prev) => {
          const currentWindow = prev[payload.conversation.id];
          if (!currentWindow) {
            return prev;
          }

          return {
            ...prev,
            [payload.conversation.id]: {
              ...currentWindow,
              messages: upsertMessages(currentWindow.messages, incomingMessage),
            },
          };
        });
      });
    },
  );

  useEffect(() => {
    const eventSource = new EventSource("/events/messages");
    const registerHandler = (eventType: RealtimeMessageEventType) => {
      const handler = (event: Event) => {
        if (!(event instanceof MessageEvent)) {
          return;
        }

        try {
          const payload = JSON.parse(event.data) as RealtimeMessageEvent;
          applyRealtimeEvent(eventType, payload);
        } catch {
          // Ignore malformed realtime payloads and keep the stream alive.
        }
      };

      eventSource.addEventListener(eventType, handler);
      return handler;
    };

    const createdHandler = registerHandler("message.created");
    const updatedHandler = registerHandler("message.updated");

    return () => {
      eventSource.removeEventListener("message.created", createdHandler);
      eventSource.removeEventListener("message.updated", updatedHandler);
      eventSource.close();
    };
  }, []);

  const sortedConversations = useMemo(() => {
    return [...conversations].sort(
      (a, b) =>
        new Date(b.lastMessageAt).getTime() - new Date(a.lastMessageAt).getTime(),
    );
  }, [conversations]);

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

  async function submitMessage() {
    if (!selectedConversationId || isSendingMessage) {
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

    setIsSendingMessage(true);

    try {
      const response = await fetch(
        `/conversations/${encodeURIComponent(selectedConversationId)}/messages`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Accept: "application/json",
          },
          credentials: "same-origin",
          body: JSON.stringify({ content }),
        },
      );

      if (!response.ok) {
        let errorMessage = "Unable to send message right now.";
        try {
          const payload = (await response.json()) as { error?: string };
          if (payload.error) {
            errorMessage = payload.error;
          }
        } catch {
          // Keep the fallback error message when the response body is not JSON.
        }

        throw new Error(errorMessage);
      }

      const payload = (await response.json()) as { message: MessageRecord };
      const newMessage = mapMessageRecord(payload.message);

      setConversations((prev) =>
        prev.map((conversation) => {
          if (conversation.id !== selectedConversationId) {
            return conversation;
          }

          return {
            ...conversation,
            lastMessageAt: newMessage.createdAt,
            updatedAt: newMessage.updatedAt,
            messages: upsertMessages(conversation.messages, newMessage),
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
            messages: upsertMessages(currentWindow.messages, newMessage),
          },
        };
      });

      setMessageDraft("");
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Unable to send message right now.",
      );
    } finally {
      setIsSendingMessage(false);
    }
  }

  function sendMessage(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void submitMessage();
  }

  function composerKeyDown(event: KeyboardEvent<HTMLTextAreaElement>) {
    if (event.key === "Enter" && event.ctrlKey) {
      event.preventDefault();
      void submitMessage();
    }
  }

  function selectConversation(conversationId: string) {
    router.get(`/conversations/${encodeURIComponent(conversationId)}`, {}, {
      preserveScroll: true,
    });
  }

  function openCreateConversationDialog(open: boolean) {
    setIsCreateConversationDialogOpen(open);

    if (!open) {
      setFromPhoneNumberId(phoneNumbers[0]?.id ?? "");
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
        recipientPhoneNumber: recipient,
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
    isSendingMessage,
    sendMessage,
    composerKeyDown,
    sentMedia,
  };
}
