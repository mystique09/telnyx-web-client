import { Link, useForm } from "@inertiajs/react";
import { useMemo, useState, type FormEvent } from "react";
import { toast } from "sonner";
import { BarChart3, LogOut, MessageSquare, Phone, Plus } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { useFlash } from "@/hooks/use-flash";
import { seedConversations, seedPhoneNumbers, type PhoneNumber, USER_ID } from "@/lib/mock-messaging";
import type { PropsWithFlash } from "@/lib/types";

function App({ flash }: PropsWithFlash) {
  useFlash(flash);
  const { post: postLogout, processing: isLoggingOut } = useForm({});

  const [phoneNumbers, setPhoneNumbers] = useState<PhoneNumber[]>(seedPhoneNumbers);
  const [phoneNameInput, setPhoneNameInput] = useState("");
  const [phoneValueInput, setPhoneValueInput] = useState("");
  const [isAddPhoneDialogOpen, setIsAddPhoneDialogOpen] = useState(false);

  const totalMessages = useMemo(() => {
    return seedConversations.reduce((total, conversation) => total + conversation.messages.length, 0);
  }, []);

  function handleAddPhoneNumber(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const name = phoneNameInput.trim();
    const phone = phoneValueInput.trim();

    if (!name || !phone) {
      toast.error("Phone name and number are required.");
      return;
    }

    if (!/^\+?[1-9]\d{6,14}$/.test(phone)) {
      toast.error("Use a valid phone format, for example +13125551234.");
      return;
    }

    if (phoneNumbers.some((item) => item.phone === phone)) {
      toast.error("That phone number already exists.");
      return;
    }

    setPhoneNumbers((prev) => [
      ...prev,
      {
        id: `phone-${Date.now()}`,
        userId: USER_ID,
        name,
        phone,
      },
    ]);

    setPhoneNameInput("");
    setPhoneValueInput("");
    setIsAddPhoneDialogOpen(false);
    toast.success("Phone number added.");
  }

  return (
    <div className="h-screen w-full bg-background">
      <div className="flex h-full w-full flex-col overflow-hidden bg-background md:flex-row">
        <aside className="w-full border-b bg-card md:w-80 md:border-r md:border-b-0">
          <div className="space-y-4 p-4">
            <div className="space-y-1">
              <p className="text-sm font-medium">Telnyx Web Client</p>
              <p className="text-xs text-muted-foreground">Messaging workspace</p>
            </div>

            <div className="space-y-2">
              <Button asChild className="w-full justify-start gap-2">
                <Link href="/">
                  <BarChart3 className="size-4" />
                  Dashboard
                </Link>
              </Button>
              <Button asChild variant="outline" className="w-full justify-start gap-2">
                <Link href="/conversations">
                  <MessageSquare className="size-4" />
                  Conversations
                </Link>
              </Button>
              <Button
                type="button"
                variant="ghost"
                className="w-full justify-start gap-2"
                onClick={() => postLogout("/auth/logout")}
                disabled={isLoggingOut}
              >
                <LogOut className="size-4" />
                {isLoggingOut ? "Logging out..." : "Logout"}
              </Button>
            </div>
          </div>

          <Separator />
        </aside>

        <main className="flex-1 overflow-y-auto p-4 md:p-6">
          <div className="mx-auto w-full max-w-6xl space-y-6">
            <div>
              <h1 className="text-2xl font-semibold">Dashboard Analytics</h1>
              <p className="text-sm text-muted-foreground">
                Placeholder analytics for now. Backend wiring can be added later.
              </p>
            </div>

            <div className="grid gap-4 md:grid-cols-3">
              <Card>
                <CardHeader>
                  <CardTitle>Total Conversations</CardTitle>
                  <CardDescription>Current active threads</CardDescription>
                </CardHeader>
                <CardContent>
                  <p className="text-3xl font-semibold">{seedConversations.length}</p>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>Total Messages</CardTitle>
                  <CardDescription>Inbound + outbound volume</CardDescription>
                </CardHeader>
                <CardContent>
                  <p className="text-3xl font-semibold">{totalMessages}</p>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>Phone Numbers</CardTitle>
                  <CardDescription>Numbers owned by this account</CardDescription>
                </CardHeader>
                <CardContent>
                  <p className="text-3xl font-semibold">{phoneNumbers.length}</p>
                </CardContent>
              </Card>
            </div>

            <Card>
              <CardHeader className="gap-3">
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <CardTitle className="flex items-center gap-2">
                    <Phone className="size-4" />
                    Manage Phone Numbers
                  </CardTitle>

                  <Dialog
                    open={isAddPhoneDialogOpen}
                    onOpenChange={(open) => {
                      setIsAddPhoneDialogOpen(open);
                      if (!open) {
                        setPhoneNameInput("");
                        setPhoneValueInput("");
                      }
                    }}
                  >
                    <DialogTrigger asChild>
                      <Button type="button" size="sm" className="gap-2">
                        <Plus className="size-4" />
                        Add number
                      </Button>
                    </DialogTrigger>
                    <DialogContent>
                      <DialogHeader>
                        <DialogTitle>Add Phone Number</DialogTitle>
                        <DialogDescription>
                          Create a new phone number entry linked to this user.
                        </DialogDescription>
                      </DialogHeader>

                      <form id="add-phone-number-form" onSubmit={handleAddPhoneNumber} className="space-y-4">
                        <div className="space-y-2">
                          <Label htmlFor="phone-name">Number Name</Label>
                          <Input
                            id="phone-name"
                            placeholder="Primary Support"
                            value={phoneNameInput}
                            onChange={(event) => setPhoneNameInput(event.target.value)}
                          />
                        </div>

                        <div className="space-y-2">
                          <Label htmlFor="phone-value">Phone Number</Label>
                          <Input
                            id="phone-value"
                            placeholder="+13125551234"
                            value={phoneValueInput}
                            onChange={(event) => setPhoneValueInput(event.target.value)}
                          />
                        </div>
                      </form>

                      <DialogFooter>
                        <Button type="button" variant="outline" onClick={() => setIsAddPhoneDialogOpen(false)}>
                          Cancel
                        </Button>
                        <Button type="submit" form="add-phone-number-form" className="gap-2">
                          <Plus className="size-4" />
                          Save Number
                        </Button>
                      </DialogFooter>
                    </DialogContent>
                  </Dialog>
                </div>
                <CardDescription>
                  Add additional phone numbers for this user. Stored with `user_id`.
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-2">
                {phoneNumbers.map((phoneNumber) => (
                  <div
                    key={phoneNumber.id}
                    className="flex items-center justify-between rounded-lg border px-3 py-2"
                  >
                    <div>
                      <p className="text-sm font-medium">{phoneNumber.name}</p>
                      <p className="text-xs text-muted-foreground">{phoneNumber.phone}</p>
                    </div>
                    <Badge variant="outline">user_id linked</Badge>
                  </div>
                ))}
              </CardContent>
            </Card>
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;
