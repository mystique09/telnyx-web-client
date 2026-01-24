import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useFlash } from "@/hooks/use-flash";
import type { PropsWithFlash } from "@/lib/types";

function App({ flash }: PropsWithFlash) {
  useFlash(flash);

  return (
    <div className="min-h-screen bg-linear-to-br from-gray-50 to-gray-100 p-8">
      <div className="mx-auto max-w-4xl">
        <h1 className="mb-8 text-4xl font-bold text-gray-900">Dashboard</h1>

        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <Card>
            <CardHeader>
              <CardTitle>Welcome</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-gray-600">
                Welcome to your dashboard. This is a simple landing page.
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Status</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-gray-600">
                System status: <span className="text-green-600">Online</span>
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Quick Actions</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-gray-600">
                Coming soon: quick actions and shortcuts.
              </p>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

export default App;
