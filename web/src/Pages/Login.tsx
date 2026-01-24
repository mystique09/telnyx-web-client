import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Link, useForm } from "@inertiajs/react";
import { Eye, EyeOff, Lock, Mail } from "lucide-react";
import { useState } from "react";

interface LoginPageProps {
  errors?: {
    email?: string;
    password?: string;
    general?: string;
  };
}

function Login({ errors }: LoginPageProps) {
  const [showPassword, setShowPassword] = useState(false);

  const {
    data,
    setData,
    post,
    processing,
    errors: formErrors,
  } = useForm({
    email: "",
    password: "",
    general: "",
  });

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    post("/login");
  }

  // Merge prop errors with form errors (formErrors comes from Inertia)
  const emailError = formErrors.email || errors?.email;
  const passwordError = formErrors.password || errors?.password;
  const generalError = formErrors.general || errors?.general;

  return (
    <div className="flex min-h-screen items-center justify-center bg-linear-to-br from-gray-50 to-gray-100 p-4">
      <Card className="w-full max-w-md shadow-lg">
        <CardHeader className="space-y-1">
          <CardTitle className="text-2xl font-bold">Welcome back</CardTitle>
          <CardDescription>
            Enter your email and password to sign in to your account
          </CardDescription>
        </CardHeader>
        <CardContent>
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
                  placeholder="name@example.com"
                  className="pl-10"
                  value={data.email}
                  onChange={(e) => setData("email", e.target.value)}
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
                  placeholder="Enter your password"
                  className="pl-10 pr-10"
                  value={data.password}
                  onChange={(e) => setData("password", e.target.value)}
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

            <Button type="submit" className="w-full" disabled={processing}>
              {processing ? "Signing in..." : "Sign in"}
            </Button>
          </form>
        </CardContent>
        <CardFooter>
          <div className="text-sm text-gray-500">
            <Link
              type="button"
              href="/forgot-password"
              className="font-medium text-gray-900 underline underline-offset-4 hover:text-gray-700"
            >
              Forgot your password?
            </Link>
          </div>
        </CardFooter>
      </Card>
    </div>
  );
}

export default Login;
