// @ts-ignore: `@codspeed/vitest-plugin` doesn't specify `types` in `package.json#exports`.
import codspeedPlugin from "@codspeed/vitest-plugin";
import { defineConfig } from "vitest/config";

export default defineConfig({
	plugins: process.env.CI ? [codspeedPlugin()] : [],
});
