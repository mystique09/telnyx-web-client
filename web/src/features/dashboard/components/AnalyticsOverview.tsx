import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

type AnalyticsOverviewProps = {
  totalConversations: number;
  totalMessages: number;
  totalPhoneNumbers: number;
};

export function AnalyticsOverview({
  totalConversations,
  totalMessages,
  totalPhoneNumbers,
}: AnalyticsOverviewProps) {
  return (
    <div className="grid gap-4 md:grid-cols-3">
      <Card>
        <CardHeader>
          <CardTitle>Total Conversations</CardTitle>
          <CardDescription>Current active threads</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-3xl font-semibold">{totalConversations}</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Total Messages</CardTitle>
          <CardDescription>Inbound + outbound volume</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-3xl font-semibold">{totalMessages}</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Phone Numbers</CardTitle>
          <CardDescription>Numbers owned by this account</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-3xl font-semibold">{totalPhoneNumbers}</p>
        </CardContent>
      </Card>
    </div>
  );
}
