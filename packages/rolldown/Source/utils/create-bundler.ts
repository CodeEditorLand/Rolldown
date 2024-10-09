import { Bundler } from "../binding";
import { bindingifyInputOptions } from "../options/bindingify-input-options";
import { bindingifyOutputOptions } from "../options/bindingify-output-options";
import type { InputOptions } from "../options/input-options";
import type { OutputOptions } from "../options/output-options";
import { PluginDriver } from "../plugin/plugin-driver";
import { initializeParallelPlugins } from "./initialize-parallel-plugins";
import { normalizeInputOptions } from "./normalize-input-options";
import { normalizeOutputOptions } from "./normalize-output-options";

export async function createBundler(
	inputOptions: InputOptions,
	outputOptions: OutputOptions,
): Promise<{
	bundler: Bundler;
	stopWorkers?: () => Promise<void>;
}> {
	const pluginDriver = new PluginDriver();
	inputOptions = await pluginDriver.callOptionsHook(inputOptions);
	// Convert `InputOptions` to `NormalizedInputOptions`.
	const normalizedInputOptions = await normalizeInputOptions(inputOptions);

	const parallelPluginInitResult = await initializeParallelPlugins(
		normalizedInputOptions.plugins,
	);

	try {
		outputOptions = pluginDriver.callOutputOptionsHook(
			normalizedInputOptions,
			outputOptions,
		);
		const normalizedOutputOptions = normalizeOutputOptions(outputOptions);

		// Convert `NormalizedInputOptions` to `BindingInputOptions`
		const bindingInputOptions = bindingifyInputOptions(
			normalizedInputOptions,
			normalizedOutputOptions,
		);

		return {
			bundler: new Bundler(
				bindingInputOptions,
				bindingifyOutputOptions(normalizedOutputOptions),
				parallelPluginInitResult?.registry,
			),
			stopWorkers: parallelPluginInitResult?.stopWorkers,
		};
	} catch (e) {
		await parallelPluginInitResult?.stopWorkers();
		throw e;
	}
}
