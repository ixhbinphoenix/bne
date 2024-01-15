import { defineConfig } from "astro/config";
import compileServiceWorker from "./compileServiceWorker";

// https://astro.build/config
import preact from "@astrojs/preact";
import vercel from "@astrojs/vercel/serverless";
import node from "@astrojs/node"

export default defineConfig({
  output: "server",
  integrations: [preact()],
  server: {
    headers: {
      "Service-Worker-Allowed": "/"
    }
  },
  adapter: vercel({
    imageService: false
  }),
  vite: {
    plugins: [compileServiceWorker()]
  }
});
