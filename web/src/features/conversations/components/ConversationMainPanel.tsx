import type { FormEvent, KeyboardEvent } from "react";
import { MessageSquare } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty";
import type {
  Conversation,
  Message,
  PhoneNumber,
  SentMediaItem,
} from "@/lib/mock-messaging";
import { MessageComposer } from "./MessageComposer";
import { MessageList } from "./MessageList";
import { SentMediaPanel } from "./SentMediaPanel";

type ConversationMainPanelProps = {
  hasConversations: boolean;
  selectedConversation: Conversation | null;
  selectedPhoneNumber: PhoneNumber | null;
  visibleMessages: Message[];
  nextCursor: string | null;
  onLoadOlderMessages: () => void;
  messageDraft: string;
  onMessageDraftChange: (draft: string) => void;
  onSendMessage: (event: FormEvent<HTMLFormElement>) => void;
  onComposerKeyDown: (event: KeyboardEvent<HTMLTextAreaElement>) => void;
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
    <div className="flex flex-1 p-4 md:p-6">
      <Empty className="border border-dashed">
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
  sentMedia,
}: ConversationMainPanelProps) {
  if (!hasConversations) {
    return (
      <EmptyConversationState
        title="No conversations yet"
        description="Start messaging to populate this dashboard."
      />
    );
  }

  if (!selectedConversation) {
    return (
      <EmptyConversationState
        title="Select a conversation"
        description="Choose a thread from the left list to open the chatbox."
      />
    );
  }

  return (
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
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={onLoadOlderMessages}
              >
                Load older messages
              </Button>
            ) : (
              <p className="text-xs text-muted-foreground">Start of conversation</p>
            )}
          </div>

          <MessageList messages={visibleMessages} />

          <MessageComposer
            messageDraft={messageDraft}
            onMessageDraftChange={onMessageDraftChange}
            onSendMessage={onSendMessage}
            onComposerKeyDown={onComposerKeyDown}
          />
        </div>
      </div>

      <SentMediaPanel sentMedia={sentMedia} />
    </div>
  );
}
