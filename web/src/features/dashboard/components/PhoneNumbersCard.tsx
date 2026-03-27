import { Phone, RadioTower, Trash2 } from "lucide-react";

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty";
import type { DashboardPhoneNumber } from "../types";

type PhoneNumbersCardProps = {
  phoneNumbers: DashboardPhoneNumber[];
  deletingPhoneNumberId: string | null;
  onDeletePhoneNumber: (phoneNumberId: string) => void;
};

export function PhoneNumbersCard({
  phoneNumbers,
  deletingPhoneNumberId,
  onDeletePhoneNumber,
}: PhoneNumbersCardProps) {
  return (
    <section className="rounded-[2rem] border border-border/80 bg-card/90 p-6 shadow-[0_28px_85px_-60px_rgba(15,23,42,0.45)] sm:p-7">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
        <div className="space-y-2">
          <p className="font-mono text-[11px] uppercase tracking-[0.28em] text-muted-foreground">
            Routing inventory
          </p>
          <div className="space-y-2">
            <h2 className="font-display text-2xl font-semibold tracking-tight text-foreground">
              Phone numbers
            </h2>
            <p className="max-w-2xl text-sm leading-6 text-muted-foreground sm:text-base">
              The outbound identities available to your operators. Each number
              is stored against the authenticated user and ready for new
              conversations.
            </p>
          </div>
        </div>

        <Badge
          variant="outline"
          className="rounded-full px-3 py-1 text-muted-foreground"
        >
          {phoneNumbers.length} active
        </Badge>
      </div>

      <div className="mt-6 overflow-hidden rounded-[1.75rem] border border-border/80 bg-background/82">
        {phoneNumbers.length === 0 ? (
          <div className="p-6">
            <Empty className="border border-dashed bg-card/70">
              <EmptyHeader>
                <EmptyMedia variant="icon">
                  <Phone className="size-5" />
                </EmptyMedia>
                <EmptyTitle>No phone numbers yet</EmptyTitle>
                <EmptyDescription>
                  Add your first sending identity to start routing outbound
                  messages from this workspace.
                </EmptyDescription>
              </EmptyHeader>
            </Empty>
          </div>
        ) : (
          <div className="divide-y divide-border/80">
            {phoneNumbers.map((phoneNumber) => (
              <div
                key={phoneNumber.id}
                className="flex flex-col gap-4 px-5 py-4 transition-colors hover:bg-muted/30 sm:flex-row sm:items-center sm:justify-between"
              >
                <div className="flex items-start gap-4">
                  <div className="rounded-2xl bg-primary/8 p-3 text-primary">
                    <Phone className="size-4" />
                  </div>
                  <div className="space-y-1">
                    <div className="flex items-center gap-2">
                      <p className="text-sm font-medium text-foreground">
                        {phoneNumber.name}
                      </p>
                      <Badge
                        variant="outline"
                        className="rounded-full px-2 py-0.5 text-[11px]"
                      >
                        Ready
                      </Badge>
                    </div>
                    <p className="font-mono text-xs text-muted-foreground">
                      {phoneNumber.phone}
                    </p>
                  </div>
                </div>

                <div className="flex items-center justify-between gap-3 sm:justify-end">
                  <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <RadioTower className="size-4" />
                    Linked to current operator profile
                  </div>

                  <AlertDialog>
                    <AlertDialogTrigger asChild>
                      <Button
                        type="button"
                        variant="ghost"
                        size="icon-sm"
                        className="rounded-full text-muted-foreground hover:text-destructive"
                        disabled={deletingPhoneNumberId !== null}
                        aria-label={`Delete ${phoneNumber.name}`}
                      >
                        <Trash2 className="size-4" />
                      </Button>
                    </AlertDialogTrigger>
                    <AlertDialogContent className="rounded-[1.75rem]">
                      <AlertDialogHeader>
                        <AlertDialogTitle>
                          Delete phone number?
                        </AlertDialogTitle>
                        <AlertDialogDescription className="leading-6">
                          This removes{" "}
                          <span className="font-medium text-foreground">
                            {phoneNumber.name}
                          </span>{" "}
                          from the workspace inventory. Existing records remain
                          in history, but you will no longer use{" "}
                          <span className="font-mono text-foreground">
                            {phoneNumber.phone}
                          </span>{" "}
                          for new outbound conversations.
                        </AlertDialogDescription>
                      </AlertDialogHeader>
                      <AlertDialogFooter>
                        <AlertDialogCancel className="rounded-2xl">
                          Cancel
                        </AlertDialogCancel>
                        <AlertDialogAction
                          className="rounded-2xl bg-destructive text-white hover:bg-destructive/90"
                          onClick={() => onDeletePhoneNumber(phoneNumber.id)}
                        >
                          {deletingPhoneNumberId === phoneNumber.id
                            ? "Deleting..."
                            : "Delete number"}
                        </AlertDialogAction>
                      </AlertDialogFooter>
                    </AlertDialogContent>
                  </AlertDialog>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </section>
  );
}
