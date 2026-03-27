import { Badge } from "@/components/ui/badge";
import { MessageSquareText, MessagesSquare, PhoneCall } from "lucide-react";

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
    <section className="relative overflow-hidden rounded-[2rem] border border-border/80 bg-card/90 p-6 shadow-[0_30px_90px_-55px_rgba(15,23,42,0.5)] sm:p-7">
      <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_top_right,rgba(138,164,152,0.18),transparent_30%)]" />
      <div className="relative space-y-8">
        <div className="flex flex-col gap-5 lg:flex-row lg:items-end lg:justify-between">
          <div className="space-y-3">
            <Badge
              variant="outline"
              className="rounded-full border-border/70 bg-background/70 px-3 py-1 font-mono text-[11px] uppercase tracking-[0.28em] text-muted-foreground"
            >
              Live telemetry
            </Badge>
            <div className="space-y-2">
              <h2 className="font-display text-3xl font-semibold tracking-tight text-foreground">
                Activity that matters first
              </h2>
              <p className="max-w-2xl text-sm leading-6 text-muted-foreground sm:text-base">
                Keep volume, active threads, and provisioned numbers visible
                before you pivot into any individual conversation.
              </p>
            </div>
          </div>

          <div className="rounded-full border border-border/70 bg-background/80 px-4 py-2 text-xs text-muted-foreground shadow-sm">
            Server-rendered workspace totals
          </div>
        </div>

        <div className="grid gap-4 lg:grid-cols-[1.25fr_0.875fr_0.875fr]">
          <div className="rounded-[1.75rem] border border-border/80 bg-background/82 p-5">
            <div className="flex items-start justify-between gap-4">
              <div>
                <p className="font-mono text-[11px] uppercase tracking-[0.28em] text-muted-foreground">
                  Message volume
                </p>
                <p className="mt-5 font-display text-5xl font-semibold tracking-tight text-foreground">
                  {totalMessages}
                </p>
              </div>
              <div className="rounded-2xl bg-primary/8 p-3 text-primary">
                <MessagesSquare className="size-5" />
              </div>
            </div>
            <p className="mt-5 max-w-md text-sm leading-6 text-muted-foreground">
              Total inbound and outbound traffic currently represented in this
              workspace snapshot.
            </p>
          </div>

          <div className="rounded-[1.75rem] border border-border/80 bg-background/74 p-5">
            <div className="flex items-center justify-between gap-4">
              <p className="text-sm font-medium text-foreground">
                Active conversations
              </p>
              <MessageSquareText className="size-4 text-muted-foreground" />
            </div>
            <p className="mt-6 font-display text-4xl font-semibold tracking-tight text-foreground">
              {totalConversations}
            </p>
            <p className="mt-3 text-sm leading-6 text-muted-foreground">
              Current threads available for follow-up and reply workflows.
            </p>
          </div>

          <div className="rounded-[1.75rem] border border-border/80 bg-background/74 p-5">
            <div className="flex items-center justify-between gap-4">
              <p className="text-sm font-medium text-foreground">
                Sending numbers
              </p>
              <PhoneCall className="size-4 text-muted-foreground" />
            </div>
            <p className="mt-6 font-display text-4xl font-semibold tracking-tight text-foreground">
              {totalPhoneNumbers}
            </p>
            <p className="mt-3 text-sm leading-6 text-muted-foreground">
              Inventory currently linked to the authenticated account.
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}
