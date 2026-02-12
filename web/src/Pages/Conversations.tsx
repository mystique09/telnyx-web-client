import ConversationsFeature from "@/features/conversations/ConversationsFeature";
import { useFlash } from "@/hooks/use-flash";
import type { PropsWithFlash } from "@/lib/types";

function Conversations({ flash }: PropsWithFlash) {
  useFlash(flash);

  return <ConversationsFeature />;
}

export default Conversations;
