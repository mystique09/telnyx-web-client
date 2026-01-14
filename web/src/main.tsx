import { createInertiaApp } from "@inertiajs/react";
import "./index.css";

import { createRoot } from "react-dom/client";

createInertiaApp({
  id: "app",
  resolve: (name) => {
    const pages = import.meta.glob("./Pages/**/*.tsx", { eager: true });
    return pages[`./Pages/${name}.tsx`];
  },
  setup({ el, App, props }) {
    createRoot(el).render(<App {...props} />);
  },
  defaults: {
    form: {
      recentlySuccessfulDuration: 5000,
    },
    prefetch: {
      cacheFor: "1m",
      hoverDelay: 150,
    },
    visitOptions: (_href, options) => {
      return {
        headers: {
          ...options.headers,
          "X-Custom-Header": "value",
        },
      };
    },
  },
});
