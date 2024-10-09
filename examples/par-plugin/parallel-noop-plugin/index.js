import path from "node:path";
import { defineParallelPlugin } from "rolldown";

/** @type {import('rolldown').DefineParallelPluginResult<void>} */
export default defineParallelPlugin(
	path.resolve(import.meta.dirname, "./impl.js"),
);
