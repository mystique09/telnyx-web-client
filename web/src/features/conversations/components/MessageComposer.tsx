import type { FormEvent, KeyboardEvent } from "react";
import { SendHorizontal } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";

type MessageComposerProps = {
  messageDraft: string;
  onMessageDraftChange: (draft: string) => void;
  onSendMessage: (event: FormEvent<HTMLFormElement>) => void;
  onComposerKeyDown: (event: KeyboardEvent<HTMLTextAreaElement>) => void;
};

export function MessageComposer({
  messageDraft,
  onMessageDraftChange,
  onSendMessage,
  onComposerKeyDown,
}: MessageComposerProps) {
  return (
    <form onSubmit={onSendMessage} className="border-t p-4 md:p-6">
      <div className="flex flex-col gap-3">
        <Textarea
          value={messageDraft}
          onChange={(event) => onMessageDraftChange(event.target.value)}
          onKeyDown={onComposerKeyDown}
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
  );
}
