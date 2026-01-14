import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import tsconfigPaths from "vite-tsconfig-paths";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss(), tsconfigPaths()],
  server: {
    proxy: {
      "/inertia": {
        target: "http://127.0.0.1:8080",
        changeOrigin: true,
      },
      "/inertia-version": {
        target: "http://127.0.0.1:8080",
        changeOrigin: true,
      },
    },
  },
});
