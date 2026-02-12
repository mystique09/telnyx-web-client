import DashboardFeature from "@/features/dashboard/DashboardFeature";
import type { DashboardPageProps } from "@/features/dashboard/types";
import { useFlash } from "@/hooks/use-flash";

function App(props: DashboardPageProps) {
  const { flash } = props;
  useFlash(flash);

  return <DashboardFeature pageProps={props} />;
}

export default App;
