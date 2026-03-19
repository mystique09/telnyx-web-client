import type { PropsWithFlash } from "@/lib/types";

export type MessageType = "INBOUND" | "OUTBOUND";
export type MessageStatus = "pending" | "queued" | "delivered" | "failed" | "sent";
export type MediaKind = "image" | "video" | "document";

export type MediaFile = {
  id: string;
  name: string;
  kind: MediaKind;
  sizeLabel: string;
};

export interface ConversationRecord {
  id: string;
  phoneNumberId: string;
  userId: string;
  recipientPhoneNumber?: string | null;
  lastMessageAt: string;
  createdAt: string;
  updatedAt: string;
}

export interface MessageRecord {
  id: string;
  conversationId: string;
  userId: string;
  messageType: MessageType;
  status: MessageStatus;
  providerMessageId?: string | null;
  providerStatus?: string | null;
  providerStatusUpdatedAt?: string | null;
  providerErrorCode?: string | null;
  providerErrorDetail?: string | null;
  fromNumber: string;
  content: string;
  createdAt: string;
  updatedAt: string;
}

export interface PhoneNumberRecord {
  id: string;
  userId: string;
  name: string;
  phone: string;
  createdAt: string;
  updatedAt: string;
}

export interface Message extends MessageRecord {
  mediaFiles?: MediaFile[];
}

export interface Conversation {
  id: string;
  phoneNumberId: string;
  userId: string;
  recipientPhoneNumber?: string | null;
  lastMessageAt: string;
  createdAt: string;
  updatedAt: string;
  messages: Message[];
}

export interface PhoneNumber {
  id: string;
  userId: string;
  name: string;
  phone: string;
  createdAt: string;
  updatedAt: string;
}

export interface MessageWindow {
  messages: Message[];
  nextCursor: string | null;
}

export type RealtimeMessageEventType = "message.created" | "message.updated";

export interface RealtimeMessageEvent {
  type: RealtimeMessageEventType;
  message: MessageRecord;
  conversation: ConversationRecord;
}

export type SentMediaItem = MediaFile & {
  messageId: string;
  sentAt: string;
  status: MessageStatus;
};

export interface ConversationsPageProps extends PropsWithFlash {
  [key: string]: unknown;
  conversations?: ConversationRecord[];
  conversation?: ConversationRecord | null;
  messages?: MessageRecord[];
  phoneNumbers?: PhoneNumberRecord[];
}
