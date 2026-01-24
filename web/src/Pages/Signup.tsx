"use client";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Link, useForm } from "@inertiajs/react";
import {
  AlertCircle,
  Eye,
  EyeOff,
  Lock,
  Mail,
  ShieldCheck,
} from "lucide-react";
import { useState } from "react";
import { z } from "zod";

interface SignupPageProps {
  errors?: {
    email?: string;
    password?: string;
    general?: string;
  };
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

function Signup({ errors }: SignupPageProps) {
  const [showPassword, setShowPassword] = useState(false);
  const [clientErrors, setClientErrors] = useState<
    Partial<Record<keyof SignupFormValues, string>>
  >({});
  const [isAdminSetup] = useState(true);

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

  if (!isAdminSetup) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-linear-to-br from-gray-50 to-gray-100 p-4">
        <Card className="w-full max-w-md shadow-lg">
          <CardHeader>
            <div className="flex flex-col items-center space-y-2">
              <div className="rounded-full bg-yellow-100 p-3">
                <ShieldCheck className="h-8 w-8 text-yellow-600" />
              </div>
              <CardTitle className="text-2xl font-bold">
                Admin Already Exists
              </CardTitle>
              <CardDescription className="text-center">
                An administrator account has already been created. Please log in
                with your existing credentials.
              </CardDescription>
            </div>
          </CardHeader>
          <CardContent>
            <Link href="/auth/login" className="block">
              <Button className="w-full">Go to Login</Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-linear-to-br from-gray-50 to-gray-100 p-4">
      <Card className="w-full max-w-md shadow-lg">
        <CardHeader className="space-y-1">
          <div className="flex flex-col items-center space-y-2">
            <div className="rounded-full bg-blue-100 p-3">
              <ShieldCheck className="h-8 w-8 text-blue-600" />
            </div>
            <CardTitle className="text-2xl font-bold">
              Create Admin Account
            </CardTitle>
            <CardDescription className="text-center">
              This is a one-time setup to create the administrator account. All
              other users will be created by the admin.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <div className="mb-4 rounded-lg border border-yellow-200 bg-yellow-50 p-3">
            <div className="flex items-start space-x-2">
              <AlertCircle className="h-4 w-4 shrink-0 text-yellow-600 mt-0.5" />
              <div className="text-sm text-yellow-800">
                <p className="font-semibold">Important:</p>
                <p>
                  This page will be hidden after the admin account is created.
                  Make sure to remember your credentials.
                </p>
              </div>
            </div>
          </div>

          {generalError && (
            <div className="mb-4 rounded-md bg-red-50 p-3 text-sm text-red-600">
              {generalError}
            </div>
          )}

          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label htmlFor="email" className="mb-2 block text-sm font-medium">
                Email
              </label>
              <div className="relative">
                <Mail className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
                <Input
                  id="email"
                  type="email"
                  placeholder="admin@example.com"
                  className="pl-10"
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
                <p className="mt-1 text-sm text-red-600">{emailError}</p>
              )}
            </div>

            <div>
              <label
                htmlFor="password"
                className="mb-2 block text-sm font-medium"
              >
                Password
              </label>
              <div className="relative">
                <Lock className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
                <Input
                  id="password"
                  type={showPassword ? "text" : "password"}
                  placeholder="Create a strong password"
                  className="pl-10 pr-10"
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
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-3 top-3 text-gray-400 hover:text-gray-600 focus:outline-none"
                >
                  {showPassword ? (
                    <EyeOff className="h-4 w-4" />
                  ) : (
                    <Eye className="h-4 w-4" />
                  )}
                </button>
              </div>
              {passwordError && (
                <p className="mt-1 text-sm text-red-600">{passwordError}</p>
              )}
            </div>

            <div>
              <label
                htmlFor="password_confirmation"
                className="mb-2 block text-sm font-medium"
              >
                Confirm Password
              </label>
              <div className="relative">
                <Lock className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
                <Input
                  id="password_confirmation"
                  type={showPassword ? "text" : "password"}
                  placeholder="Confirm your password"
                  className="pl-10"
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
              </div>
              {confirmPasswordError && (
                <p className="mt-1 text-sm text-red-600">
                  {confirmPasswordError}
                </p>
              )}
            </div>

            <Button type="submit" className="w-full" disabled={processing}>
              {processing ? "Creating account..." : "Create Admin Account"}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}

export default Signup;
