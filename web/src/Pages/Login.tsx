"use client";

import { AuthShell } from "@/components/auth-shell";
import { PasswordField } from "@/components/password-field";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useFlash } from "@/hooks/use-flash";
import type { PropsWithFlash } from "@/lib/types";
import { Link, useForm } from "@inertiajs/react";
import { AlertCircle, Mail } from "lucide-react";
import { useState } from "react";
import { z } from "zod";

interface LoginPageProps extends PropsWithFlash {
  errors?: {
    email?: string;
    password?: string;
    general?: string;
  };
}

const loginSchema = z.object({
  email: z
    .email("Please enter a valid email address")
    .min(1, "Email is required"),
  password: z.string().min(1, "Password is required"),
});

type LoginFormValues = z.infer<typeof loginSchema>;

function Login({ errors, flash }: LoginPageProps) {
  useFlash(flash);

  const [clientErrors, setClientErrors] = useState<
    Partial<Record<keyof LoginFormValues, string>>
  >({});

  const { data, setData, post, processing } = useForm<LoginFormValues>({
    email: "",
    password: "",
  });

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    const result = loginSchema.safeParse(data);
    if (!result.success) {
      const formattedErrors: Record<string, string> = {};
      for (const issue of result.error.issues) {
        const field = issue.path[0] as string;
        formattedErrors[field] = issue.message;
      }
      setClientErrors(formattedErrors);
      return;
    }

    setClientErrors({});
    post("/auth/login");
  }

  const emailError = clientErrors.email || errors?.email;
  const passwordError = clientErrors.password || errors?.password;
  const generalError = errors?.general;

  return (
    <AuthShell
      eyebrow="Secure access"
      title="Sign in to your operator workspace"
      description="Use your admin credentials to manage routing, monitor live threads, and keep the Telnyx workspace in sync."
      supportingTitle="One control room for every active message."
      supportingDescription="The web client keeps analytics, phone number inventory, and conversation response work inside the same authenticated surface."
    >
      <div className="space-y-6">
          {generalError && (
          <Alert
            variant="destructive"
            className="rounded-[1.5rem] border-destructive/30 bg-destructive/5"
          >
            <AlertCircle className="size-4" />
            <AlertTitle>Unable to sign in</AlertTitle>
            <AlertDescription>{generalError}</AlertDescription>
          </Alert>
          )}
        <form onSubmit={handleSubmit} className="space-y-5">
          <div className="space-y-2">
            <label htmlFor="email" className="text-sm font-medium">
              Email address
              </label>
              <div className="relative">
              <Mail className="pointer-events-none absolute left-4 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  id="email"
                  type="email"
                  placeholder="name@example.com"
                className="h-12 rounded-2xl border-border/80 bg-background/80 pl-11"
                  value={data.email}
                  onChange={(e) => {
                    setData("email", e.target.value);
                    if (clientErrors.email)
                      setClientErrors((prev) => ({
                        ...prev,
                        email: undefined,
                      }));
                  }}
                />
              </div>
              {emailError && (
              <p className="text-sm text-destructive">{emailError}</p>
              )}
            </div>

          <div className="space-y-2">
            <div className="flex items-center justify-between gap-3">
              <label htmlFor="password" className="text-sm font-medium">
                Password
              </label>
              <Link
                href="/auth/forgot-password"
                className="text-sm font-medium text-foreground underline-offset-4 transition-colors hover:text-primary hover:underline"
              >
                Forgot password?
              </Link>
            </div>
            <PasswordField
                  id="password"
                  placeholder="Enter your password"
              className="h-12 rounded-2xl border-border/80 bg-background/80"
                  value={data.password}
                  onChange={(e) => {
                    setData("password", e.target.value);
                    if (clientErrors.password)
                      setClientErrors((prev) => ({
                        ...prev,
                        password: undefined,
                      }));
                  }}
            />
              {passwordError && (
              <p className="text-sm text-destructive">{passwordError}</p>
              )}
            </div>

          <Button
            type="submit"
            className="h-12 w-full rounded-2xl text-base"
            disabled={processing}
          >
              {processing ? "Signing in..." : "Sign in"}
            </Button>
          </form>

        <div className="rounded-[1.5rem] border border-border/80 bg-muted/35 p-4">
          <p className="text-sm font-medium text-foreground">
            Reserved for authenticated workspace operators.
          </p>
          <p className="mt-2 text-sm leading-6 text-muted-foreground">
            Signing in restores access to analytics, conversation history, and
            live delivery updates without leaving the same workspace.
          </p>
        </div>
      </div>
    </AuthShell>
  );
}

export default Login;
