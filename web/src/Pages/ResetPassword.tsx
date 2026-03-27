import { useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { AuthShell } from "@/components/auth-shell";
import { PasswordField } from "@/components/password-field";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Link } from "@inertiajs/react";
import { CheckCircle2 } from "lucide-react";

const resetPasswordSchema = z
  .object({
    password: z
      .string()
      .min(8, "Password must be at least 8 characters")
      .regex(
        /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/,
        "Password must contain at least one uppercase letter, one lowercase letter, and one number",
      ),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });

type ResetPasswordFormValues = z.infer<typeof resetPasswordSchema>;

function ResetPassword() {
  const [isSuccess, setIsSuccess] = useState(false);

  const form = useForm<ResetPasswordFormValues>({
    resolver: zodResolver(resetPasswordSchema),
    defaultValues: {
      password: "",
      confirmPassword: "",
    },
  });

  function onSubmit(values: ResetPasswordFormValues) {
    console.log("Password reset:", values);
    setIsSuccess(true);
  }

  if (isSuccess) {
    return (
      <AuthShell
        eyebrow="Recovery complete"
        title="Your password has been reset"
        description="Use the new password to return to the operator workspace."
        supportingTitle="Recovery finished cleanly."
        supportingDescription="The next step is straightforward: sign in again and pick up your messaging work where you left it."
      >
        <div className="space-y-6">
          <div className="rounded-[1.75rem] border border-emerald-200/80 bg-emerald-50/80 p-6 text-center">
            <div className="mx-auto flex size-16 items-center justify-center rounded-full bg-emerald-100 text-emerald-700">
              <CheckCircle2 className="size-8" />
            </div>
            <h3 className="mt-5 font-display text-2xl font-semibold tracking-tight text-foreground">
              Password reset successful
            </h3>
            <p className="mt-3 text-sm leading-6 text-muted-foreground">
              Your new credentials are active. Return to login and continue in
              the workspace.
            </p>
          </div>

          <Button asChild className="h-12 w-full rounded-2xl text-base">
            <Link href="/auth/login">Go to login</Link>
          </Button>
        </div>
      </AuthShell>
    );
  }

  return (
    <AuthShell
      eyebrow="Set new password"
      title="Choose a new admin password"
      description="Use a strong new password to complete the recovery flow and restore access."
      supportingTitle="Finish recovery with a credential you can trust."
      supportingDescription="A clear reset flow keeps operators moving while preserving the control expected from an admin workspace."
    >
      <div className="space-y-6">
          <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-5">
              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>New Password</FormLabel>
                    <PasswordField
                          placeholder="Enter your new password"
                      className="h-12 rounded-2xl border-border/80 bg-background/80"
                          {...field}
                    />
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="confirmPassword"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Confirm New Password</FormLabel>
                    <PasswordField
                          placeholder="Confirm your new password"
                      className="h-12 rounded-2xl border-border/80 bg-background/80"
                          {...field}
                    />
                    <FormMessage />
                  </FormItem>
                )}
              />

            <div className="rounded-[1.5rem] border border-border/80 bg-muted/35 p-4">
              <p className="text-sm font-medium text-foreground">
                Password requirements
              </p>
              <p className="mt-2 text-sm leading-6 text-muted-foreground">
                Use at least eight characters with uppercase, lowercase, and a
                number so the new credential clears validation immediately.
              </p>
            </div>

              <Button
                type="submit"
              className="h-12 w-full rounded-2xl text-base"
                disabled={form.formState.isSubmitting}
              >
                {form.formState.isSubmitting
                  ? "Resetting password..."
                  : "Reset password"}
              </Button>
            </form>
          </Form>
      </div>
    </AuthShell>
  );
}

export default ResetPassword;
