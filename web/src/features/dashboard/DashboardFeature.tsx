import { AnalyticsOverview } from "./components/AnalyticsOverview";
import { AddPhoneNumberDialog } from "./components/AddPhoneNumberDialog";
import { PhoneNumbersCard } from "./components/PhoneNumbersCard";
import { useDashboardController } from "./hooks/use-dashboard-controller";
import type { DashboardPageProps } from "./types";
import { WorkspaceShell } from "@/components/workspace-shell";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Plus, RadioTower } from "lucide-react";

type DashboardFeatureProps = {
  pageProps: DashboardPageProps;
};

function DashboardFeature({ pageProps }: DashboardFeatureProps) {
  const controller = useDashboardController(pageProps);

  return (
    <WorkspaceShell
      activeView="dashboard"
      title="Messaging workspace overview"
      description="Keep account activity, phone number inventory, and operator readiness in one scan before you drop into active threads."
      isLoggingOut={controller.isLoggingOut}
      onLogout={controller.logout}
      actions={
        <AddPhoneNumberDialog
          open={controller.isAddPhoneDialogOpen}
          onOpenChange={controller.openAddPhoneDialog}
          isCreatingPhoneNumber={controller.isCreatingPhoneNumber}
          phoneNameInput={controller.phoneNameInput}
          onPhoneNameInputChange={controller.setPhoneNameInput}
          phoneValueInput={controller.phoneValueInput}
          onPhoneValueInputChange={controller.setPhoneValueInput}
          onPhoneValidationChange={controller.setPhoneValidation}
          onAddPhoneNumber={controller.addPhoneNumber}
          trigger={
            <Button className="h-11 rounded-full px-5 shadow-sm">
              <Plus className="size-4" />
              Add phone number
            </Button>
          }
        />
      }
      sidebarContent={
        <div className="space-y-4">
          <div className="rounded-[1.5rem] border border-white/10 bg-white/[0.04] p-4">
            <p className="font-mono text-[11px] uppercase tracking-[0.28em] text-white/40">
              At a glance
            </p>
            <div className="mt-5 space-y-4">
              <div className="flex items-end justify-between gap-3">
                <div>
                  <p className="text-2xl font-semibold text-white">
                    {controller.totalMessages}
                  </p>
                  <p className="text-xs text-white/60">Total messages</p>
                </div>
                <Badge className="rounded-full bg-white/10 px-2.5 py-1 text-white shadow-none hover:bg-white/10">
                  Live
                </Badge>
              </div>
              <div className="grid gap-3 sm:grid-cols-2">
                <div className="rounded-2xl border border-white/8 bg-black/10 p-3">
                  <p className="text-lg font-semibold text-white">
                    {controller.totalConversations}
                  </p>
                  <p className="text-xs text-white/55">Threads</p>
                </div>
                <div className="rounded-2xl border border-white/8 bg-black/10 p-3">
                  <p className="text-lg font-semibold text-white">
                    {controller.totalPhoneNumbers}
                  </p>
                  <p className="text-xs text-white/55">Phone numbers</p>
                </div>
              </div>
            </div>
          </div>

          <div className="rounded-[1.5rem] border border-white/10 bg-black/12 p-4">
            <div className="flex items-start gap-3">
              <div className="rounded-2xl bg-white/10 p-2 text-white">
                <RadioTower className="size-4" />
              </div>
              <div className="space-y-2">
                <p className="text-sm font-medium text-white">
                  Verified event flow
                </p>
                <p className="text-xs leading-5 text-white/60">
                  Route inventory and delivery activity update from the same
                  authenticated workspace surface.
                </p>
              </div>
            </div>
          </div>
        </div>
      }
    >
      <div className="space-y-6">
        <AnalyticsOverview
          totalConversations={controller.totalConversations}
          totalMessages={controller.totalMessages}
          totalPhoneNumbers={controller.totalPhoneNumbers}
        />

        <PhoneNumbersCard
          phoneNumbers={controller.phoneNumbers}
          deletingPhoneNumberId={controller.deletingPhoneNumberId}
          onDeletePhoneNumber={controller.deletePhoneNumber}
        />
      </div>
    </WorkspaceShell>
  );
}

export default DashboardFeature;
