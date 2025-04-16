export * from './geometry.js';
export * from './object.js';
export * from './projection.js';

export type * from './projection.js';

/**
 * Clean a string to remove whitespace, single and double quotes, and replace multiple spaces with a single space
 * @param str - string to clean
 * @returns - cleaned string
 */
export function cleanString(str: string): string {
  return str
    .trim() // Remove whitespace at the start and end
    .replace(/^['"]|['"]$/g, '') // Remove single or double quotes from start and end
    .replace(/\s+/g, ' '); // Replace multiple spaces with a single space
}
