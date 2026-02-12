import { Link, useForm, usePage } from "@inertiajs/react";
import {
  useEffect,
  useMemo,
  useState,
  type FormEvent,
  type KeyboardEvent,
} from "react";
import {
  BarChart3,
  LogOut,
  MessageSquare,
  PenSquare,
  SendHorizontal,
} from "lucide-react";

import { Badge } from "@/components/ui/badge";
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
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Textarea } from "@/components/ui/textarea";
import { useFlash } from "@/hooks/use-flash";
import {
  compareMessagesAsc,
  createClientId,
  formatConversationTime,
  formatMediaKind,
  formatMessageStatus,
  formatMessageTime,
  getLatestMessage,
  mediaKindToneClassName,
  messageStatusClassName,
  paginateMessages,
  seedConversations,
  seedPhoneNumbers,
  type Conversation,
  type Message,
  type MessageWindow,
  type PhoneNumber,
  type SentMediaItem,
  USER_ID,
} from "@/lib/mock-messaging";
import type { PropsWithFlash } from "@/lib/types";
import { cn } from "@/lib/utils";
import { toast } from "sonner";

function conversationIdFromPath(url: string): string | null {
  const path = (url.split("?")[0] ?? "").replace(/\/+$/, "");
  if (!path.startsWith("/conversations/")) {
    return null;
  }

  const idSegment = path.slice("/conversations/".length).split("/")[0];
  const id = decodeURIComponent(idSegment ?? "");

  if (!id) {
    return null;
  }

  return id;
}

function replaceConversationPath(conversationId: string | null) {
  if (typeof window === "undefined") {
    return;
  }

  const nextPath = conversationId
    ? `/conversations/${encodeURIComponent(conversationId)}`
    : "/conversations";
  window.history.pushState({}, "", nextPath);
}

function Conversations({ flash }: PropsWithFlash) {
  useFlash(flash);
  const { post: postLogout, processing: isLoggingOut } = useForm({});

  const { url } = usePage();
  const [phoneNumbers] = useState<PhoneNumber[]>(seedPhoneNumbers);
  const [conversations, setConversations] = useState<Conversation[]>(seedConversations);
  const [messageWindows, setMessageWindows] = useState<Record<string, MessageWindow>>({});
  const [messageDraft, setMessageDraft] = useState("");
  const [isCreateConversationDialogOpen, setIsCreateConversationDialogOpen] = useState(false);
  const [fromPhoneNumberId, setFromPhoneNumberId] = useState<string>(seedPhoneNumbers[0]?.id ?? "");
  const [conversationNameInput, setConversationNameInput] = useState("");
  const [recipientPhoneInput, setRecipientPhoneInput] = useState("");
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(
    conversationIdFromPath(url),
  );

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

      return new Date(latestB.createdAt).getTime() - new Date(latestA.createdAt).getTime();
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
      sortedConversations.find((conversation) => conversation.id === selectedConversationId) ??
      null
    );
  }, [selectedConversationId, sortedConversations]);

  const selectedPhoneNumber = useMemo(() => {
    if (!selectedConversation) {
      return null;
    }

    return phoneNumbers.find((item) => item.id === selectedConversation.phoneNumberId) ?? null;
  }, [phoneNumbers, selectedConversation]);

  const sentMedia = useMemo<SentMediaItem[]>(() => {
    if (!selectedConversation) {
      return [];
    }

    return selectedConversation.messages
      .filter((message) => message.messageType === "OUTBOUND" && (message.mediaFiles?.length ?? 0) > 0)
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

  function handleLoadOlderMessages() {
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

  function handleSendMessage(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    submitMessage();
  }

  function handleComposerKeyDown(event: KeyboardEvent<HTMLTextAreaElement>) {
    if (event.key === "Enter" && event.ctrlKey) {
      event.preventDefault();
      submitMessage();
    }
  }

  function handleSelectConversation(conversationId: string) {
    setSelectedConversationId(conversationId);
    replaceConversationPath(conversationId);
  }

  function handleOpenCreateConversationDialog(open: boolean) {
    setIsCreateConversationDialogOpen(open);
    if (!open) {
      setFromPhoneNumberId(phoneNumbers[0]?.id ?? "");
      setConversationNameInput("");
      setRecipientPhoneInput("");
    }
  }

  function handleCreateConversation(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const conversationName = conversationNameInput.trim();
    const recipient = recipientPhoneInput.trim();
    const selectedPhone = phoneNumbers.find((item) => item.id === fromPhoneNumberId);

    if (!selectedPhone) {
      toast.error("Select a phone number to send from.");
      return;
    }

    if (!/^\+?[1-9]\d{6,14}$/.test(recipient)) {
      toast.error("Use a valid recipient phone format, for example +14155551234.");
      return;
    }

    const existing = conversations.find(
      (conversation) =>
        conversation.phoneNumberId === selectedPhone.id &&
        conversation.counterpartyNumber === recipient,
    );

    if (existing) {
      if (conversationName.length > 0 && existing.title !== conversationName) {
        setConversations((prev) =>
          prev.map((conversation) =>
            conversation.id === existing.id
              ? {
                  ...conversation,
                  title: conversationName,
                }
              : conversation,
          ),
        );
      }
      setSelectedConversationId(existing.id);
      replaceConversationPath(existing.id);
      handleOpenCreateConversationDialog(false);
      toast.success("Opened existing conversation.");
      return;
    }

    const conversationId = createClientId("conversation");
    const newConversation: Conversation = {
      id: conversationId,
      phoneNumberId: selectedPhone.id,
      userId: USER_ID,
      title: conversationName.length > 0 ? conversationName : recipient,
      counterpartyNumber: recipient,
      messages: [],
    };

    setConversations((prev) => [newConversation, ...prev]);
    setMessageWindows((prev) => ({
      ...prev,
      [conversationId]: {
        messages: [],
        nextCursor: null,
      },
    }));
    setSelectedConversationId(conversationId);
    replaceConversationPath(conversationId);
    handleOpenCreateConversationDialog(false);
    toast.success("New conversation ready.");
  }

  return (
    <div className="h-screen w-full bg-background">
      <div className="flex h-full w-full flex-col overflow-hidden bg-background md:flex-row">
        <aside className="w-full border-b bg-card md:w-80 md:border-r md:border-b-0">
          <div className="space-y-4 p-4">
            <div className="space-y-1">
              <p className="text-sm font-medium">Telnyx Web Client</p>
              <p className="text-xs text-muted-foreground">Messaging workspace</p>
            </div>

            <div className="space-y-2">
              <Button asChild variant="outline" className="w-full justify-start gap-2">
                <Link href="/">
                  <BarChart3 className="size-4" />
                  Dashboard
                </Link>
              </Button>
              <Button asChild className="w-full justify-start gap-2">
                <Link href="/conversations">
                  <MessageSquare className="size-4" />
                  Conversations
                </Link>
              </Button>
              <Button
                type="button"
                variant="ghost"
                className="w-full justify-start gap-2"
                onClick={() => postLogout("/auth/logout")}
                disabled={isLoggingOut}
              >
                <LogOut className="size-4" />
                {isLoggingOut ? "Logging out..." : "Logout"}
              </Button>
            </div>
          </div>

          <Separator />

          <div className="space-y-2 p-2">
            <div className="flex items-center justify-between px-2">
              <p className="text-xs font-medium text-muted-foreground">Conversations</p>
              <Dialog open={isCreateConversationDialogOpen} onOpenChange={handleOpenCreateConversationDialog}>
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

                  <form id="new-conversation-form" onSubmit={handleCreateConversation} className="space-y-4">
                    <div className="space-y-2">
                      <Label htmlFor="conversation-name">Conversation Name</Label>
                      <Input
                        id="conversation-name"
                        placeholder="Acme Logistics"
                        value={conversationNameInput}
                        onChange={(event) => setConversationNameInput(event.target.value)}
                      />
                    </div>

                    <div className="space-y-2">
                      <Label htmlFor="from-number">From Phone Number</Label>
                      <Select value={fromPhoneNumberId} onValueChange={setFromPhoneNumberId}>
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
                        onChange={(event) => setRecipientPhoneInput(event.target.value)}
                      />
                    </div>
                  </form>

                  <DialogFooter>
                    <Button
                      type="button"
                      variant="outline"
                      onClick={() => handleOpenCreateConversationDialog(false)}
                    >
                      Cancel
                    </Button>
                    <Button type="submit" form="new-conversation-form">
                      Open Chat
                    </Button>
                  </DialogFooter>
                </DialogContent>
              </Dialog>
            </div>
            {sortedConversations.length === 0 ? (
              <p className="px-2 py-3 text-sm text-muted-foreground">
                No conversations available.
              </p>
            ) : (
              sortedConversations.map((conversation) => {
                const latest = getLatestMessage(conversation);
                const isActive = selectedConversationId === conversation.id;
                return (
                  <button
                    key={conversation.id}
                    type="button"
                    onClick={() => handleSelectConversation(conversation.id)}
                    className={cn(
                      "block w-full rounded-lg border px-3 py-2 text-left transition-colors",
                      isActive
                        ? "border-primary bg-primary/5"
                        : "border-transparent hover:border-border hover:bg-muted/50",
                    )}
                  >
                    <div className="flex items-center justify-between gap-2">
                      <p className="truncate text-sm font-medium">{conversation.title}</p>
                      {latest ? (
                        <span className="shrink-0 text-xs text-muted-foreground">
                          {formatConversationTime(latest.createdAt)}
                        </span>
                      ) : null}
                    </div>
                    <p className="mt-1 truncate text-xs text-muted-foreground">
                      {latest?.content ?? "No messages yet"}
                    </p>
                  </button>
                );
              })
            )}
          </div>
        </aside>

        <main className="flex flex-1 flex-col overflow-hidden">
          {sortedConversations.length === 0 ? (
            <div className="flex flex-1 p-4 md:p-6">
              <Empty className="border border-dashed">
                <EmptyHeader>
                  <EmptyMedia variant="icon">
                    <MessageSquare className="size-5" />
                  </EmptyMedia>
                  <EmptyTitle>No conversations yet</EmptyTitle>
                  <EmptyDescription>Start messaging to populate this dashboard.</EmptyDescription>
                </EmptyHeader>
              </Empty>
            </div>
          ) : !selectedConversation ? (
            <div className="flex flex-1 p-4 md:p-6">
              <Empty className="border border-dashed">
                <EmptyHeader>
                  <EmptyMedia variant="icon">
                    <MessageSquare className="size-5" />
                  </EmptyMedia>
                  <EmptyTitle>Select a conversation</EmptyTitle>
                  <EmptyDescription>
                    Choose a thread from the left list to open the chatbox.
                  </EmptyDescription>
                </EmptyHeader>
              </Empty>
            </div>
          ) : (
            <div className="flex h-full flex-1 overflow-hidden">
              <div className="flex min-w-0 flex-1 flex-col overflow-hidden">
                <header className="border-b px-4 py-4 md:px-6">
                  <div className="flex flex-wrap items-center gap-2">
                    <h2 className="text-lg font-semibold">{selectedConversation.title}</h2>
                    <Badge variant="secondary">{selectedConversation.counterpartyNumber}</Badge>
                    {selectedPhoneNumber ? (
                      <Badge variant="outline">via {selectedPhoneNumber.phone}</Badge>
                    ) : null}
                  </div>
                </header>

                <div className="flex flex-1 flex-col overflow-hidden">
                  <div className="border-b px-4 py-2 md:px-6">
                    {nextCursor ? (
                      <Button type="button" variant="outline" size="sm" onClick={handleLoadOlderMessages}>
                        Load older messages
                      </Button>
                    ) : (
                      <p className="text-xs text-muted-foreground">Start of conversation</p>
                    )}
                  </div>

                  <div className="flex-1 space-y-3 overflow-y-auto px-4 py-4 md:px-6">
                    {visibleMessages.map((message) => {
                      const inbound = message.messageType === "INBOUND";
                      return (
                        <div
                          key={message.id}
                          className={cn("flex", inbound ? "justify-start" : "justify-end")}
                        >
                          <div
                            className={cn(
                              "max-w-[85%] rounded-2xl px-4 py-2 text-sm shadow-sm md:max-w-[70%]",
                              inbound
                                ? "rounded-bl-md bg-muted text-foreground"
                                : "rounded-br-md bg-primary text-primary-foreground",
                            )}
                          >
                            <p>{message.content}</p>
                            <p
                              className={cn(
                                "mt-1 text-[11px]",
                                inbound ? "text-muted-foreground" : "text-primary-foreground/80",
                              )}
                            >
                              {inbound ? `From ${message.fromNumber}` : `Sent via ${message.fromNumber}`} -{" "}
                              <span className={cn("font-medium", messageStatusClassName(message.status))}>
                                {formatMessageStatus(message.status)}
                              </span>{" "}
                              - {formatMessageTime(message.createdAt)}
                            </p>
                          </div>
                        </div>
                      );
                    })}
                  </div>

                  <form onSubmit={handleSendMessage} className="border-t p-4 md:p-6">
                    <div className="flex flex-col gap-3">
                      <Textarea
                        value={messageDraft}
                        onChange={(event) => setMessageDraft(event.target.value)}
                        onKeyDown={handleComposerKeyDown}
                        placeholder="Type a message..."
                        className="min-h-20"
                      />
                      <div className="flex items-center justify-between gap-3">
                        <p className="text-xs text-muted-foreground">Tip: Ctrl+Enter sends message</p>
                        <Button type="submit" className="gap-2">
                          <SendHorizontal className="size-4" />
                          Send
                        </Button>
                      </div>
                    </div>
                  </form>
                </div>
              </div>

              <aside className="hidden w-72 shrink-0 border-l bg-muted/20 lg:flex lg:flex-col">
                <div className="border-b px-4 py-4">
                  <h3 className="text-sm font-semibold">Sent Media Files</h3>
                  <p className="text-xs text-muted-foreground">
                    {sentMedia.length} total in this conversation
                  </p>
                </div>

                <div className="flex-1 space-y-3 overflow-y-auto p-4">
                  {sentMedia.length === 0 ? (
                    <p className="rounded-lg border border-dashed p-3 text-xs text-muted-foreground">
                      No sent media files yet.
                    </p>
                  ) : (
                    sentMedia.map((media) => (
                      <div key={`${media.messageId}-${media.id}`} className="rounded-lg border bg-background p-3">
                        <div
                          className={cn(
                            "mb-2 rounded-md border px-2 py-1 text-[10px] font-semibold uppercase tracking-wide",
                            mediaKindToneClassName(media.kind),
                          )}
                        >
                          {formatMediaKind(media.kind)}
                        </div>
                        <p className="truncate text-sm font-medium">{media.name}</p>
                        <p className="mt-1 text-xs text-muted-foreground">
                          {media.sizeLabel} - {formatConversationTime(media.sentAt)}
                        </p>
                        <p className={cn("mt-1 text-xs font-medium", messageStatusClassName(media.status))}>
                          {formatMessageStatus(media.status)}
                        </p>
                      </div>
                    ))
                  )}
                </div>
              </aside>
            </div>
          )}
        </main>
      </div>
    </div>
  );
}

export default Conversations;
