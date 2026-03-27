import { cn } from "@/lib/utils";

type BrandMarkProps = {
  className?: string;
  inverted?: boolean;
};

export function BrandMark({ className, inverted = false }: BrandMarkProps) {
  return (
    <div className={cn("flex items-center gap-3", className)}>
      <div
        className={cn(
          "relative flex size-12 shrink-0 items-center justify-center overflow-hidden rounded-[1.35rem] border shadow-[0_24px_60px_-32px_rgba(15,23,42,0.7)]",
          inverted
            ? "border-white/10 bg-white/[0.06]"
            : "border-border/80 bg-background/90",
        )}
      >
        <span
          className={cn(
            "absolute inset-x-3 top-[18px] h-px rounded-full",
            inverted ? "bg-white/18" : "bg-primary/18",
          )}
        />
        <span
          className={cn(
            "absolute inset-x-3 bottom-[18px] h-px rounded-full",
            inverted ? "bg-white/18" : "bg-primary/18",
          )}
        />
        <span
          className={cn(
            "absolute left-[15px] top-3.5 h-6 w-px rounded-full",
            inverted ? "bg-white/18" : "bg-primary/18",
          )}
        />
        <span
          className={cn(
            "absolute right-[15px] top-3.5 h-6 w-px rounded-full",
            inverted ? "bg-white/18" : "bg-primary/18",
          )}
        />
        <span
          className={cn(
            "absolute left-[11px] top-[11px] size-2 rounded-full",
            inverted ? "bg-white/90" : "bg-primary",
          )}
        />
        <span
          className={cn(
            "absolute right-[11px] top-[11px] size-2 rounded-full",
            inverted ? "bg-white/55" : "bg-primary/50",
          )}
        />
        <span
          className={cn(
            "absolute bottom-[11px] left-[11px] size-2 rounded-full",
            inverted ? "bg-white/55" : "bg-primary/50",
          )}
        />
        <span
          className={cn(
            "absolute bottom-[11px] right-[11px] size-2 rounded-full",
            inverted ? "bg-white/90" : "bg-primary",
          )}
        />
      </div>

      <div className="space-y-1">
        <p
          className={cn(
            "font-display text-base font-semibold tracking-tight",
            inverted ? "text-white" : "text-foreground",
          )}
        >
          Telnyx
        </p>
        <p
          className={cn(
            "font-mono text-[11px] uppercase tracking-[0.28em]",
            inverted ? "text-white/60" : "text-muted-foreground",
          )}
        >
          Web Client
        </p>
      </div>
    </div>
  );
}
