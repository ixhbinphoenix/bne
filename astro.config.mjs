import { defineConfig } from 'astro/config';

// https://astro.build/config
import preact from "@astrojs/preact";

import vercel from "@astrojs/vercel/serverless";

export default defineConfig({
  output: 'server',
  integrations: [preact()],
  adapter: vercel()
});