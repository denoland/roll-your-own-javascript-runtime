import * as esbuild from "npm:esbuild";
import { denoPlugins } from "jsr:@luca/esbuild-deno-loader";

const result = await esbuild.build({
  plugins: [...denoPlugins()],
  entryPoints: [
    "./bark.js",
    "./workerMain.js",
    "./nodeApiUsage.js",
    "./npmUsage.js",
  ],
  outdir: "./bundles",
  bundle: true,
  format: "esm",
});

esbuild.stop();
