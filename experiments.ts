// // import { UniversalTransverseMercator, EPSG_32644, GeoTIFFReader } from './src';
// // import { FileReader } from './src/file';

// // const fileReader = new FileReader('/Users/craigoconnor/Downloads/RUSLE_3Mar.tif');
// // const geotiffReader = new GeoTIFFReader(fileReader, [UniversalTransverseMercator], { EPSG_32644 });

// // console.log(geotiffReader)

// // const data = geotiffReader.getImage();
// // console.log(data)

// // const bbox = data.getBoundingBox();
// // console.log(bbox)
// // // 78.0187878659497, 25.000466298000198, 84.93569253742118, 30.35673826820515
// // // 199_080, 2_768_310, 878_370, 3_364_890

// // const rasterData = await data.rasterData();
// // console.log(rasterData)

// // // EPSG_32644: +proj=utm +zone=44 +datum=WGS84 +units=m +no_defs +type=crs
// // // const point = await data.getValue(286_456, 3_355_034);
// // // const point = await data.getValue(79.0, 30.2);
// // // console.log(point)

// import { parseProj, Transformer } from './src';

// const code = `PROJCRS["WGS84 / Pseudo-Mercator",
//    BASEGEOGCRS["WGS 84",
//        ENSEMBLE["World Geodetic System 1984 ensemble",
//            MEMBER["World Geodetic System 1984 (Transit)", ID["EPSG",1166]],
//            MEMBER["World Geodetic System 1984 (G730)", ID["EPSG",1152]],
//            MEMBER["World Geodetic System 1984 (G873)", ID["EPSG",1153]],
//            MEMBER["World Geodetic System 1984 (G1150)", ID["EPSG",1154]],
//            MEMBER["World Geodetic System 1984 (G1674)", ID["EPSG",1155]],
//            MEMBER["World Geodetic System 1984 (G1762)", ID["EPSG",1156]],
//            MEMBER["World Geodetic System 1984 (G2139)", ID["EPSG",1309]],
//            MEMBER["World Geodetic System 1984 (G2296)", ID["EPSG",1383]],
//            ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7030]],
//            ENSEMBLEACCURACY[2],
//            ID["EPSG",6326]],
//       ID["EPSG",4326]],
//    CONVERSION["Popular Visualisation Pseudo-Mercator",
//        METHOD["Popular Visualisation Pseudo Mercator",ID["EPSG",1024]],
//        PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],
//        PARAMETER["Longitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],
//        PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],
//        PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],
//        ID["EPSG",3856]],
//    CS[Cartesian,2,ID["EPSG",4499]],
//    AXIS["Easting (X)",east],
//    AXIS["Northing (Y)",north],
//    LENGTHUNIT["metre",1,ID["EPSG",9001]],
//    ID["EPSG",3857]]`;

// const transformer = new Transformer();
// const wktParsed = parseProj(code, transformer);

// console.log(wktParsed)

// // {
// //   type: "PROJCRS",
// //   srsCode: "WGS84 / Pseudo-Mercator",
// //   name: "WGS84 / Pseudo-Mercator",
// //   a: 6378137,
// //   b: 6356752.314245179,
// //   rf: 298.257223563,
// //   es: 0.006694379990141316,
// //   e: 0.08181919084262149,
// //   ep2: 0.006739496742276434,
// //   datumType: 5,
// // }

// import { LASZipReader } from './src';
// import { FileReader } from './src/file';

// let laz = new LASZipReader(
//     new FileReader(`${__dirname}/tests/readers/las/fixtures/simpleV3.laz`));

// let features = await Array.fromAsync(laz);
// console.log(features[0].geometry.coordinates.m)
// console.log(features[1].geometry.coordinates.m)
// console.log(features[2].geometry.coordinates.m)
// console.log(features[features.length - 1].geometry.coordinates)
// // console.log(features.length)

// // console.log('laz', laz)

// import { LASZipReader, idFromLonLat, LambertConformalConic } from './src';
// import { FileReader } from './src/file';

// import type { Properties } from './src';

// // const reader = new NewLineDelimitedJSONReader(new FileReader(`${__dirname}/tests/readers/las/fixtures/autzen_trim.jsonld`));
// const reader = new LASZipReader(
//     new FileReader(`${__dirname}/tests/readers/las/fixtures/autzen_trim.laz`),
//     [LambertConformalConic],
// );
// const features = await Array.fromAsync(reader);
// console.log(features.length);
// 110_000

// let points: { id: bigint, x: number, y: number, z: number, m: Properties }[] = [];

// for await (const feature of reader) {
//     if (feature.geometry.type !== 'Point') continue;
//     const id = idFromLonLat(feature.geometry.coordinates);
//     points.push({
//         id,
//         x: feature.geometry.coordinates.x,
//         y: feature.geometry.coordinates.y,
//         z: feature.geometry.coordinates.z ?? 0,
//         m: feature.geometry.coordinates.m as Properties
//     });
// }
// points = points.sort((a, b) => Number(a.id - b.id));

// console.log(points.slice(0, 10))

// let nums: bigint[] = [];
// let diff = 0n;
// for (const point of points) {
//     nums.push(point.id - diff);
//     diff = point.id;
// }

// console.log(nums.slice(0, 10))

import { Transformer, WebMercator } from './src';

const transformer = new Transformer();
transformer.insertDefinition(WebMercator);
transformer.setDestination(`PROJCRS["WGS 84 / Pseudo-Mercator",
            BASEGEOGCRS["WGS 84",
                ENSEMBLE["World Geodetic System 1984 ensemble",
                    MEMBER["World Geodetic System 1984 (Transit)", ID["EPSG",1166]],
                    MEMBER["World Geodetic System 1984 (G730)", ID["EPSG",1152]],
                    MEMBER["World Geodetic System 1984 (G873)", ID["EPSG",1153]],
                    MEMBER["World Geodetic System 1984 (G1150)", ID["EPSG",1154]],
                    MEMBER["World Geodetic System 1984 (G1674)", ID["EPSG",1155]],
                    MEMBER["World Geodetic System 1984 (G1762)", ID["EPSG",1156]],
                    MEMBER["World Geodetic System 1984 (G2139)", ID["EPSG",1309]],
                    MEMBER["World Geodetic System 1984 (G2296)", ID["EPSG",1383]],
                    ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7030]],
                    ENSEMBLEACCURACY[2],
                    ID["EPSG",6326]],
                ID["EPSG",4326]],
            CONVERSION["Popular Visualisation Pseudo-Mercator",
                METHOD["Popular Visualisation Pseudo Mercator",ID["EPSG",1024]],
                PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],
                PARAMETER["Longitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],
                PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],
                PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],
                ID["EPSG",3856]],
            CS[Cartesian,2,ID["EPSG",4499]],
            AXIS["Easting (X)",east],
            AXIS["Northing (Y)",north],
            LENGTHUNIT["metre",1,ID["EPSG",9001]],
            ID["EPSG",3857]]`);

console.log('transformer', transformer)
let fwd = transformer.forward({ x: 1, y: 1 });
console.log('fwd', fwd)
