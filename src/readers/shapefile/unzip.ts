import { iter } from 'but-unzip';

const regex = /.+\.(shp|dbf|json|prj|cpg)$/i;
/**
 * @param buffer
 */
export default async (buffer) => {
  const files = {};
  const proms = [];
  for (const entry of iter(buffer)) {
    if (!regex.test(entry.filename)) {
      continue;
    }
    proms.push(Promise.resolve(entry.read()).then((bytes) => (files[entry.filename] = bytes)));
  }
  await Promise.all(proms);
  const out = {};
  const decoder = new TextDecoder();
  for (const [key, value] of Object.entries(files)) {
    if (key.slice(-3).toLowerCase() === 'shp' || key.slice(-3).toLowerCase() === 'dbf') {
      out[key] = new DataView(value.buffer, value.byteOffset, value.byteLength);
    } else {
      out[key] = decoder.decode(value);
    }
  }
  return out;
};
