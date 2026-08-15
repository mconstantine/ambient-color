import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import devtools from "solid-devtools/vite";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [
    devtools(),
    solidPlugin(),
    tailwindcss(),
  ],
  server: {
    host: "0.0.0.0",
    port: 3000,
    fs: {
      allow: [".."],
    },
  },
  build: {
    target: "esnext",
  },
});
