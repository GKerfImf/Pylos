import { defineConfig } from "vite";
import path from "path";
import react from "@vitejs/plugin-react";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
  plugins: [
    react(),
    viteStaticCopy({
      targets: [
        {
          src: "models/pylos.glb",
          dest: "models",
        },
      ],
    }),
  ],
  resolve: {
    alias: {
      src: "/src",
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, "index.html"),
      },
    },
    outDir: "../server/static",
    emptyOutDir: false,
  },
});
