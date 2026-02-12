import type { PropsWithFlash } from "@/lib/types";

export interface ConversationRecord {
  id: string;
  phoneNumberId: string;
  userId: string;
  lastMessageAt: string;
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

export interface ConversationsPageProps extends PropsWithFlash {
  [key: string]: unknown;
  conversations?: ConversationRecord[];
  conversation?: ConversationRecord | null;
  phoneNumbers?: PhoneNumberRecord[];
}
