import { join } from 'path';
import { randomBytes } from 'crypto';
import { tmpdir } from 'os';
import { mkdirSync, rmSync } from 'fs';

/**
 * Creates a unique path string starting with the system temp directory.
 * Note: This only creates the string, not the actual folder on disk.
 *
 * @param name - The name of the folder
 * @returns A unique path string
 */
export function createTempPath(name: string): string {
  // Generate a random 6-character suffix (e.g., 'a1b2c3')
  const suffix = randomBytes(9).toString('hex');
  // Joins: /tmp (or C:\Users\...\Temp) + name + suffix
  const tmp_folder = join(tmpdir(), `${name}_${suffix}`);
  // creates the folder on disk
  mkdirSync(tmp_folder);
  return tmp_folder;
}

/**
 * Deletes a file or directory at the given path.
 *
 * @param path - The path of the file or directory to delete
 */
export function deletePath(path: string): void {
  try {
    // recursive: true handles directories with files inside
    // force: true ignores the error if the path doesn't exist
    rmSync(path, { recursive: true, force: true });
  } catch (err) {
    console.error(`Failed to delete path: ${path}`, err);
  }
}
