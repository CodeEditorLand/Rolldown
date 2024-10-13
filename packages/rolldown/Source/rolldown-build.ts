import { Bundler } from "./binding";
import type { InputOptions } from "./options/input-options";
import type { OutputOptions } from "./options/output-options";
import type { RolldownOutput } from "./types/rolldown-output";
import { createBundler } from "./utils/create-bundler";
import { transformToRollupOutput } from "./utils/transform-to-rollup-output";
import type { HasProperty, TypeAssert } from "./utils/type-assert";

export class RolldownBuild {
	#inputOptions: InputOptions;
	#bundler?: Bundler;
	#stopWorkers?: () => Promise<void>;

	constructor(inputOptions: InputOptions) {
		// TODO: Check if `inputOptions.output` is set. If so, throw an warning that it is ignored.
		this.#inputOptions = inputOptions;
	}

	async #getBundler(outputOptions: OutputOptions): Promise<Bundler> {
		if (typeof this.#bundler === "undefined") {
			const { bundler, stopWorkers } = await createBundler(
				this.#inputOptions,
				outputOptions,
			);
			this.#bundler = bundler;
			this.#stopWorkers = stopWorkers;
		}
		return this.#bundler;
	}

	async generate(outputOptions: OutputOptions = {}): Promise<RolldownOutput> {
		const bundler = await this.#getBundler(outputOptions);
		const output = await bundler.generate();
		return transformToRollupOutput(output);
	}

	async write(outputOptions: OutputOptions = {}): Promise<RolldownOutput> {
		const bundler = await this.#getBundler(outputOptions);
		const output = await bundler.write();
		return transformToRollupOutput(output);
	}

	async close(): Promise<void> {
		const bundler = await this.#getBundler({});
		await this.#stopWorkers?.();
		await bundler.close();
	}

	async watch(): Promise<void> {
		const bundler = await this.#getBundler({});
		await bundler.watch();
	}
}

function _assert() {
	type _ = TypeAssert<HasProperty<RolldownBuild, "generate" | "write">>;
}