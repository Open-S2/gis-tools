import { fromArrayBuffer } from './geotiff/src/geotiff';

const tiff = await fromArrayBuffer(await Bun.file('./tests/readers/geotiff/fixtures/initial.tiff').arrayBuffer());
const image = await tiff.getImage();
const data = await image.readRasters({ interleave: true });

console.log(data);

export function test(a: number): number {
  return a;
}
