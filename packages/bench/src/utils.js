import nodePath from 'node:path'
import nodeUrl from 'node:url'

/**
 * @typedef {import('./types.js').BenchSuite} BenchSuite
 */

const dirname = nodePath.dirname(nodeUrl.fileURLToPath(import.meta.url))

export const REPO_ROOT = nodePath.join(dirname, '../../..')

export const PROJECT_ROOT = nodePath.join(dirname, '..')
