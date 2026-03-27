import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { AuthShell } from "@/components/auth-shell";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Mail, ArrowLeft } from "lucide-react";
import { Link } from "@inertiajs/react";

const forgotPasswordSchema = z.object({
  email: z.email("Invalid email address"),
});

type ForgotPasswordFormValues = z.infer<typeof forgotPasswordSchema>;

function ForgotPassword() {
  const form = useForm<ForgotPasswordFormValues>({
    resolver: zodResolver(forgotPasswordSchema),
    defaultValues: {
      email: "",
    },
  });

  function onSubmit(values: ForgotPasswordFormValues) {
    console.log("Password reset request for:", values.email);
  }

  return (
    <AuthShell
      eyebrow="Recovery"
      title="Request a password reset"
      description="Enter the admin email attached to this workspace and we’ll start the reset flow."
      supportingTitle="Keep account recovery calm and unmistakable."
      supportingDescription="Recovery should be short, explicit, and secure so operators can get back into the workspace quickly."
    >
      <div className="space-y-6">
          <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-5">
              <FormField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Email address</FormLabel>
                    <FormControl>
                      <div className="relative">
                      <Mail className="pointer-events-none absolute left-4 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
                        <Input
                          placeholder="name@example.com"
                          type="email"
                        className="h-12 rounded-2xl border-border/80 bg-background/80 pl-11"
                          {...field}
                        />
                      </div>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Button
                type="submit"
              className="h-12 w-full rounded-2xl text-base"
                disabled={form.formState.isSubmitting}
              >
                {form.formState.isSubmitting
                  ? "Sending email..."
                  : "Send reset link"}
              </Button>
            </form>
          </Form>

        <div className="rounded-[1.5rem] border border-border/80 bg-muted/35 p-4">
          <p className="text-sm font-medium text-foreground">Before you send</p>
          <p className="mt-2 text-sm leading-6 text-muted-foreground">
            Use the admin email for this workspace. Reset links should only be
            requested from a trusted environment.
          </p>
        </div>

        <Button
          asChild
          variant="ghost"
          className="h-11 justify-start rounded-2xl px-3 text-foreground"
        >
          <Link href="/auth/login" className="flex items-center gap-2">
            <ArrowLeft className="size-4" />
            Back to login
          </Link>
        </Button>
      </div>
    </AuthShell>
  );
}

export default ForgotPassword;
