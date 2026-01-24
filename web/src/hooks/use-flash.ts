import type { PropsWithFlash } from "@/lib/types";
import { useEffect } from "react";
import { toast } from "sonner";

export function useFlash(flash?: PropsWithFlash["flash"]) {
  useEffect(() => {
    if (flash?.type && flash?.message) {
      if (flash.type === "success") {
        toast.success(flash?.message);
      } else if (flash.type === "error") {
        toast.error(flash?.message);
      }
    }
  }, [flash]);
}
