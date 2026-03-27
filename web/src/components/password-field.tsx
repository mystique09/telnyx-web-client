import { forwardRef, useState } from "react";
import { Eye, EyeOff, Lock } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

type PasswordFieldProps = React.ComponentProps<typeof Input> & {
  showLabel?: string;
  hideLabel?: string;
};

export const PasswordField = forwardRef<HTMLInputElement, PasswordFieldProps>(
  function PasswordField(
    {
      className,
      showLabel = "Show password",
      hideLabel = "Hide password",
      ...props
    },
    ref,
  ) {
    const [isVisible, setIsVisible] = useState(false);

    return (
      <div className="relative">
        <Lock className="pointer-events-none absolute left-4 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          ref={ref}
          type={isVisible ? "text" : "password"}
          className={cn("pl-11 pr-12", className)}
          {...props}
        />
        <Button
          type="button"
          variant="ghost"
          size="icon"
          className="absolute right-1.5 top-1.5 size-9 rounded-xl text-muted-foreground hover:text-foreground"
          onClick={() => setIsVisible((current) => !current)}
          aria-label={isVisible ? hideLabel : showLabel}
          aria-pressed={isVisible}
        >
          {isVisible ? <EyeOff className="size-4" /> : <Eye className="size-4" />}
        </Button>
      </div>
    );
  },
);
