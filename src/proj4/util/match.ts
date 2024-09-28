/**
 * @param obj - the object to search
 * @param key - the key to search with
 * @returns - the value of the key
 */
export default function match<T>(obj: Record<string, T>, key?: string): T | undefined {
  if (key === undefined) return;
  if (obj[key] !== undefined) return obj[key];
  const keys = Object.keys(obj);
  const ignoredChar = /[\s_\-/()]/g;
  const lkey = key.toLowerCase().replace(ignoredChar, '');
  let i = -1;
  let testkey, processedKey;
  while (++i < keys.length) {
    testkey = keys[i];
    processedKey = testkey.toLowerCase().replace(ignoredChar, '');
    if (processedKey === lkey) {
      return obj[testkey];
    }
  }
}
