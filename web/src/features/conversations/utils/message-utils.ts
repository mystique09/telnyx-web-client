import type {
  Conversation,
  MediaKind,
  Message,
  MessageStatus,
} from "../types";

export const PAGE_SIZE = 10;

function parseDateValue(dateValue: string): number | null {
  const timestamp = Date.parse(dateValue);
  if (!Number.isNaN(timestamp)) {
    return timestamp;
  }

  const normalized = dateValue.replace(" ", "T").replace(/ ([+-]\d{2}:\d{2}):\d{2}$/, "$1");
  const normalizedTimestamp = Date.parse(normalized);
  if (!Number.isNaN(normalizedTimestamp)) {
    return normalizedTimestamp;
  }

  return null;
}

export function compareMessagesDesc(a: Message, b: Message): number {
  const aTime = parseDateValue(a.createdAt);
  const bTime = parseDateValue(b.createdAt);
  const timeDiff = (bTime ?? 0) - (aTime ?? 0);
  if (timeDiff !== 0) {
    return timeDiff;
  }

  return b.id.localeCompare(a.id);
}

export function compareMessagesAsc(a: Message, b: Message): number {
  return -compareMessagesDesc(a, b);
}

export function paginateMessages(messages: Message[], cursorId?: string | null) {
  const ordered = [...messages].sort(compareMessagesDesc);
  const cursorIndex = cursorId ? ordered.findIndex((message) => message.id === cursorId) : -1;
  const start = cursorIndex >= 0 ? cursorIndex + 1 : 0;
  const page = ordered.slice(start, start + PAGE_SIZE);
  const nextCursor =
    start + page.length < ordered.length && page.length > 0
      ? page[page.length - 1].id
      : null;

  return {
    page: [...page].sort(compareMessagesAsc),
    nextCursor,
  };
}

export function getLatestMessage(conversation: Conversation): Message | undefined {
  return [...conversation.messages].sort(compareMessagesDesc)[0];
}

export function formatMessageTime(dateValue: string): string {
  const timestamp = parseDateValue(dateValue);
  if (timestamp === null) {
    return "Unknown time";
  }

  return new Date(timestamp).toLocaleTimeString([], {
    hour: "numeric",
    minute: "2-digit",
  });
}

export function formatConversationTime(dateValue: string): string {
  const timestamp = parseDateValue(dateValue);
  if (timestamp === null) {
    return "";
  }

  const date = new Date(timestamp);
  const now = new Date();
  const sameDay =
    date.getFullYear() === now.getFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getDate() === now.getDate();

  if (sameDay) {
    return formatMessageTime(dateValue);
  }

  return date.toLocaleDateString([], { month: "short", day: "numeric" });
}

export function formatMessageStatus(status: MessageStatus): string {
  return status.slice(0, 1).toUpperCase() + status.slice(1);
}

export function messageStatusClassName(status: MessageStatus): string {
  switch (status) {
    case "delivered":
      return "text-emerald-500";
    case "failed":
      return "text-red-500";
    case "pending":
    case "queued":
      return "text-amber-500";
    case "sent":
    default:
      return "text-muted-foreground";
  }
}

export function formatMediaKind(kind: MediaKind): string {
  switch (kind) {
    case "image":
      return "Image";
    case "video":
      return "Video";
    case "document":
    default:
      return "Document";
  }
}

export function mediaKindToneClassName(kind: MediaKind): string {
  switch (kind) {
    case "image":
      return "border-sky-300 bg-sky-50 text-sky-700";
    case "video":
      return "border-violet-300 bg-violet-50 text-violet-700";
    case "document":
    default:
      return "border-emerald-300 bg-emerald-50 text-emerald-700";
  }
}
