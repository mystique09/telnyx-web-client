import type { FormEvent, KeyboardEvent } from "react";
import { SendHorizontal } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";

type MessageComposerProps = {
  messageDraft: string;
  onMessageDraftChange: (draft: string) => void;
  onSendMessage: (event: FormEvent<HTMLFormElement>) => void;
  onComposerKeyDown: (event: KeyboardEvent<HTMLTextAreaElement>) => void;
  isSendingMessage: boolean;
};

export function MessageComposer({
  messageDraft,
  onMessageDraftChange,
  onSendMessage,
  onComposerKeyDown,
  isSendingMessage,
}: MessageComposerProps) {
  return (
    <form
      onSubmit={onSendMessage}
      className="shrink-0 border-t border-border/80 bg-background/85 p-4 backdrop-blur sm:px-6 sm:py-5"
    >
      <div className="rounded-[1.75rem] border border-border/80 bg-card/92 p-4 shadow-[0_24px_70px_-52px_rgba(15,23,42,0.72)]">
        <Textarea
          value={messageDraft}
          onChange={(event) => onMessageDraftChange(event.target.value)}
          onKeyDown={onComposerKeyDown}
          placeholder="Type a message..."
          className="min-h-28 rounded-none border-0 bg-transparent p-0 text-base shadow-none focus-visible:ring-0"
          disabled={isSendingMessage}
        />

        <div className="mt-4 flex flex-col gap-3 border-t border-border/70 pt-4 sm:flex-row sm:items-center sm:justify-between">
          <div className="space-y-1">
            <p className="text-xs font-medium text-foreground">
              {messageDraft.length} characters
            </p>
            <p className="text-xs text-muted-foreground">
              Tip: Ctrl+Enter sends the current message.
            </p>
          </div>

          <Button
            type="submit"
            className="rounded-full px-5"
            disabled={isSendingMessage}
          >
            <SendHorizontal className="size-4" />
            {isSendingMessage ? "Sending..." : "Send message"}
          </Button>
        </div>
      </div>
    </form>
  );
}
