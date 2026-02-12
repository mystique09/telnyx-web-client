import type { PropsWithFlash } from "@/lib/types";

export interface DashboardPhoneNumberRecord {
  id: string;
  userId: string;
  name: string;
  phone: string;
  createdAt: string;
  updatedAt: string;
}

export interface DashboardAnalyticsRecord {
  totalConversations: number;
  totalMessages: number;
  totalPhoneNumbers: number;
}

export interface DashboardPageProps extends PropsWithFlash {
  [key: string]: unknown;
  analytics: DashboardAnalyticsRecord;
  phoneNumbers: DashboardPhoneNumberRecord[];
}
