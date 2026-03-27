"use client";

import { AuthShell } from "@/components/auth-shell";
import { PasswordField } from "@/components/password-field";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useFlash } from "@/hooks/use-flash";
import type { PropsWithFlash } from "@/lib/types";
import { Link, useForm } from "@inertiajs/react";
import { AlertCircle, Mail, ShieldCheck } from "lucide-react";
import { useState } from "react";
import { z } from "zod";

interface SignupPageProps extends PropsWithFlash {
  errors?: {
    email?: string;
    password?: string;
    general?: string;
  };
  adminAlreadyExists?: boolean;
}

// Client-side validation schema with user-friendly messages
const signupSchema = z
  .object({
    email: z
      .email("Please enter a valid email address")
      .min(1, "Email is required"),
    password: z
      .string()
      .min(8, "Password must be at least 8 characters long")
      .regex(
        /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/,
        "Password must contain at least one uppercase letter, one lowercase letter, and one number",
      ),
    password_confirmation: z.string().min(1, "Please confirm your password"),
  })
  .refine((data) => data.password === data.password_confirmation, {
    message: "Passwords do not match",
    path: ["password_confirmation"],
  });

type SignupFormValues = z.infer<typeof signupSchema>;

function Signup({
  errors,
  flash,
  adminAlreadyExists = false,
}: SignupPageProps) {
  useFlash(flash);

  const [clientErrors, setClientErrors] = useState<
    Partial<Record<keyof SignupFormValues, string>>
  >({});

  const { data, setData, post, processing } = useForm<SignupFormValues>({
    email: "",
    password: "",
    password_confirmation: "",
  });

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    // Client-side validation with friendly messages
    const result = signupSchema.safeParse(data);
    if (!result.success) {
      const formattedErrors: Record<string, string> = {};
      for (const issue of result.error.issues) {
        const field = issue.path[0] as string;
        formattedErrors[field] = issue.message;
      }
      setClientErrors(formattedErrors);
      return;
    }

    // Clear client errors and submit
    setClientErrors({});
    post("/auth/signup");
  }

  // Merge prop errors (from server) with client errors
  const emailError = clientErrors.email || errors?.email;
  const passwordError = clientErrors.password || errors?.password;
  const confirmPasswordError = clientErrors.password_confirmation;
  const generalError = errors?.general;

  if (adminAlreadyExists) {
    return (
      <AuthShell
        eyebrow="Admin status"
        title="Admin access is already configured"
        description="This environment already has an administrator account. Sign in with existing credentials to continue."
        supportingTitle="The workspace is already claimed."
        supportingDescription="Admin creation is intended as a one-time bootstrap step before operators move into live messaging work."
      >
        <div className="space-y-6">
          <Alert className="rounded-[1.5rem] border-border/80 bg-muted/35">
            <ShieldCheck className="size-4" />
            <AlertTitle>Setup complete</AlertTitle>
            <AlertDescription>
              Use the existing admin account to manage phone numbers,
              conversations, and delivery operations.
            </AlertDescription>
          </Alert>

          <Button asChild className="h-12 w-full rounded-2xl text-base">
            <Link href="/auth/login">Go to login</Link>
          </Button>
        </div>
      </AuthShell>
    );
  }

  return (
    <AuthShell
      eyebrow="Initial setup"
      title="Create the first admin account"
      description="This account becomes the administrative entry point for the workspace and can provision the rest of the operator workflow."
      supportingTitle="Bootstrap the workspace with one durable admin identity."
      supportingDescription="Create the initial administrator account, then return here only for sign-in and password recovery."
    >
      <div className="space-y-6">
        <Alert className="rounded-[1.5rem] border-amber-200/70 bg-amber-50/80">
          <AlertCircle className="size-4 text-amber-700" />
          <AlertTitle className="text-amber-900">One-time admin step</AlertTitle>
          <AlertDescription className="text-amber-900/80">
            Once the first admin account exists, this setup flow should no
            longer be part of the normal operator path. Keep these credentials
            secure.
          </AlertDescription>
        </Alert>

          {generalError && (
          <Alert
            variant="destructive"
            className="rounded-[1.5rem] border-destructive/30 bg-destructive/5"
          >
            <AlertCircle className="size-4" />
            <AlertTitle>Unable to create admin</AlertTitle>
            <AlertDescription>{generalError}</AlertDescription>
          </Alert>
          )}

        <form onSubmit={handleSubmit} className="space-y-5">
          <div className="space-y-2">
            <label htmlFor="email" className="text-sm font-medium">
              Admin email
              </label>
              <div className="relative">
              <Mail className="pointer-events-none absolute left-4 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  id="email"
                  type="email"
                  placeholder="admin@example.com"
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
            <label htmlFor="password" className="text-sm font-medium">
              Password
              </label>
            <PasswordField
                  id="password"
                  placeholder="Create a strong password"
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

          <div className="space-y-2">
            <label htmlFor="password_confirmation" className="text-sm font-medium">
              Confirm password
              </label>
            <PasswordField
                  id="password_confirmation"
                  placeholder="Confirm your password"
              className="h-12 rounded-2xl border-border/80 bg-background/80"
                  value={data.password_confirmation}
                  onChange={(e) => {
                    setData("password_confirmation", e.target.value);
                    if (clientErrors.password_confirmation)
                      setClientErrors((prev) => ({
                        ...prev,
                        password_confirmation: undefined,
                      }));
                  }}
            />
              {confirmPasswordError && (
              <p className="text-sm text-destructive">
                  {confirmPasswordError}
                </p>
              )}
            </div>

          <div className="rounded-[1.5rem] border border-border/80 bg-muted/35 p-4">
            <p className="text-sm font-medium text-foreground">Password rules</p>
            <p className="mt-2 text-sm leading-6 text-muted-foreground">
              Use at least eight characters, including an uppercase letter, a
              lowercase letter, and a number.
            </p>
          </div>

          <Button
            type="submit"
            className="h-12 w-full rounded-2xl text-base"
            disabled={processing}
          >
              {processing ? "Creating account..." : "Create Admin Account"}
            </Button>
          </form>

        <div className="flex items-center justify-between gap-3 text-sm">
          <p className="text-muted-foreground">Already have access?</p>
          <Link
            href="/auth/login"
            className="font-medium text-foreground underline-offset-4 transition-colors hover:text-primary hover:underline"
          >
            Back to login
          </Link>
        </div>
      </div>
    </AuthShell>
  );
}

export default Signup;
