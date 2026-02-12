import { AnalyticsOverview } from "./components/AnalyticsOverview";
import { DashboardSidebar } from "./components/DashboardSidebar";
import { PhoneNumbersCard } from "./components/PhoneNumbersCard";
import { useDashboardController } from "./hooks/use-dashboard-controller";

function DashboardFeature() {
  const controller = useDashboardController();

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
                Placeholder analytics for now. Backend wiring can be added later.
              </p>
            </div>

            <AnalyticsOverview
              totalConversations={controller.totalConversations}
              totalMessages={controller.totalMessages}
              totalPhoneNumbers={controller.phoneNumbers.length}
            />

            <PhoneNumbersCard
              phoneNumbers={controller.phoneNumbers}
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
