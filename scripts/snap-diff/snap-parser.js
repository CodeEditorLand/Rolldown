import assert from "node:assert";
import { snakeCase } from "change-case";
import { trimStart } from "lodash-es";
import markdown from "markdown-it";

/**
 * @param {string} source
 *
 **/
export function parseEsbuildSnap(source) {
	let cases = source.split(
		"================================================================================",
	);
	return cases.map(parseEsbuildCase);
}

/**
 * @param {string} source
 * @returns {{name: string, sourceList: {name: string, content: string}[]}}
 * */
function parseEsbuildCase(source) {
	let lines = source.trimStart().split("\n");
	let [name, ...rest] = lines;
	let normalizedName = snakeCase(trimStart(name, "Test"));
	let content = rest.join("\n");
	return { name: normalizedName, sourceList: parseContent(content) };
}

/**
 * @param {string} content
 */
function parseContent(content) {
	// Define a regex pattern to match the filename and its content
	const regex =
		/----------\s*(.+?)\s*----------\s*([\s\S]*?)(?=----------|$)/g;

	const result = [];
	let match;

	// Use regex to find all matches in the input
	while ((match = regex.exec(content)) !== null) {
		const filename = match[1].trim(); // Extract the filename
		const content = match[2].trim(); // Extract the content

		// Push an object with filename and content into the result array
		result.push({
			name: filename,
			content: content,
		});
	}

	return result;
}

/**
 * @param {string | undefined} source
 *
 */
export function parseRolldownSnap(source) {
	if (!source) {
		return undefined;
	}
	let match;
	// strip `---source---` block
	while ((match = /---\n([\s\S]+?)\n---/.exec(source))) {
		source = source.slice(match.index + match[0].length);
	}
	// default mode
	const md = markdown();
	let tokens = md.parse(source);
	let i = 0;
	let ret = [];
	while (i < tokens.length) {
		let token = tokens[i];

		if (token.type === "heading_open" && token.tag === "h2") {
			let headingToken = tokens[i + 1];
			assert(headingToken.type === "inline");
			let filename = headingToken.content;
			let codeToken = tokens[i + 3];
			assert(codeToken.tag === "code");
			let content = codeToken.content;
			ret.push({ filename, content });
			i += 3;
		}
		i++;
	}
	return ret;
}
