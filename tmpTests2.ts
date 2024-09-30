// import { fromArrayBuffer } from './geotiff/src/geotiff';

// const tiff = await fromArrayBuffer(await Bun.file('./tests/readers/geotiff/fixtures/initial.tiff').arrayBuffer());
// const image = await tiff.getImage();
// const data = await image.readRasters({ interleave: true });

// console.log(data);

// export function test(a: number): number {
//   return a;
// }

// import { NadGrid } from './src/readers/nadgrid';
// import nadgrid from './proj4js-master/lib/nadgrid';
// import MMapReader from './src/readers/mmap';

const data = await Bun.file('./tests/proj4/fixtures/BETA2007.gsb').arrayBuffer();

// const gridA = nadgrid('BETA2007.gsb', data);
// const { ll: llA, del: delA, lim: limA, count: countA, cvs: cvsA } = gridA.subgrids[0];

// const gridB = new NadGrid('BETA2007.gsb', new MMapReader('./tests/proj4/fixtures/BETA2007.gsb'));
// const subGridB1 = gridB.subgrids[0];
// const { ll: llB, del: delB, lim: limB, count: countB, cvs: cvsB } = subGridB1;
// console.log('ll', llA, llB);
// console.log('del', delA, delB);
// console.log('lim', limA, limB);
// console.log('count', countA, countB);
// console.log('cvs', cvsA, cvsB);

import proj4 from './proj4js-master/lib/';

proj4.nadgrid('BETA2007.gsb', data);

// ['EPSG:31466', 'EPSG:25832', 2559552, 5670982, 349757.381712518, 5671004.065049540, 0.01, 0.01],

// proj4.defs('EPSG:31466', '+proj=tmerc +lat_0=0 +lon_0=6 +k=1 +x_0=2500000 +y_0=0 +ellps=bessel +nadgrids=BETA2007.gsb +units=m +no_defs +type=crs');
//         proj4.defs('EPSG:25832', '+proj=utm +zone=32 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs +type=crs');

proj4.defs('EPSG:31466', '+proj=tmerc +lat_0=0 +lon_0=6 +k=1 +x_0=2500000 +y_0=0 +ellps=bessel +nadgrids=BETA2007.gsb +units=m +no_defs +type=crs');
proj4.defs('EPSG:25832', '+proj=utm +zone=32 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs +type=crs')

const res = proj4('EPSG:31466', 'EPSG:25832', [2559552, 5670982]);
console.log('res', res)
