import { FileText, Film, Image as ImageIcon } from "lucide-react";

import { cn } from "@/lib/utils";
import type { SentMediaItem } from "../types";
import {
  formatConversationTime,
  formatMediaKind,
  formatMessageStatus,
  mediaKindToneClassName,
  messageStatusClassName,
} from "../utils/message-utils";

type SentMediaPanelProps = {
  sentMedia: SentMediaItem[];
};

function MediaKindIcon({ kind }: { kind: SentMediaItem["kind"] }) {
  switch (kind) {
    case "image":
      return <ImageIcon className="size-4" />;
    case "video":
      return <Film className="size-4" />;
    case "document":
    default:
      return <FileText className="size-4" />;
  }
}

export function SentMediaPanel({ sentMedia }: SentMediaPanelProps) {
  return (
    <aside className="hidden w-80 shrink-0 xl:flex xl:flex-col">
      <div className="flex h-full flex-col overflow-hidden rounded-[2rem] border border-border/80 bg-card/90 shadow-[0_28px_85px_-60px_rgba(15,23,42,0.45)]">
        <div className="border-b border-border/80 px-5 py-5">
          <p className="font-mono text-[11px] uppercase tracking-[0.28em] text-muted-foreground">
            Sent media
          </p>
          <h3 className="mt-3 font-display text-xl font-semibold tracking-tight text-foreground">
            Files shared in this thread
          </h3>
          <p className="mt-2 text-sm leading-6 text-muted-foreground">
            {sentMedia.length} assets currently visible in the loaded message
            history for this conversation.
          </p>
        </div>

        <div className="flex-1 space-y-3 overflow-y-auto p-4">
          {sentMedia.length === 0 ? (
            <div className="rounded-[1.5rem] border border-dashed border-border/80 bg-background/70 p-4 text-sm leading-6 text-muted-foreground">
              No sent media files yet. Shared images, documents, and video clips
              will appear here as the thread grows.
            </div>
          ) : (
            sentMedia.map((media) => (
              <div
                key={`${media.messageId}-${media.id}`}
                className="rounded-[1.5rem] border border-border/80 bg-background/88 p-4"
              >
                <div className="flex items-start justify-between gap-3">
                  <div
                    className={cn(
                      "rounded-2xl border px-2.5 py-2 text-[10px] font-semibold uppercase tracking-[0.24em]",
                      mediaKindToneClassName(media.kind),
                    )}
                  >
                    <div className="flex items-center gap-2">
                      <MediaKindIcon kind={media.kind} />
                      {formatMediaKind(media.kind)}
                    </div>
                  </div>
                  <span className="text-[11px] text-muted-foreground">
                    {formatConversationTime(media.sentAt)}
                  </span>
                </div>

                <p className="mt-4 truncate text-sm font-medium text-foreground">
                  {media.name}
                </p>
                <p className="mt-2 text-xs text-muted-foreground">
                  {media.sizeLabel}
                </p>
                <p
                  className={cn(
                    "mt-3 text-xs font-medium",
                    messageStatusClassName(media.status),
                  )}
                >
                  {formatMessageStatus(media.status)}
                </p>
              </div>
            ))
          )}
        </div>
      </div>
    </aside>
  );
}
