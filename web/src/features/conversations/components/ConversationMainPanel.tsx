import type { FormEvent, KeyboardEvent } from "react";
import { MessageSquare, Paperclip } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty";
import type { Conversation, Message, PhoneNumber, SentMediaItem } from "../types";
import { MessageComposer } from "./MessageComposer";
import { MessageList } from "./MessageList";
import { SentMediaPanel } from "./SentMediaPanel";

type ConversationMainPanelProps = {
  hasConversations: boolean;
  selectedConversation: Conversation | null;
  selectedPhoneNumber: PhoneNumber | null;
  visibleMessages: Message[];
  nextCursor: string | null;
  onLoadOlderMessages: () => Promise<boolean>;
  messageDraft: string;
  onMessageDraftChange: (draft: string) => void;
  onSendMessage: (event: FormEvent<HTMLFormElement>) => void;
  onComposerKeyDown: (event: KeyboardEvent<HTMLTextAreaElement>) => void;
  isSendingMessage: boolean;
  sentMedia: SentMediaItem[];
};

function EmptyConversationState({
  title,
  description,
}: {
  title: string;
  description: string;
}) {
  return (
    <div className="flex flex-1 py-4">
      <Empty className="w-full rounded-[2rem] border border-dashed bg-card/85 shadow-[0_28px_80px_-60px_rgba(15,23,42,0.45)]">
        <EmptyHeader>
          <EmptyMedia variant="icon">
            <MessageSquare className="size-5" />
          </EmptyMedia>
          <EmptyTitle>{title}</EmptyTitle>
          <EmptyDescription>{description}</EmptyDescription>
        </EmptyHeader>
      </Empty>
    </div>
  );
}

export function ConversationMainPanel({
  hasConversations,
  selectedConversation,
  selectedPhoneNumber,
  visibleMessages,
  nextCursor,
  onLoadOlderMessages,
  messageDraft,
  onMessageDraftChange,
  onSendMessage,
  onComposerKeyDown,
  isSendingMessage,
  sentMedia,
}: ConversationMainPanelProps) {
  if (!hasConversations) {
    return (
      <EmptyConversationState
        title="No conversations yet"
        description="Open a new thread to populate this workspace and start sending messages."
      />
    );
  }

  if (!selectedConversation) {
    return (
      <EmptyConversationState
        title="Select a conversation"
        description="Choose a thread from the sidebar to open the message history and reply surface."
      />
    );
  }

  return (
    <div className="flex min-h-0 flex-1 flex-col gap-4 xl:flex-row">
      <section className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden rounded-[2rem] border border-border/80 bg-card/90 shadow-[0_28px_85px_-60px_rgba(15,23,42,0.45)]">
        <header className="shrink-0 space-y-4 border-b border-border/80 px-5 py-5 sm:px-6">
          <div className="flex flex-col gap-5 lg:flex-row lg:items-end lg:justify-between">
            <div className="space-y-3">
              <p className="font-mono text-[11px] uppercase tracking-[0.28em] text-muted-foreground">
                Active thread
              </p>
              <div className="flex flex-wrap items-center gap-2">
                <h2 className="font-display text-2xl font-semibold tracking-tight text-foreground">
                  {selectedConversation.recipientPhoneNumber ??
                    `Conversation ${selectedConversation.id.slice(0, 8)}`}
                </h2>
                {selectedPhoneNumber ? (
                  <Badge
                    variant="outline"
                    className="rounded-full px-3 py-1 text-muted-foreground"
                  >
                    via {selectedPhoneNumber.phone}
                  </Badge>
                ) : null}
              </div>
              <p className="max-w-2xl text-sm leading-6 text-muted-foreground">
                Reply in real time, load older messages on demand, and keep
                media context visible while the thread stays open.
              </p>
            </div>

            <div className="flex flex-wrap items-center gap-2">
              <Badge
                variant="outline"
                className="rounded-full px-3 py-1 text-muted-foreground"
              >
                {visibleMessages.length} visible messages
              </Badge>
              <Badge
                variant="outline"
                className="rounded-full px-3 py-1 text-muted-foreground"
              >
                {sentMedia.length} loaded media files
              </Badge>
            </div>
          </div>

          <div className="flex flex-col gap-3 rounded-[1.5rem] border border-border/80 bg-background/70 px-4 py-3 sm:flex-row sm:items-center sm:justify-between">
            <p className="text-xs text-muted-foreground">
              {nextCursor
                ? "Older messages load automatically when you reach the top."
                : "Reached the beginning of this conversation."}
            </p>

            <div className="flex items-center gap-2 text-xs text-muted-foreground">
              <Paperclip className="size-4" />
              Live updates continue while this thread stays open.
            </div>
          </div>
        </header>

        <MessageList
          conversationId={selectedConversation.id}
          messages={visibleMessages}
          nextCursor={nextCursor}
          onLoadOlderMessages={onLoadOlderMessages}
        />

        <MessageComposer
          messageDraft={messageDraft}
          onMessageDraftChange={onMessageDraftChange}
          onSendMessage={onSendMessage}
          onComposerKeyDown={onComposerKeyDown}
          isSendingMessage={isSendingMessage}
        />
      </section>

      <SentMediaPanel sentMedia={sentMedia} />
    </div>
  );
}
