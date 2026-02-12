export type MessageType = "INBOUND" | "OUTBOUND";
export type MessageStatus = "pending" | "delivered" | "failed" | "sent";
export type MediaKind = "image" | "video" | "document";

export type MediaFile = {
  id: string;
  name: string;
  kind: MediaKind;
  sizeLabel: string;
};

export type PhoneNumber = {
  id: string;
  userId: string;
  name: string;
  phone: string;
};

export type Message = {
  id: string;
  conversationId: string;
  userId: string;
  messageType: MessageType;
  status: MessageStatus;
  mediaFiles?: MediaFile[];
  fromNumber: string;
  content: string;
  createdAt: string;
};

export type Conversation = {
  id: string;
  phoneNumberId: string;
  userId: string;
  title: string;
  counterpartyNumber: string;
  messages: Message[];
};

export type MessageWindow = {
  messages: Message[];
  nextCursor: string | null;
};

export type SentMediaItem = MediaFile & {
  messageId: string;
  sentAt: string;
  status: MessageStatus;
};

export const PAGE_SIZE = 10;
export const USER_ID = "00000000-0000-0000-0000-000000000001";
const SEED_NOW = Date.now();

function minutesAgo(minutes: number): string {
  return new Date(SEED_NOW - minutes * 60_000).toISOString();
}

export function createClientId(prefix: string): string {
  if (typeof crypto !== "undefined" && typeof crypto.randomUUID === "function") {
    return `${prefix}-${crypto.randomUUID()}`;
  }

  return `${prefix}-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

export function compareMessagesDesc(a: Message, b: Message): number {
  const timeDiff = new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
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
  return new Date(dateValue).toLocaleTimeString([], {
    hour: "numeric",
    minute: "2-digit",
  });
}

export function formatConversationTime(dateValue: string): string {
  const date = new Date(dateValue);
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

export const seedPhoneNumbers: PhoneNumber[] = [
  {
    id: "phone-01",
    userId: USER_ID,
    name: "Primary Support",
    phone: "+13125550100",
  },
  {
    id: "phone-02",
    userId: USER_ID,
    name: "Sales Line",
    phone: "+13125550177",
  },
];

export const seedConversations: Conversation[] = [
  {
    id: "conversation-01",
    phoneNumberId: "phone-01",
    userId: USER_ID,
    title: "Acme Logistics",
    counterpartyNumber: "+14155550189",
    messages: [
      {
        id: "conversation-01-msg-01",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+14155550189",
        content: "Need an ETA update for shipment #4438.",
        createdAt: minutesAgo(95),
      },
      {
        id: "conversation-01-msg-02",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "sent",
        fromNumber: "+13125550100",
        content: "Checking now. I will send a full update shortly.",
        createdAt: minutesAgo(92),
      },
      {
        id: "conversation-01-msg-03",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+14155550189",
        content: "Thanks. Customer is asking for a delivery window.",
        createdAt: minutesAgo(86),
      },
      {
        id: "conversation-01-msg-04",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "sent",
        mediaFiles: [
          {
            id: "media-01",
            name: "delivery-window.png",
            kind: "image",
            sizeLabel: "328 KB",
          },
        ],
        fromNumber: "+13125550100",
        content: "Current estimate is 3:30-4:15 PM local.",
        createdAt: minutesAgo(80),
      },
      {
        id: "conversation-01-msg-05",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+14155550189",
        content: "Perfect, forwarding that now.",
        createdAt: minutesAgo(75),
      },
      {
        id: "conversation-01-msg-06",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "sent",
        fromNumber: "+13125550100",
        content: "If anything shifts, I will notify you immediately.",
        createdAt: minutesAgo(72),
      },
      {
        id: "conversation-01-msg-07",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+14155550189",
        content: "Can you share POD once delivered?",
        createdAt: minutesAgo(66),
      },
      {
        id: "conversation-01-msg-08",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "sent",
        mediaFiles: [
          {
            id: "media-02",
            name: "pod-template.pdf",
            kind: "document",
            sizeLabel: "1.2 MB",
          },
        ],
        fromNumber: "+13125550100",
        content: "Yes, we will send POD right after drop-off.",
        createdAt: minutesAgo(63),
      },
      {
        id: "conversation-01-msg-09",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+14155550189",
        content: "Great, appreciate the fast turnaround.",
        createdAt: minutesAgo(55),
      },
      {
        id: "conversation-01-msg-10",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "failed",
        fromNumber: "+13125550100",
        content: "No problem, we are tracking it in real time.",
        createdAt: minutesAgo(51),
      },
      {
        id: "conversation-01-msg-11",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+14155550189",
        content: "Driver just arrived. Looks on schedule.",
        createdAt: minutesAgo(28),
      },
      {
        id: "conversation-01-msg-12",
        conversationId: "conversation-01",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "sent",
        mediaFiles: [
          {
            id: "media-03",
            name: "dropoff-proof.jpg",
            kind: "image",
            sizeLabel: "441 KB",
          },
        ],
        fromNumber: "+13125550100",
        content: "Confirmed on our side too.",
        createdAt: minutesAgo(24),
      },
    ],
  },
  {
    id: "conversation-02",
    phoneNumberId: "phone-02",
    userId: USER_ID,
    title: "Northwind Retail",
    counterpartyNumber: "+15035550088",
    messages: [
      {
        id: "conversation-02-msg-01",
        conversationId: "conversation-02",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+15035550088",
        content: "Can we schedule a product demo this week?",
        createdAt: minutesAgo(180),
      },
      {
        id: "conversation-02-msg-02",
        conversationId: "conversation-02",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "sent",
        fromNumber: "+13125550177",
        content: "Yes. We have openings Wednesday or Thursday afternoon.",
        createdAt: minutesAgo(172),
      },
      {
        id: "conversation-02-msg-03",
        conversationId: "conversation-02",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+15035550088",
        content: "Thursday at 2 PM works for us.",
        createdAt: minutesAgo(166),
      },
      {
        id: "conversation-02-msg-04",
        conversationId: "conversation-02",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "sent",
        mediaFiles: [
          {
            id: "media-04",
            name: "demo-walkthrough.mp4",
            kind: "video",
            sizeLabel: "8.4 MB",
          },
        ],
        fromNumber: "+13125550177",
        content: "Booked. Sending calendar invite now.",
        createdAt: minutesAgo(160),
      },
      {
        id: "conversation-02-msg-05",
        conversationId: "conversation-02",
        userId: USER_ID,
        messageType: "INBOUND",
        status: "delivered",
        fromNumber: "+15035550088",
        content: "Invite received, thank you.",
        createdAt: minutesAgo(151),
      },
      {
        id: "conversation-02-msg-06",
        conversationId: "conversation-02",
        userId: USER_ID,
        messageType: "OUTBOUND",
        status: "pending",
        fromNumber: "+13125550177",
        content: "See you then. We will walk through messaging flows.",
        createdAt: minutesAgo(145),
      },
    ],
  },
];
