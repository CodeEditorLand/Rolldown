import { version } from "../package.json";
import { build, type BuildOptions } from "./api/build";
import { rolldown } from "./api/rolldown";
import { RolldownBuild } from "./api/rolldown/rolldown-build";
import { watch } from "./api/watch";
import { RolldownWatcher } from "./api/watch/watch-emitter";
import { PreRenderedChunk } from "./binding";
import type { LogOrStringHandler } from "./log/logging";
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
import type {
	ModuleFormat,
	OutputOptions,
	PreRenderedAsset,
} from "./options/output-options";
import { WatchOptions } from "./options/watch-options";
import type {
	AsyncPluginHooks,
	CustomPluginOptions,
	FunctionPluginHooks,
	HookFilterExtension,
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
	RolldownPluginOption,
	SourceDescription,
	TransformResult,
} from "./plugin";
import type {
	HookFilter,
	ModuleTypeFilter,
	StringFilter,
} from "./plugin/hook-filter";
import {
	MinimalPluginContext,
	PluginContextMeta,
} from "./plugin/minimal-plugin-context";
import { DefineParallelPluginResult } from "./plugin/parallel-plugin";
import {
	EmittedAsset,
	EmittedFile,
	GetModuleInfo,
	PluginContext,
} from "./plugin/plugin-context";
import { TransformPluginContext } from "./plugin/transform-plugin-context";
import { ConfigExport } from "./types/config-export";
import { ModuleInfo } from "./types/module-info";
import { OutputBundle } from "./types/output-bundle";
import type { RolldownOptions } from "./types/rolldown-options";
import {
	OutputAsset,
	OutputChunk,
	RenderedChunk,
	RenderedModule,
	RolldownOutput,
	SourceMap,
} from "./types/rolldown-output";
import { ExistingRawSourceMap, SourceMapInput } from "./types/sourcemap";
import { PartialNull } from "./types/utils";
import { defineConfig } from "./utils/define-config";

export { defineConfig, rolldown, watch, build };
export const VERSION: string = version;

export type {
	OutputAsset,
	OutputChunk,
	RolldownOptions,
	RolldownOutput,
	RolldownBuild,
	InputOptions,
	NormalizedInputOptions,
	OutputOptions,
	NormalizedOutputOptions,
	Plugin,
	RolldownPlugin,
	RolldownPluginOption,
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
	HookFilterExtension,
	HookFilter,
	StringFilter,
	ModuleTypeFilter,
	PluginContext,
	TransformPluginContext,
	ObjectHook,
	PreRenderedChunk,
	PreRenderedAsset,
	SourceMap,
	SourceDescription,
	PartialNull,
	PartialResolvedId,
	ResolvedId,
	ModuleOptions,
	ModuleInfo,
	PluginContextMeta,
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
	RenderedModule,
	RenderedChunk,
	LogOrStringHandler,
	GetModuleInfo,
};

export type {
	RollupError,
	RollupLog,
	LoggingFunction,
	LogLevel,
	WarningHandlerWithDefault,
} from "./types/misc";
