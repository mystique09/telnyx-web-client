import {
  formatConversationTime,
  formatMediaKind,
  formatMessageStatus,
  mediaKindToneClassName,
  messageStatusClassName,
  type SentMediaItem,
} from "@/lib/mock-messaging";
import { cn } from "@/lib/utils";

type SentMediaPanelProps = {
  sentMedia: SentMediaItem[];
};

export function SentMediaPanel({ sentMedia }: SentMediaPanelProps) {
  return (
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
  );
}
