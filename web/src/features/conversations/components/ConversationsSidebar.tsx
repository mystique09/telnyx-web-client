import { useState } from "react";

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { Trash2 } from "lucide-react";
import type { Conversation } from "../types";
import { formatConversationTime, getLatestMessage } from "../utils/message-utils";

type ConversationsSidebarProps = {
  conversations: Conversation[];
  selectedConversationId: string | null;
  onSelectConversation: (conversationId: string) => void;
  deletingConversationId: string | null;
  onDeleteConversation: (conversationId: string) => void;
};

export function ConversationsSidebar({
  conversations,
  selectedConversationId,
  onSelectConversation,
  deletingConversationId,
  onDeleteConversation,
}: ConversationsSidebarProps) {
  const [actionConversationId, setActionConversationId] = useState<
    string | null
  >(null);

  return (
    <div className="space-y-4">
      <div className="rounded-[1.5rem] border border-white/10 bg-white/[0.04] p-4">
        <div className="flex items-start justify-between gap-3">
          <div className="space-y-1">
            <p className="text-sm font-medium text-white">Recent threads</p>
            <p className="text-xs leading-5 text-white/60">
              Select a conversation to reply in real time.
            </p>
          </div>
          <Badge className="rounded-full bg-white/10 px-2.5 py-1 text-white shadow-none hover:bg-white/10">
            {conversations.length}
          </Badge>
        </div>
      </div>

      {conversations.length === 0 ? (
        <div className="rounded-[1.5rem] border border-dashed border-white/12 bg-black/10 p-4 text-sm text-white/60">
          No conversations available yet. Open a new thread to populate this
          queue.
        </div>
      ) : (
        <div className="space-y-2">
          {conversations.map((conversation) => {
            const latest = getLatestMessage(conversation);
            const isActive = selectedConversationId === conversation.id;
            const isDeleting = deletingConversationId === conversation.id;
            const isActionVisible =
              actionConversationId === conversation.id || isDeleting;

            return (
              <div
                key={conversation.id}
                className="relative"
                onMouseEnter={() => setActionConversationId(conversation.id)}
                onMouseLeave={() =>
                  setActionConversationId((current) =>
                    current === conversation.id ? null : current,
                  )
                }
                onFocusCapture={() => setActionConversationId(conversation.id)}
                onBlurCapture={(event) => {
                  if (!event.currentTarget.contains(event.relatedTarget)) {
                    setActionConversationId((current) =>
                      current === conversation.id ? null : current,
                    );
                  }
                }}
              >
                <button
                  type="button"
                  onClick={() => onSelectConversation(conversation.id)}
                  className={cn(
                    "block w-full rounded-[1.5rem] border px-4 py-3 pr-14 text-left transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/45 focus-visible:ring-offset-0",
                    isActive
                      ? "border-white/15 bg-white text-slate-950 shadow-[0_24px_48px_-30px_rgba(255,255,255,0.65)]"
                      : "border-white/10 bg-white/[0.04] text-white/80 hover:bg-white/[0.08] hover:text-white",
                  )}
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="min-w-0">
                      <p className="truncate text-sm font-medium">
                        {conversation.recipientPhoneNumber ??
                          `Conversation ${conversation.id.slice(0, 8)}`}
                      </p>
                      <p
                        className={cn(
                          "mt-2 truncate text-xs leading-5",
                          isActive ? "text-slate-600" : "text-white/55",
                        )}
                      >
                        {latest?.content ??
                          conversation.recipientPhoneNumber ??
                          "No messages yet"}
                      </p>
                    </div>
                  </div>
                </button>

                <div className="absolute top-3 right-3 flex min-w-11 justify-end">
                  {conversation.lastMessageAt ? (
                    <span
                      className={cn(
                        "pointer-events-none text-[11px] transition duration-150",
                        isActive ? "text-slate-500" : "text-white/45",
                        isActionVisible ? "opacity-0" : "opacity-100",
                      )}
                    >
                      {formatConversationTime(
                        latest?.createdAt ?? conversation.lastMessageAt,
                      )}
                    </span>
                  ) : null}

                  <AlertDialog>
                    <AlertDialogTrigger asChild>
                      <Button
                        type="button"
                        variant="ghost"
                        size="icon-sm"
                        className={cn(
                          "absolute top-1/2 right-0 h-7 w-7 -translate-y-1/2 rounded-full transition duration-150",
                          isActive
                            ? "text-slate-500 hover:bg-slate-950/8 hover:text-destructive"
                            : "text-white/55 hover:bg-white/10 hover:text-destructive",
                          isActionVisible
                            ? "pointer-events-auto opacity-100"
                            : "pointer-events-none opacity-0",
                        )}
                        disabled={deletingConversationId !== null}
                        aria-label={`Delete ${conversation.recipientPhoneNumber ?? `conversation ${conversation.id.slice(0, 8)}`}`}
                      >
                        <Trash2 className="size-4" />
                      </Button>
                    </AlertDialogTrigger>
                    <AlertDialogContent className="rounded-[1.75rem]">
                      <AlertDialogHeader>
                        <AlertDialogTitle>Delete conversation?</AlertDialogTitle>
                        <AlertDialogDescription className="leading-6">
                          This permanently removes the thread with{" "}
                          <span className="font-medium text-foreground">
                            {conversation.recipientPhoneNumber ??
                              `Conversation ${conversation.id.slice(0, 8)}`}
                          </span>
                          . Message history for this thread will no longer be
                          available in the workspace.
                        </AlertDialogDescription>
                      </AlertDialogHeader>
                      <AlertDialogFooter>
                        <AlertDialogCancel className="rounded-2xl">
                          Cancel
                        </AlertDialogCancel>
                        <AlertDialogAction
                          className="rounded-2xl bg-destructive text-white hover:bg-destructive/90"
                          onClick={() => onDeleteConversation(conversation.id)}
                        >
                          {isDeleting ? "Deleting..." : "Delete thread"}
                        </AlertDialogAction>
                      </AlertDialogFooter>
                    </AlertDialogContent>
                  </AlertDialog>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
