import { AnalyticsOverview } from "./components/AnalyticsOverview";
import { DashboardSidebar } from "./components/DashboardSidebar";
import { PhoneNumbersCard } from "./components/PhoneNumbersCard";
import { useDashboardController } from "./hooks/use-dashboard-controller";
import type { DashboardPageProps } from "./types";

type DashboardFeatureProps = {
  pageProps: DashboardPageProps;
};

function DashboardFeature({ pageProps }: DashboardFeatureProps) {
  const controller = useDashboardController(pageProps);

  return (
    <div className="h-screen w-full bg-background">
      <div className="flex h-full w-full flex-col overflow-hidden bg-background md:flex-row">
        <DashboardSidebar
          isLoggingOut={controller.isLoggingOut}
          onLogout={controller.logout}
        />

        <main className="flex-1 overflow-y-auto p-4 md:p-6">
          <div className="mx-auto w-full max-w-6xl space-y-6">
            <div>
              <h1 className="text-2xl font-semibold">Dashboard Analytics</h1>
              <p className="text-sm text-muted-foreground">
                Live account totals from your current workspace data.
              </p>
            </div>

            <AnalyticsOverview
              totalConversations={controller.totalConversations}
              totalMessages={controller.totalMessages}
              totalPhoneNumbers={controller.totalPhoneNumbers}
            />

            <PhoneNumbersCard
              phoneNumbers={controller.phoneNumbers}
              isCreatingPhoneNumber={controller.isCreatingPhoneNumber}
              isAddPhoneDialogOpen={controller.isAddPhoneDialogOpen}
              onOpenAddPhoneDialog={controller.openAddPhoneDialog}
              phoneNameInput={controller.phoneNameInput}
              onPhoneNameInputChange={controller.setPhoneNameInput}
              phoneValueInput={controller.phoneValueInput}
              onPhoneValueInputChange={controller.setPhoneValueInput}
              onPhoneValidationChange={controller.setPhoneValidation}
              onAddPhoneNumber={controller.addPhoneNumber}
            />
          </div>
        </main>
      </div>
    </div>
  );
}

export default DashboardFeature;
