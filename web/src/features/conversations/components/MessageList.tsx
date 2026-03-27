import {
  useEffect,
  useEffectEvent,
  useLayoutEffect,
  useRef,
  useState,
} from "react";
import { LoaderCircle } from "lucide-react";

import { cn } from "@/lib/utils";
import type { Message } from "../types";
import {
  formatMessageStatus,
  formatMessageTime,
  messageStatusClassName,
} from "../utils/message-utils";

type MessageListProps = {
  conversationId: string;
  messages: Message[];
  nextCursor: string | null;
  onLoadOlderMessages: () => Promise<boolean>;
};

type PendingPrependState = {
  conversationId: string;
  scrollHeight: number;
  scrollTop: number;
};

export function MessageList({
  conversationId,
  messages,
  nextCursor,
  onLoadOlderMessages,
}: MessageListProps) {
  const scrollContainerRef = useRef<HTMLDivElement | null>(null);
  const topSentinelRef = useRef<HTMLDivElement | null>(null);
  const previousConversationIdRef = useRef<string | null>(null);
  const pendingPrependRef = useRef<PendingPrependState | null>(null);
  const canRequestOlderMessagesRef = useRef(true);
  const [isLoadingOlderMessages, setIsLoadingOlderMessages] = useState(false);

  const requestOlderMessages = useEffectEvent(async () => {
    if (!nextCursor || isLoadingOlderMessages) {
      return;
    }

    const container = scrollContainerRef.current;
    if (!container) {
      return;
    }

    pendingPrependRef.current = {
      conversationId,
      scrollHeight: container.scrollHeight,
      scrollTop: container.scrollTop,
    };
    setIsLoadingOlderMessages(true);
    const didLoad = await onLoadOlderMessages();

    if (!didLoad) {
      pendingPrependRef.current = null;
      setIsLoadingOlderMessages(false);
      canRequestOlderMessagesRef.current = false;
    }
  });

  useLayoutEffect(() => {
    const container = scrollContainerRef.current;
    if (!container) {
      return;
    }

    if (previousConversationIdRef.current !== conversationId) {
      previousConversationIdRef.current = conversationId;
      pendingPrependRef.current = null;
      canRequestOlderMessagesRef.current = true;
      setIsLoadingOlderMessages(false);
      container.scrollTop = container.scrollHeight;
      return;
    }

    const pendingPrepend = pendingPrependRef.current;
    if (!pendingPrepend || pendingPrepend.conversationId !== conversationId) {
      return;
    }

    container.scrollTop =
      container.scrollHeight -
      pendingPrepend.scrollHeight +
      pendingPrepend.scrollTop;
    pendingPrependRef.current = null;
    setIsLoadingOlderMessages(false);
  }, [conversationId, messages.length]);

  useEffect(() => {
    const root = scrollContainerRef.current;
    const target = topSentinelRef.current;
    if (!root || !target || !nextCursor) {
      return;
    }

    const observer = new IntersectionObserver(
      (entries) => {
        const firstEntry = entries[0];
        if (!firstEntry) {
          return;
        }

        if (!firstEntry.isIntersecting) {
          canRequestOlderMessagesRef.current = true;
          return;
        }

        if (!canRequestOlderMessagesRef.current) {
          return;
        }

        canRequestOlderMessagesRef.current = false;
        void requestOlderMessages();
      },
      {
        root,
        rootMargin: "120px 0px 0px 0px",
        threshold: 0,
      },
    );

    observer.observe(target);
    return () => observer.disconnect();
  }, [conversationId, nextCursor]);

  if (messages.length === 0) {
    return (
      <div className="flex min-h-0 flex-1 items-center justify-center bg-[linear-gradient(180deg,rgba(255,255,255,0.35),rgba(244,243,238,0.75))] px-6 py-10">
        <div className="rounded-[1.75rem] border border-dashed border-border/80 bg-card/70 px-6 py-8 text-center">
          <p className="text-sm font-medium text-foreground">No messages yet</p>
          <p className="mt-2 max-w-sm text-sm leading-6 text-muted-foreground">
            Send the first outbound message from the composer below to start
            this conversation.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div
      ref={scrollContainerRef}
      className="min-h-0 flex-1 overflow-y-auto bg-[linear-gradient(180deg,rgba(255,255,255,0.35),rgba(244,243,238,0.75))] px-4 py-5 sm:px-6"
      tabIndex={0}
      aria-label="Conversation messages"
    >
      <div ref={topSentinelRef} className="h-px w-full" />

      <div className="mx-auto mb-5 flex w-fit items-center gap-2 rounded-full border border-border/80 bg-background/80 px-3 py-1 font-mono text-[11px] uppercase tracking-[0.24em] text-muted-foreground">
        {isLoadingOlderMessages ? (
          <>
            <LoaderCircle className="size-3 animate-spin" />
            Loading earlier messages
          </>
        ) : nextCursor ? (
          "Earlier messages available"
        ) : (
          "Beginning of conversation"
        )}
      </div>

      <div className="space-y-3">
        {messages.map((message) => {
          const inbound = message.messageType === "INBOUND";

          return (
            <div
              key={message.id}
              className={cn("flex", inbound ? "justify-start" : "justify-end")}
            >
              <div
                className={cn(
                  "max-w-[85%] rounded-[1.5rem] px-4 py-3 text-sm shadow-[0_20px_60px_-45px_rgba(15,23,42,0.75)] md:max-w-[72%]",
                  inbound
                    ? "rounded-bl-md border border-border/80 bg-background/92 text-foreground"
                    : "rounded-br-md bg-primary text-primary-foreground",
                )}
              >
                <p className="leading-6">{message.content}</p>
                <p
                  className={cn(
                    "mt-3 text-[11px]",
                    inbound
                      ? "text-muted-foreground"
                      : "text-primary-foreground/78",
                  )}
                >
                  {inbound
                    ? `From ${message.fromNumber}`
                    : `Sent via ${message.fromNumber}`}{" "}
                  -{" "}
                  <span
                    className={cn(
                      "font-medium",
                      messageStatusClassName(message.status),
                    )}
                  >
                    {formatMessageStatus(message.status)}
                  </span>{" "}
                  - {formatMessageTime(message.createdAt)}
                </p>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
