import type { PropsWithFlash } from "@/lib/types";

export interface DashboardPhoneNumberRecord {
  id: string;
  userId: string;
  name: string;
  phone: string;
  createdAt: string;
  updatedAt: string;
}

export type DashboardPhoneNumber = Pick<
  DashboardPhoneNumberRecord,
  "id" | "userId" | "name" | "phone"
>;

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
