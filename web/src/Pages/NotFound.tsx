import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Link } from "@inertiajs/react";
import { ArrowLeft, Home } from "lucide-react";

function NotFound() {
  return (
    <div className="relative flex min-h-screen items-center justify-center overflow-hidden bg-neutral-50/50 p-4">
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute -top-40 -right-40 h-80 w-80 rounded-full bg-neutral-200/30 blur-3xl" />
        <div className="absolute -bottom-40 -left-40 h-80 w-80 rounded-full bg-neutral-200/30 blur-3xl" />
        <div className="absolute top-1/2 left-1/2 h-96 w-96 -translate-x-1/2 -translate-y-1/2 rounded-full bg-neutral-100/50 blur-3xl" />
      </div>

      <div className="absolute inset-0 overflow-hidden bg-neutral-50/50" />

      <Card className="relative z-10 w-full max-w-md bg-white/80 shadow-xl backdrop-blur-sm gap-1">
        <CardHeader className="space-y-6 pb-2 text-center">
          <div className="relative mx-auto">
            <div className="absolute inset-0 rounded-full bg-amber-100/50 blur-xl" />
            <div className="relative flex h-20 w-20 items-center justify-center rounded-full bg-linear-to-br from-amber-50 to-orange-50 shadow-inner ring-1 ring-amber-200/50">
              <span className="text-3xl font-bold text-amber-600">404</span>
            </div>
          </div>

          <div className="space-y-1">
            <CardTitle className="text-2xl font-semibold tracking-tight text-neutral-900">
              Page not found
            </CardTitle>
            <CardDescription className="text-sm text-neutral-500">
              Oops! This page has wandered off into the void.
            </CardDescription>
          </div>
        </CardHeader>

        <CardContent className="space-y-1">
          <div className="rounded-lg bg-neutral-50/80 p-4 text-center">
            <p className="text-sm leading-relaxed text-neutral-600">
              The page you&apos;re looking for doesn&apos;t exist or has been
              moved. Don&apos;t worry, it happens to the best of us!
            </p>
          </div>

          <div className="py-2 flex flex-col gap-2">
            <p className="text-center text-xs font-medium uppercase tracking-wider text-neutral-400">
              What you can do
            </p>

            <div className="grid gap-2">
              <Link href="/" className="block">
                <Button
                  variant="outline"
                  className="w-full gap-2 bg-neutral-900 text-black transition-all duration-200 hover:bg-neutral-800 hover:shadow-lg active:scale-[0.98]"
                >
                  <Home className="h-4 w-4" />
                  Home
                </Button>
              </Link>

              <Button
                variant="default"
                className="w-full gap-2 text-black bg-white transition-all duration-200 hover:bg-neutral-50 active:scale-[0.98] rounded-none"
                onClick={() => window.history.back()}
              >
                <ArrowLeft className="h-4 w-4" />
                Go Back
              </Button>
            </div>
          </div>

          <p className="text-center text-xs text-neutral-400 pt-6">
            Need help? Try navigating from the homepage or contact support if
            the problem persists.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}

export default NotFound;
