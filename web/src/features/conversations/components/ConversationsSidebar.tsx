import { Link } from "@inertiajs/react";
import type { FormEvent } from "react";
import { BarChart3, LogOut, MessageSquare } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import {
  formatConversationTime,
  getLatestMessage,
  type Conversation,
  type PhoneNumber,
} from "@/lib/mock-messaging";
import { cn } from "@/lib/utils";
import { CreateConversationDialog } from "./CreateConversationDialog";

type ConversationsSidebarProps = {
  isLoggingOut: boolean;
  onLogout: () => void;
  phoneNumbers: PhoneNumber[];
  conversations: Conversation[];
  selectedConversationId: string | null;
  onSelectConversation: (conversationId: string) => void;
  isCreateConversationDialogOpen: boolean;
  onOpenCreateConversationDialog: (open: boolean) => void;
  fromPhoneNumberId: string;
  onFromPhoneNumberIdChange: (phoneNumberId: string) => void;
  conversationNameInput: string;
  onConversationNameInputChange: (name: string) => void;
  recipientPhoneInput: string;
  onRecipientPhoneInputChange: (recipient: string) => void;
  onCreateConversation: (event: FormEvent<HTMLFormElement>) => void;
};

export function ConversationsSidebar({
  isLoggingOut,
  onLogout,
  phoneNumbers,
  conversations,
  selectedConversationId,
  onSelectConversation,
  isCreateConversationDialogOpen,
  onOpenCreateConversationDialog,
  fromPhoneNumberId,
  onFromPhoneNumberIdChange,
  conversationNameInput,
  onConversationNameInputChange,
  recipientPhoneInput,
  onRecipientPhoneInputChange,
  onCreateConversation,
}: ConversationsSidebarProps) {
  return (
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
            onClick={onLogout}
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
          <CreateConversationDialog
            open={isCreateConversationDialogOpen}
            onOpenChange={onOpenCreateConversationDialog}
            phoneNumbers={phoneNumbers}
            fromPhoneNumberId={fromPhoneNumberId}
            onFromPhoneNumberIdChange={onFromPhoneNumberIdChange}
            conversationNameInput={conversationNameInput}
            onConversationNameInputChange={onConversationNameInputChange}
            recipientPhoneInput={recipientPhoneInput}
            onRecipientPhoneInputChange={onRecipientPhoneInputChange}
            onCreateConversation={onCreateConversation}
          />
        </div>

        {conversations.length === 0 ? (
          <p className="px-2 py-3 text-sm text-muted-foreground">
            No conversations available.
          </p>
        ) : (
          conversations.map((conversation) => {
            const latest = getLatestMessage(conversation);
            const isActive = selectedConversationId === conversation.id;

            return (
              <button
                key={conversation.id}
                type="button"
                onClick={() => onSelectConversation(conversation.id)}
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
  );
}
