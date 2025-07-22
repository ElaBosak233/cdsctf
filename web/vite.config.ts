import path from "node:path";

import tailwindcss from "@tailwindcss/vite";
import React from "@vitejs/plugin-react-swc";
import { defineConfig, loadEnv } from "vite";
import viteCompression from "vite-plugin-compression";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");

  return {
    server: {
      host: "0.0.0.0",
      proxy: {
        "/api": {
          target: env.VITE_DEV_API || "http://127.0.0.1:8888",
          changeOrigin: true,
          ws: true,
        },
      },
    },
    plugins: [React(), tailwindcss(), viteCompression()],
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./src"),
      },
    },
    assetsInclude: ["**/*.cdsx"],
  };
});
