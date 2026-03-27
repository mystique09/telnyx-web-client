import { type ReactNode } from "react";

import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { BrandMark } from "./brand-mark";

type AuthShellProps = {
  eyebrow: string;
  title: string;
  description: string;
  children: ReactNode;
  supportingTitle?: string;
  supportingDescription?: string;
  highlights?: unknown;
  footer?: ReactNode;
};

export function AuthShell({
  eyebrow,
  title,
  description,
  children,
}: AuthShellProps) {
  return (
    <div className="relative min-h-screen overflow-hidden bg-[linear-gradient(180deg,rgba(247,247,244,0.98)_0%,rgba(241,240,234,0.94)_100%)]">
      <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_top_left,rgba(26,56,74,0.09),transparent_38%),radial-gradient(circle_at_bottom_right,rgba(106,141,122,0.14),transparent_34%)]" />
      <div className="pointer-events-none absolute left-[6%] top-20 size-72 rounded-full bg-[#93a3b2]/20 blur-3xl animate-float-slow" />
      <div className="pointer-events-none absolute bottom-8 right-[8%] size-80 rounded-full bg-[#b4c4af]/20 blur-3xl animate-float-slow [animation-delay:-7s]" />

      <div className="relative mx-auto flex min-h-screen max-w-5xl items-center justify-center px-4 py-8 sm:px-6 lg:px-8">
        <section className="w-full max-w-xl">
          <div className="rounded-[2rem] border border-border/80 bg-background/90 p-1 shadow-[0_32px_110px_-52px_rgba(15,23,42,0.72)] backdrop-blur">
            <div className="rounded-[calc(2rem-0.25rem)] border border-border/70 bg-card/92 p-6 sm:p-8">
              <div className="space-y-6">
                <BrandMark />

                <div className="space-y-4">
                  <Badge
                    variant="outline"
                    className="rounded-full border-border/70 bg-background/70 px-3 py-1 font-mono text-[11px] uppercase tracking-[0.28em] text-muted-foreground"
                  >
                    {eyebrow}
                  </Badge>
                  <div className="space-y-3">
                    <h1 className="font-display text-3xl font-semibold tracking-tight text-balance text-foreground sm:text-4xl">
                      {title}
                    </h1>
                    <p className="text-sm leading-6 text-muted-foreground sm:text-base">
                      {description}
                    </p>
                  </div>
                </div>
              </div>

              <Separator className="my-6 bg-border/70" />
              {children}
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}
