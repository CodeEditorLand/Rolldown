// oxlint-disable
import { workspaceRoot } from "@rolldown/testing";

import * as selfExports from "./constants.js";

export const REPO_ROOT = workspaceRoot();

if (process.argv[1] === import.meta.filename) {
	// If this file is executed directly, print the exports
	console.log(selfExports);
}
