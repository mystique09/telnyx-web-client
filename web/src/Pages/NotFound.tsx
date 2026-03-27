import { BrandMark } from "@/components/brand-mark";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Link } from "@inertiajs/react";
import { ArrowLeft, Compass, Home, Search } from "lucide-react";

function NotFound() {
  return (
    <div className="relative min-h-screen overflow-hidden bg-[linear-gradient(180deg,rgba(247,247,244,0.98)_0%,rgba(240,239,234,0.94)_100%)]">
      <div className="pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute left-[8%] top-20 size-72 rounded-full bg-[#93a3b2]/18 blur-3xl animate-float-slow" />
        <div className="absolute bottom-12 right-[10%] size-80 rounded-full bg-[#b4c4af]/18 blur-3xl animate-float-slow [animation-delay:-6s]" />
      </div>

      <div className="relative mx-auto flex min-h-screen max-w-6xl flex-col px-4 py-6 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between gap-4">
          <BrandMark />
          <Badge
            variant="outline"
            className="rounded-full border-border/70 bg-background/70 px-3 py-1 font-mono text-[11px] uppercase tracking-[0.28em] text-muted-foreground"
          >
            Error state
          </Badge>
        </div>

        <div className="flex flex-1 items-center py-10">
          <div className="grid w-full gap-8 lg:grid-cols-[0.9fr_1.1fr] lg:items-end">
            <div className="space-y-6">
              <p className="font-mono text-[11px] uppercase tracking-[0.35em] text-muted-foreground">
                404
              </p>
              <div className="space-y-4">
                <h1 className="font-display text-5xl font-semibold tracking-tight text-balance text-foreground sm:text-6xl">
                  That route has gone quiet.
                </h1>
                <p className="max-w-xl text-base leading-7 text-muted-foreground sm:text-lg">
                  The page you requested does not exist or has moved. Use the
                  main workspace routes to get back to active operations.
                </p>
              </div>

              <div className="flex flex-col gap-3 sm:flex-row">
                <Button asChild className="h-12 rounded-full px-5 text-base">
                  <Link href="/" className="flex items-center gap-2">
                    <Home className="size-4" />
                    Return home
                  </Link>
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  className="h-12 rounded-full px-5 text-base"
                  onClick={() => window.history.back()}
                >
                  <ArrowLeft className="size-4" />
                  Go back
                </Button>
              </div>
            </div>

            <div className="rounded-[2rem] border border-border/80 bg-background/86 p-6 shadow-[0_36px_110px_-60px_rgba(15,23,42,0.55)] sm:p-8">
              <div className="space-y-5">
                <div className="flex size-16 items-center justify-center rounded-3xl bg-primary/8 text-primary">
                  <Compass className="size-7" />
                </div>
                <div className="space-y-2">
                  <h2 className="font-display text-2xl font-semibold tracking-tight text-foreground">
                    Try one of these paths
                  </h2>
                  <p className="text-sm leading-6 text-muted-foreground">
                    The utility pages, dashboard, and conversations surface all
                    share the same navigation system now. These routes will get
                    you moving again.
                  </p>
                </div>
              </div>

              <div className="mt-8 space-y-3">
                <div className="rounded-[1.5rem] border border-border/80 bg-card/90 p-4">
                  <div className="flex items-start gap-3">
                    <Home className="mt-0.5 size-4 text-muted-foreground" />
                    <div>
                      <p className="text-sm font-medium text-foreground">
                        Dashboard
                      </p>
                      <p className="mt-1 text-sm leading-6 text-muted-foreground">
                        Review workspace analytics, total message volume, and
                        phone number inventory.
                      </p>
                    </div>
                  </div>
                </div>

                <div className="rounded-[1.5rem] border border-border/80 bg-card/90 p-4">
                  <div className="flex items-start gap-3">
                    <Search className="mt-0.5 size-4 text-muted-foreground" />
                    <div>
                      <p className="text-sm font-medium text-foreground">
                        Conversations
                      </p>
                      <p className="mt-1 text-sm leading-6 text-muted-foreground">
                        Jump into active threads, older messages, and sent media
                        without leaving the workspace.
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default NotFound;
