import type { ReactNode } from "react";

import { Link } from "@inertiajs/react";
import { BarChart3, LogOut, MessageSquare } from "lucide-react";

import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarSeparator,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { BrandMark } from "./brand-mark";

type WorkspaceShellProps = {
  activeView: "dashboard" | "conversations";
  title: string;
  description: string;
  sidebarContent?: ReactNode;
  actions?: ReactNode;
  children: ReactNode;
  isLoggingOut: boolean;
  onLogout: () => void;
  scrollMode?: "page" | "content";
};

const NAV_ITEMS = [
  {
    href: "/",
    label: "Dashboard",
    key: "dashboard",
    icon: BarChart3,
  },
  {
    href: "/conversations",
    label: "Conversations",
    key: "conversations",
    icon: MessageSquare,
  },
] as const;

export function WorkspaceShell({
  activeView,
  title,
  description,
  sidebarContent,
  actions,
  children,
  isLoggingOut,
  onLogout,
  scrollMode = "page",
}: WorkspaceShellProps) {
  return (
    <SidebarProvider className="min-h-screen bg-[radial-gradient(circle_at_top_left,rgba(26,56,74,0.08),transparent_28%),linear-gradient(180deg,rgba(247,247,244,0.98)_0%,rgba(240,239,234,0.96)_100%)]">
      <Sidebar variant="floating" collapsible="offcanvas" className="border-none">
        <SidebarHeader className="gap-4 p-4">
          <div className="rounded-[1.75rem] border border-white/10 bg-sidebar p-4 shadow-[0_32px_90px_-55px_rgba(2,6,23,0.9)]">
            <BrandMark inverted />
            <div className="mt-8 space-y-3">
              <Badge className="rounded-full bg-white/10 px-3 py-1 font-mono text-[11px] uppercase tracking-[0.28em] text-white shadow-none hover:bg-white/10">
                Live workspace
              </Badge>
              <p className="text-base font-semibold text-white">
                Unified message operations across routing, delivery, and thread
                history.
              </p>
              <p className="text-sm leading-6 text-white/65">
                Move between analytics and active conversations without leaving
                the same operator surface.
              </p>
            </div>
          </div>
        </SidebarHeader>

        <SidebarContent className="px-2">
          <SidebarGroup>
            <SidebarGroupLabel className="px-3 font-mono text-[11px] uppercase tracking-[0.28em] text-white/40">
              Navigate
            </SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {NAV_ITEMS.map((item) => {
                  const Icon = item.icon;
                  return (
                    <SidebarMenuItem key={item.href}>
                      <SidebarMenuButton
                        asChild
                        size="lg"
                        isActive={item.key === activeView}
                        className="rounded-2xl px-3 py-3 text-white/70 hover:bg-white/6 hover:text-white data-[active=true]:bg-white data-[active=true]:text-slate-950"
                      >
                        <Link href={item.href}>
                          <Icon className="size-4" />
                          <span>{item.label}</span>
                        </Link>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  );
                })}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>

          {sidebarContent ? (
            <>
              <SidebarSeparator className="mx-3 bg-white/10" />
              <SidebarGroup className="px-3 pb-4">{sidebarContent}</SidebarGroup>
            </>
          ) : null}
        </SidebarContent>

        <SidebarFooter className="gap-3 p-4">
          <div className="rounded-[1.5rem] border border-white/10 bg-white/[0.04] p-3">
            <div className="flex items-center gap-3">
              <Avatar className="size-10 border border-white/10">
                <AvatarFallback className="bg-white/10 font-mono text-xs uppercase tracking-[0.24em] text-white">
                  TX
                </AvatarFallback>
              </Avatar>
              <div>
                <p className="text-sm font-medium text-white">
                  Telnyx operator
                </p>
                <p className="text-xs text-white/60">
                  Secure session active
                </p>
              </div>
            </div>
          </div>

          <Button
            type="button"
            variant="outline"
            className="w-full justify-between rounded-2xl border-white/12 bg-white/[0.04] px-4 text-white shadow-none hover:bg-white/10 hover:text-white"
            onClick={onLogout}
            disabled={isLoggingOut}
          >
            <span>{isLoggingOut ? "Logging out..." : "Logout"}</span>
            <LogOut className="size-4" />
          </Button>
        </SidebarFooter>
      </Sidebar>

      <SidebarInset
        className={
          scrollMode === "content"
            ? "h-screen min-h-screen overflow-hidden bg-transparent"
            : "min-h-screen bg-transparent"
        }
      >
        <div
          className={
            scrollMode === "content"
              ? "flex h-screen min-h-0 flex-col overflow-hidden"
              : "flex min-h-screen flex-col"
          }
        >
          <header className="sticky top-0 z-20 shrink-0 border-b border-border/70 bg-background/84 backdrop-blur-xl">
            <div className="mx-auto flex w-full max-w-7xl items-start justify-between gap-4 px-4 py-4 sm:px-6 lg:px-8">
              <div className="flex items-start gap-3">
                <SidebarTrigger className="mt-1 rounded-full border border-border/80 bg-background shadow-sm md:hidden" />
                <div className="space-y-3">
                  <Badge
                    variant="outline"
                    className="rounded-full border-border/70 bg-background/70 px-3 py-1 font-mono text-[11px] uppercase tracking-[0.28em] text-muted-foreground"
                  >
                    {activeView === "dashboard" ? "Operations" : "Messaging"}
                  </Badge>
                  <div className="space-y-2">
                    <h1 className="font-display text-3xl font-semibold tracking-tight text-balance text-foreground sm:text-4xl">
                      {title}
                    </h1>
                    <p className="max-w-2xl text-sm leading-6 text-muted-foreground sm:text-base">
                      {description}
                    </p>
                  </div>
                </div>
              </div>

              {actions ? (
                <div className="hidden shrink-0 items-center gap-3 md:flex">
                  {actions}
                </div>
              ) : null}
            </div>

            {actions ? (
              <div className="mx-auto flex w-full max-w-7xl px-4 pb-4 md:hidden sm:px-6 lg:px-8">
                {actions}
              </div>
            ) : null}
          </header>

          <div
            className={
              scrollMode === "content"
                ? "mx-auto flex min-h-0 w-full max-w-7xl flex-1 flex-col overflow-hidden px-4 py-6 sm:px-6 lg:px-8"
                : "mx-auto flex w-full max-w-7xl flex-1 flex-col px-4 py-6 sm:px-6 lg:px-8"
            }
          >
            {children}
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
