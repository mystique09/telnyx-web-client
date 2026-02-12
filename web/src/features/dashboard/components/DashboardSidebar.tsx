import { Link } from "@inertiajs/react";
import { BarChart3, LogOut, MessageSquare } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";

type DashboardSidebarProps = {
  isLoggingOut: boolean;
  onLogout: () => void;
};

export function DashboardSidebar({
  isLoggingOut,
  onLogout,
}: DashboardSidebarProps) {
  return (
    <aside className="w-full border-b bg-card md:w-80 md:border-r md:border-b-0">
      <div className="space-y-4 p-4">
        <div className="space-y-1">
          <p className="text-sm font-medium">Telnyx Web Client</p>
          <p className="text-xs text-muted-foreground">Messaging workspace</p>
        </div>

        <div className="space-y-2">
          <Button asChild className="w-full justify-start gap-2">
            <Link href="/">
              <BarChart3 className="size-4" />
              Dashboard
            </Link>
          </Button>
          <Button asChild variant="outline" className="w-full justify-start gap-2">
            <Link href="/conversations">
              <MessageSquare className="size-4" />
              Conversations
            </Link>
          </Button>
          <Button
            type="button"
            variant="ghost"
            className="w-full justify-start gap-2"
            onClick={onLogout}
            disabled={isLoggingOut}
          >
            <LogOut className="size-4" />
            {isLoggingOut ? "Logging out..." : "Logout"}
          </Button>
        </div>
      </div>

      <Separator />
    </aside>
  );
}
