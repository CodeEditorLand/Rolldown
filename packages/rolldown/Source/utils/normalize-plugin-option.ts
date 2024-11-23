import type { RolldownPlugin } from "../plugin";
import type { OutputOptions, OutputPlugin } from "../rollup-types";
import type { InputOptions } from "../types/input-options";
import { asyncFlatten } from "./async-flatten";

export const normalizePluginOption: {
	(plugins: InputOptions["plugins"]): Promise<RolldownPlugin[]>;
	(plugins: OutputOptions["plugins"]): Promise<OutputPlugin[]>;
	(plugins: unknown): Promise<any[]>;
} = async (plugins: any) => (await asyncFlatten([plugins])).filter(Boolean);
