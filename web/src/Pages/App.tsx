import DashboardFeature from "@/features/dashboard/DashboardFeature";
import { useFlash } from "@/hooks/use-flash";
import type { PropsWithFlash } from "@/lib/types";

function App({ flash }: PropsWithFlash) {
  useFlash(flash);

  return <DashboardFeature />;
}

export default App;
