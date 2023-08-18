import { rollup, InputOptions, OutputOptions } from "rollup";
import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser"

const compileServiceWorker = () => ({
  name: "compile-typescript-service-worker",
  async writeBundle(_options: any, _outputBundle: any) {
    const inputOptions: InputOptions = {
      input: "./src/pwa/serviceWorker.ts",
      plugins: [typescript(), terser()]
    };
    const outputOptions: OutputOptions = {
      file: "./.vercel/output/static/serviceWorker.js",
      format: "es",
      compact: true
    };
    const bundle = await rollup(inputOptions);
    await bundle.write(outputOptions);
    await bundle.close()
  }
});
export default compileServiceWorker