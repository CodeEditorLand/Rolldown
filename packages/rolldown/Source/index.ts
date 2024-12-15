import { version } from "../package.json";
import { build, type BuildOptions } from "./api/build";
import { rolldown } from "./api/rolldown";
import { RolldownBuild } from "./api/rolldown/rolldown-build";
import { watch } from "./api/watch";
import { RolldownWatcher } from "./api/watch/watch-emitter";
import { PreRenderedChunk } from "./binding";
import type {
	ExternalOption,
	InputOption,
	InputOptions,
	JsxOptions,
} from "./options/input-options";
import { NormalizedInputOptions } from "./options/normalized-input-options";
import {
	InternalModuleFormat,
	NormalizedOutputOptions,
} from "./options/normalized-output-options";
import type { ModuleFormat, OutputOptions } from "./options/output-options";
import { WatchOptions } from "./options/watch-options";
import type {
	AsyncPluginHooks,
	CustomPluginOptions,
	FunctionPluginHooks,
	ImportKind,
	LoadResult,
	ModuleOptions,
	ModuleType,
	ObjectHook,
	ParallelPluginHooks,
	PartialResolvedId,
	Plugin,
	ResolvedId,
	ResolveIdResult,
	RolldownPlugin,
	SourceDescription,
	TransformResult,
} from "./plugin";
import { MinimalPluginContext } from "./plugin/minimal-plugin-context";
import { DefineParallelPluginResult } from "./plugin/parallel-plugin";
import {
	EmittedAsset,
	EmittedFile,
	PluginContext,
} from "./plugin/plugin-context";
import { TransformPluginContext } from "./plugin/transform-plugin-context";
import { ConfigExport } from "./types/config-export";
import { ModuleInfo } from "./types/module-info";
import { OutputBundle } from "./types/output-bundle";
import type { RolldownOptions } from "./types/rolldown-options";
import {
	RolldownOutput,
	RolldownOutputAsset,
	RolldownOutputChunk,
	RolldownRenderedChunk,
	SourceMap,
} from "./types/rolldown-output";
import { ExistingRawSourceMap, SourceMapInput } from "./types/sourcemap";
import { PartialNull } from "./types/utils";
import { defineConfig } from "./utils/define-config";

export { defineConfig, rolldown, watch, build };
export const VERSION: string = version;

export type {
	RolldownOutputAsset,
	RolldownOutputChunk,
	RolldownOptions,
	RolldownOutput,
	RolldownBuild,
	InputOptions,
	NormalizedInputOptions,
	OutputOptions,
	NormalizedOutputOptions,
	Plugin,
	RolldownPlugin,
	DefineParallelPluginResult,
	ConfigExport,
	ImportKind,
	InputOption,
	ExternalOption,
	ModuleFormat,
	ModuleType,
	InternalModuleFormat,
	LoadResult,
	TransformResult,
	ResolveIdResult,
	PluginContext,
	TransformPluginContext,
	ObjectHook,
	PreRenderedChunk,
	SourceMap,
	SourceDescription,
	PartialNull,
	PartialResolvedId,
	ResolvedId,
	ModuleOptions,
	ModuleInfo,
	MinimalPluginContext,
	EmittedFile,
	EmittedAsset,
	CustomPluginOptions,
	AsyncPluginHooks,
	ParallelPluginHooks,
	FunctionPluginHooks,
	ExistingRawSourceMap,
	SourceMapInput,
	OutputBundle,
	JsxOptions,
	WatchOptions,
	RolldownWatcher,
	BuildOptions,
};

// Exports for compatibility

export type {
	RolldownOutput as RollupOutput,
	RolldownOptions as RollupOptions,
	RolldownBuild as RollupBuild,
	RolldownOutputChunk as OutputChunk,
	RolldownOutputAsset as OutputAsset,
	RolldownRenderedChunk as RenderedChunk,
	RolldownWatcher as RollupWatcher,
};
export type { RollupError, RollupLog, LoggingFunction } from "./rollup";
