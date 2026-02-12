import {
  formatMessageStatus,
  formatMessageTime,
  messageStatusClassName,
  type Message,
} from "@/lib/mock-messaging";
import { cn } from "@/lib/utils";

type MessageListProps = {
  messages: Message[];
};

export function MessageList({ messages }: MessageListProps) {
  return (
    <div className="flex-1 space-y-3 overflow-y-auto px-4 py-4 md:px-6">
      {messages.map((message) => {
        const inbound = message.messageType === "INBOUND";

        return (
          <div key={message.id} className={cn("flex", inbound ? "justify-start" : "justify-end")}>
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
                {inbound
                  ? `From ${message.fromNumber}`
                  : `Sent via ${message.fromNumber}`}{" "}
                -{" "}
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
  );
}
