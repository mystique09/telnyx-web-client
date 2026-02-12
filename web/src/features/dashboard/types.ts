import type { PropsWithFlash } from "@/lib/types";

export interface DashboardPhoneNumberRecord {
  id: string;
  userId: string;
  name: string;
  phone: string;
  createdAt: string;
  updatedAt: string;
}

export interface DashboardPageProps extends PropsWithFlash {
  [key: string]: unknown;
  phoneNumbers?: DashboardPhoneNumberRecord[];
}
