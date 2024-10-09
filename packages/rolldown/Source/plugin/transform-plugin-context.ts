import { NormalizedInputOptions } from "..";
import type {
	BindingPluginContext,
	BindingTransformPluginContext,
} from "../binding";
import { normalizeLog } from "../log/logHandler";
import { augmentCodeLocation, error, logPluginError } from "../log/logs";
import type { LoggingFunctionWithPosition, RollupError } from "../rollup";
import type { Plugin } from "./index";
import { PluginContext } from "./plugin-context";
import { PluginContextData } from "./plugin-context-data";

export class TransformPluginContext extends PluginContext {
	error: (
		error: RollupError | string,
		pos?: number | { column: number; line: number },
	) => never;
	// getCombinedSourcemap: () => SourceMap

	constructor(
		options: NormalizedInputOptions,
		context: BindingPluginContext,
		plugin: Plugin,
		data: PluginContextData,
		inner: BindingTransformPluginContext,
		moduleId: string,
		moduleSource: string,
	) {
		super(options, context, plugin, data);
		const getLogHandler =
			(
				handler: LoggingFunctionWithPosition,
			): LoggingFunctionWithPosition =>
			(log, pos) => {
				log = normalizeLog(log);
				if (pos) augmentCodeLocation(log, pos, moduleSource, moduleId);
				log.id = moduleId;
				log.hook = "transform";
				handler(log);
			};

		this.debug = getLogHandler(this.debug);
		this.warn = getLogHandler(this.warn);
		this.info = getLogHandler(this.info);
		this.error = (
			e: RollupError | string,
			pos?: number | { column: number; line: number },
		): never => {
			if (typeof e === "string") e = { message: e };
			if (pos) augmentCodeLocation(e, pos, moduleSource, moduleId);
			e.id = moduleId;
			e.hook = "transform";
			return error(
				logPluginError(normalizeLog(e), plugin.name || "unknown"),
			);
		};
		// this.getCombinedSourcemap = () => JSON.parse(inner.getCombinedSourcemap())
	}
}
