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

// // const code = `PROJCRS["WGS84 / Pseudo-Mercator",
// //    BASEGEOGCRS["WGS 84",
// //        ENSEMBLE["World Geodetic System 1984 ensemble",
// //            MEMBER["World Geodetic System 1984 (Transit)", ID["EPSG",1166]],
// //            MEMBER["World Geodetic System 1984 (G730)", ID["EPSG",1152]],
// //            MEMBER["World Geodetic System 1984 (G873)", ID["EPSG",1153]],
// //            MEMBER["World Geodetic System 1984 (G1150)", ID["EPSG",1154]],
// //            MEMBER["World Geodetic System 1984 (G1674)", ID["EPSG",1155]],
// //            MEMBER["World Geodetic System 1984 (G1762)", ID["EPSG",1156]],
// //            MEMBER["World Geodetic System 1984 (G2139)", ID["EPSG",1309]],
// //            MEMBER["World Geodetic System 1984 (G2296)", ID["EPSG",1383]],
// //            ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7030]],
// //            ENSEMBLEACCURACY[2],
// //            ID["EPSG",6326]],
// //       ID["EPSG",4326]],
// //    CONVERSION["Popular Visualisation Pseudo-Mercator",
// //        METHOD["Popular Visualisation Pseudo Mercator",ID["EPSG",1024]],
// //        PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],
// //        PARAMETER["Longitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],
// //        PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],
// //        PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],
// //        ID["EPSG",3856]],
// //    CS[Cartesian,2,ID["EPSG",4499]],
// //    AXIS["Easting (X)",east],
// //    AXIS["Northing (Y)",north],
// //    LENGTHUNIT["metre",1,ID["EPSG",9001]],
// //    ID["EPSG",3857]]`;
// // const code = "+proj=utm +zone=15 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs +type=crs";
// const code = "PROJCRS[\"NAD83 / UTM zone 15N\",BASEGEOGCRS[\"NAD83\",DATUM[\"North American Datum 1983\",ELLIPSOID[\"GRS 1980\",6378137,298.257222101,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",7019]],ID[\"EPSG\",6269]],ID[\"EPSG\",4269]],CONVERSION[\"UTM zone 15N\",METHOD[\"Transverse Mercator\",ID[\"EPSG\",9807]],PARAMETER[\"Latitude of natural origin\",0,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8801]],PARAMETER[\"Longitude of natural origin\",-93,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8802]],PARAMETER[\"Scale factor at natural origin\",0.9996,SCALEUNIT[\"unity\",1,ID[\"EPSG\",9201]],ID[\"EPSG\",8805]],PARAMETER[\"False easting\",500000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8806]],PARAMETER[\"False northing\",0,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],ID[\"EPSG\",16015]],CS[Cartesian,2,ID[\"EPSG\",4400]],AXIS[\"Easting (E)\",east],AXIS[\"Northing (N)\",north],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",26915]]"

// let transformer = new Transformer();
// const wktParsed = parseProj(code, transformer);

// //   x: 470692.44,
// //   y: 4602888.9,
// console.log(wktParsed)
// // transformer.setSource(wktParsed);
// transformer = new Transformer(code, undefined, [])
// let res = transformer.forward({ x: 470692.44, y: 4602888.9 });
// console.log(res)

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




// import { Transformer, WebMercator } from './src';

// const transformer = new Transformer();
// transformer.insertDefinition(WebMercator);
// transformer.setDestination(`PROJCRS["WGS 84 / Pseudo-Mercator",
//             BASEGEOGCRS["WGS 84",
//                 ENSEMBLE["World Geodetic System 1984 ensemble",
//                     MEMBER["World Geodetic System 1984 (Transit)", ID["EPSG",1166]],
//                     MEMBER["World Geodetic System 1984 (G730)", ID["EPSG",1152]],
//                     MEMBER["World Geodetic System 1984 (G873)", ID["EPSG",1153]],
//                     MEMBER["World Geodetic System 1984 (G1150)", ID["EPSG",1154]],
//                     MEMBER["World Geodetic System 1984 (G1674)", ID["EPSG",1155]],
//                     MEMBER["World Geodetic System 1984 (G1762)", ID["EPSG",1156]],
//                     MEMBER["World Geodetic System 1984 (G2139)", ID["EPSG",1309]],
//                     MEMBER["World Geodetic System 1984 (G2296)", ID["EPSG",1383]],
//                     ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7030]],
//                     ENSEMBLEACCURACY[2],
//                     ID["EPSG",6326]],
//                 ID["EPSG",4326]],
//             CONVERSION["Popular Visualisation Pseudo-Mercator",
//                 METHOD["Popular Visualisation Pseudo Mercator",ID["EPSG",1024]],
//                 PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],
//                 PARAMETER["Longitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],
//                 PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],
//                 PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],
//                 ID["EPSG",3856]],
//             CS[Cartesian,2,ID["EPSG",4499]],
//             AXIS["Easting (X)",east],
//             AXIS["Northing (Y)",north],
//             LENGTHUNIT["metre",1,ID["EPSG",9001]],
//             ID["EPSG",3857]]`);

// console.log('transformer', transformer)
// let fwd = transformer.forward({ x: 1, y: 1 });
// console.log('fwd', fwd)







// import { decodeJpegData } from './src';

// /**
//  * @param name - the name of the fixture
//  * @returns the contents of the fixture as an array buffer
//  */
// async function fixture(name: string): Promise<ArrayBufferLike> {
//   return await Bun.file(`tests/readers/image/jpeg/fixtures/${name}`).arrayBuffer();
// }

// const jpegData = await fixture('grumpycat.jpg');
// const _rawImageData = decodeJpegData(jpegData);
// console.log(_rawImageData.data.slice(0, 20))









// import { GeoTIFFReader, ExtendedTransverseMercator } from './src';
// import { FileReader } from './src/file';

// const geotiffReader = new GeoTIFFReader(new FileReader('./tests/readers/geotiff/fixtures/projections/transverse_mercator_south_oriented.tif'), [ExtendedTransverseMercator], {});
// const image = geotiffReader.getImage();
// const bbox = image.getBoundingBox();
// console.log('bbox', bbox)
// // const _raster = await image.rasterData();
// // const _rgb = await image.getRGBA();
// // const value_2_0 = await image.getValue(2, 0);
// // console.log(value_2_0[0])




// // import { LASReader, EPSG_26915, UniversalTransverseMercator } from './src';
// // import { FileReader } from './src/file';

// // // tests/readers/las/fixtures/1.2_0.las
// // const las = new LASReader(new FileReader('./tests/readers/las/fixtures/1.2_0.las'), [UniversalTransverseMercator], { EPSG_26915 });
// // const features = await Array.fromAsync(las);

// // BEFORE:
// //   x: 470692.44,
// //   y: 4602888.9,

// // AFTER:
// // x: -93.35156259019989,
// // y: 41.577148395419115








// import GtfsRealtimeBindings from 'gtfs-realtime-bindings';
// import * as fs from 'fs';

// const message: GtfsRealtimeBindings.transit_realtime.IFeedMessage = {
//   header: {
//     gtfsRealtimeVersion: '2.0',
//     incrementality: 1, // Incrementality (0-1)
//     timestamp: Math.floor(Date.now() / 1000),
//   },
//   entity: [{
//     id: 'vehicle-1',
//     isDeleted: false,
//     tripUpdate: { // ITripUpdate
//         trip:  { // ITripDescriptor
//           tripId: "trip-002",
//           routeId: "route-A",
//           directionId: 2,
//           startTime: "10:00:00",
//           startDate: "2020-01-01",
//           scheduleRelationship: 6 // ScheduleRelationship (0-6)
//         },
//         vehicle: { // IVehicleDescriptor
//           id: "vehicle-001",
//           label: "vlabel-1",
//           licensePlate: "license-plate-1",
//         },
//         stopTimeUpdate: [{ // IStopTimeUpdate
//             stopSequence: 1,
//             stopId: "stop-001",
//             arrival: { // IStopTimeEvent
//               delay: 20,
//               time: Math.floor(Date.now() / 1000),
//               uncertainty: 0,
//             },
//             departure: { // IStopTimeEvent
//               delay: 100,
//               time: Math.floor(Date.now() / 1000),
//               uncertainty: 3,
//             },
//             scheduleRelationship: 3, // ScheduleRelationship UNIQUE (0-3)
//             stopTimeProperties: { // IStopTimeProperties
//               assignedStopId: "stop-001",
//             }
//         }],
//         timestamp: Math.floor(Date.now() / 1000),
//         delay: 20,
//         tripProperties: { // ITripProperties
//             tripId: "trip-002",
//             startDate: "2020-01-01",
//             startTime: "10:00:00",
//         },
//     },
//     vehicle: { // IVehiclePosition
//       trip: { // ITripDescriptor
//         tripId: 'trip-001',
//         routeId: 'route-A',
//         directionId: 22,
//         startTime: "10:00:00",
//         startDate: "2020-01-01",
//         scheduleRelationship: 6, // ScheduleRelationship (0-6)
//       },
//       vehicle: { // IVehicleDescriptor
//         id: "id-22",
//         label: "label-22",
//         licensePlate: "license-plate-22",
//       },
//       position: { // IPosition
//         latitude: 40.0,
//         longitude: -111.9,
//         bearing: 90.0,
//         odometer: 22.2,
//         speed: 33.2,
//       },
//       currentStopSequence: 1,
//       stopId: 'stop-001',
//       currentStatus: 2, // VehicleStopStatus (0-2)
//       timestamp: Math.floor(Date.now() / 1000),
//         congestionLevel: 4, // CongestionLevel (0-4)
//         occupancyStatus: 8, // OccupancyStatus (0-8)
//         occupancyPercentage: 10,
//         multiCarriageDetails: [{ // ICarriageDetails
//             id: "id-33",
//             label: "label-33",
//             occupancyStatus: 8, // OccupancyStatus (0-8)
//             occupancyPercentage: 33,
//             carriageSequence: 33,
//         }],
//     },
//     alert: { // IAlert
//         activePeriod: [{ // ITimeRange
//           start: Math.floor(Date.now() / 1000),
//           end: Math.floor(Date.now() / 1000),
//         }],
//         informedEntity: [{ // IEntitySelector
//           agencyId: "agency-1",
//           routeId: "route-1",
//           routeType: 1,
//           trip: { // ITripDescriptor
//             tripId: "trip-1",
//             routeId: "route-1",
//             directionId: 3,
//             startTime: "00:00:00",
//             startDate: "2020-01-01",
//             scheduleRelationship: 6, // ScheduleRelationship (0-6)
//           },
//           stopId: "stop-1",
//           directionId: 3,
//         }],
//         cause: 12, // Cause (1-12)
//         effect: 11, // Effect (1-11)
//         url: { // ITranslatedString
//           translation: [{
//             text: "text-1",
//             language: "EN",
//           }]
//         },
//         headerText: { // ITranslatedString
//           translation: [{
//             text: "text0",
//             language: "LA",
//           }]
//         },
//         descriptionText: { // ITranslatedString
//           translation: [{
//             text: "text1",
//             language: "BE",
//           }]
//         },
//         ttsHeaderText: { // ITranslatedString
//           translation: [{
//             text: "text2",
//             language: "UR",
//           }]
//         },
//         ttsDescriptionText: { // ITranslatedString
//           translation: [{
//             text: "text3",
//             language: "RA",
//           }]
//         },

//         /** Alert severityLevel */
//         severityLevel: 4, // SeverityLevel (1-4)
//     },
//   }],
// };

// const buffer = GtfsRealtimeBindings.transit_realtime.FeedMessage.encode(message).finish();
// fs.writeFileSync('./realtime_test_data_11.pb', buffer);



// const bytes = [117, 114, 108, 32];
// const buffer = Buffer.from(bytes);
// const value = buffer.readUInt32BE(0);
// console.log(value); // 1780692640


// import { BoxIndex } from './src';
// import Flatbush from 'flatbush'
// import { expect } from 'bun:test';

// import type { BoxIndexAccessor } from './src';

// const data = [
//   8, 62, 11, 66, 57, 17, 57, 19, 76, 26, 79, 29, 36, 56, 38, 56, 92, 77, 96, 80, 87, 70, 90, 74, 43,
//   41, 47, 43, 0, 58, 2, 62, 76, 86, 80, 89, 27, 13, 27, 15, 71, 63, 75, 67, 25, 2, 27, 2, 87, 6, 88,
//   6, 22, 90, 23, 93, 22, 89, 22, 93, 57, 11, 61, 13, 61, 55, 63, 56, 17, 85, 21, 87, 33, 43, 37, 43,
//   6, 1, 7, 3, 80, 87, 80, 87, 23, 50, 26, 52, 58, 89, 58, 89, 12, 30, 15, 34, 32, 58, 36, 61, 41,
//   84, 44, 87, 44, 18, 44, 19, 13, 63, 15, 67, 52, 70, 54, 74, 57, 59, 58, 59, 17, 90, 20, 92, 48,
//   53, 52, 56, 92, 68, 92, 72, 26, 52, 30, 52, 56, 23, 57, 26, 88, 48, 88, 48, 66, 13, 67, 15, 7, 82,
//   8, 86, 46, 68, 50, 68, 37, 33, 38, 36, 6, 15, 8, 18, 85, 36, 89, 38, 82, 45, 84, 48, 12, 2, 16, 3,
//   26, 15, 26, 16, 55, 23, 59, 26, 76, 37, 79, 39, 86, 74, 90, 77, 16, 75, 18, 78, 44, 18, 45, 21,
//   52, 67, 54, 71, 59, 78, 62, 78, 24, 5, 24, 8, 64, 80, 64, 83, 66, 55, 70, 55, 0, 17, 2, 19, 15,
//   71, 18, 74, 87, 57, 87, 59, 6, 34, 7, 37, 34, 30, 37, 32, 51, 19, 53, 19, 72, 51, 73, 55, 29, 45,
//   30, 45, 94, 94, 96, 95, 7, 22, 11, 24, 86, 45, 87, 48, 33, 62, 34, 65, 18, 10, 21, 14, 64, 66, 67,
//   67, 64, 25, 65, 28, 27, 4, 31, 6, 84, 4, 85, 5, 48, 80, 50, 81, 1, 61, 3, 61, 71, 89, 74, 92, 40,
//   42, 43, 43, 27, 64, 28, 66, 46, 26, 50, 26, 53, 83, 57, 87, 14, 75, 15, 79, 31, 45, 34, 45, 89,
//   84, 92, 88, 84, 51, 85, 53, 67, 87, 67, 89, 39, 26, 43, 27, 47, 61, 47, 63, 23, 49, 25, 53, 12, 3,
//   14, 5, 16, 50, 19, 53, 63, 80, 64, 84, 22, 63, 22, 64, 26, 66, 29, 66, 2, 15, 3, 15, 74, 77, 77,
//   79, 64, 11, 68, 11, 38, 4, 39, 8, 83, 73, 87, 77, 85, 52, 89, 56, 74, 60, 76, 63, 62, 66, 65, 67,
// ];

// /** Test item */
// interface Item {
//   id: number;
//   minX: number;
//   minY: number;
//   maxX: number;
//   maxY: number;
// }

// /**
//  * A function for accessing the minX, minY, maxX, and maxY properties of the items.
//  * @param item - The item to access.
//  * @returns An array of the minX, minY, maxX, and maxY properties of the item.
//  */
// const accessor: BoxIndexAccessor<Item> = (item: Item) => [
//   item.minX,
//   item.minY,
//   item.maxX,
//   item.maxY,
// ];

// const items: Item[] = [];

//   for (let i = 0; i < data.length; i += 4) {
//     items.push({
//       id: i,
//       minX: data[i],
//       minY: data[i + 1],
//       maxX: data[i + 2],
//       maxY: data[i + 3],
//     });
//   }

// /**
//  * @returns A new BoxIndex instance using data
//  */
// function createIndexMine(): BoxIndex<Item> {
//   return new BoxIndex<Item>(items, accessor);
// }


// function createIndexFlatbush(): Flatbush {
//   let index = new Flatbush(items.length);

//   // fill it with 1000 rectangles
//   for (const p of items) {
//     index.add(p.minX, p.minY, p.maxX, p.maxY);
//   }

//   // perform the indexing
//   index.finish();

//   return index;
// }

// let indexMine = createIndexMine();
// let indexFlatbush = createIndexFlatbush();

// // console.log('indexMine', indexMine);
// // console.log('indexFlatbush', indexFlatbush);

// let indicesMine = indexMine._indices;
// let indicesFlatbush = [...indexFlatbush._indices];

// expect(indicesMine).toEqual(indicesFlatbush);


// const itemsMine = indexMine.neighbors(50, 50, Infinity, 12);
// const itemsFlatbush = indexFlatbush.neighbors(50, 50, Infinity, 12).map(itemIndex => items[itemIndex]);

// expect(itemsMine.sort((itemA, itemB) => itemA.id - itemB.id)).toEqual(itemsFlatbush.sort((itemA, itemB) => itemA.id - itemB.id));

// const itemsSearchMine = indexMine.search(40, 40, 60, 60);
// const itemsSearchFlatbush = indexFlatbush.search(40, 40, 60, 60).map(itemIndex => items[itemIndex]);

// expect(itemsSearchMine.sort((itemA, itemB) => itemA.id - itemB.id)).toEqual(itemsSearchFlatbush.sort((itemA, itemB) => itemA.id - itemB.id));



// import { cleanPolygons, convert, polygonsUnion, vectorToFlat } from './src/index.js';

// import type { FeatureCollection, Properties, VectorMultiPolygon, VectorMultiPolygonGeometry } from 's2json-spec';

// const featureCollection = (await Bun.file(
//   `${__dirname}/tests/geometry/tools/fixtures/chunks-water.json`,
// ).json()) as FeatureCollection;
// const vectorFeatures = convert('WG', featureCollection);

// const vectorPolys: VectorMultiPolygon = [];

// for (const feature of vectorFeatures) {
//   const { type, coordinates } = feature.geometry;
//   if (type === 'Polygon') {
//     vectorPolys.push(coordinates);
//   } else if (type === 'MultiPolygon') {
//     vectorPolys.push(...coordinates);
//   }
// }
// const cleanedPolys = cleanPolygons(vectorPolys, true)!;
// const geo: VectorMultiPolygonGeometry<Properties> = { type: 'MultiPolygon', coordinates: cleanedPolys, is3D: false }
// const geometry = vectorToFlat(geo);
// const feature = { type: 'Feature', properties: {}, geometry };
// const res = { type: 'FeatureCollection', features: [feature] };

// await Bun.write(
//   `${__dirname}/tests/geometry/tools/fixtures/chunks-water-cleaned.json`,
//   JSON.stringify(res),
// );

import { orient2dVector } from './src/index.js';

import type { VectorPoint } from "s2json-spec";

// const startA: VectorPoint = { x: 54.57114771720956, y: 24.441853864959285 };
// const startB: VectorPoint = { x: 54.57204465994316, y: 24.442325298422087 };
const int: VectorPoint = { x: 54.571070122708974, y: 24.441813081258456 };
// const c: VectorPoint = { x: 54.57080496898934, y: 24.4416737163564 };

// const firstRes = -orient2dVector(startA, c, int); // 6.760034294416112e-19
// const secondRes = -orient2dVector(startB, c, int); // 5.519081395851809e-18

// const singleRes = -orient2dVector(startA, c, startB);

// console.log('firstRes', firstRes);
// console.log('secondRes', secondRes);
// console.log('singleRes', singleRes);

// PAIR 0 0 {
//   x: 54.57114771720956,
//   y: 24.441853864959285,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.4416737163564,
// } 3.1415926535640484 0 0

// PAIR 1 1 {
//   x: 54.57204465994316,
//   y: 24.442325298422087,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.4416737163564,
// } 3.1415926535730576 0 0


// INTERSECTION 0 0 6 7 0.22638921520691385 [ 54.571070122708974, 24.441813081258456 ]
// [54.57114771720956, 24.441853864959285],
// [54.57080496898934, 24.4416737163564],
const BStart: VectorPoint = { x: 54.57114771720956, y: 24.441853864959285 };
const BEnd: VectorPoint = { x: 54.57080496898934, y: 24.4416737163564 };

// INTERSECTION 1 1 3 4 0.4766257516779802 [ 54.571070122708974, 24.441813081258456 ]
// [54.57204465994316, 24.442325298422087],
// [54.57000000000001, 24.441250624330703],
const AStart: VectorPoint = { x: 54.57204465994316, y: 24.442325298422087 };
const AEnd: VectorPoint = { x: 54.57000000000001, y: 24.441250624330703 };

const secondIntersection: VectorPoint = {
  x: 54.57080496898934,
  y: 24.44167371635639,
};


const firstRes = -orient2dVector(AStart, int, BStart);
console.log('firstRes', firstRes);

// TODO {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } 3.141592653574547 3.1415926535640484
// [ 54.57114771720956, 24.441853864959285 ] [ 54.57114771720956, 24.441853864959285 ] [ 54.57080496898934, 24.441673716356398 ] [ 54.57080496898934, 24.4416737163564 ] false 9.42015246719407e-19 

// TODO {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } 3.1415926535730576 3.141592653574547
// [ 54.57204465994316, 24.442325298422087 ] [ 54.57114771720956, 24.441853864959285 ] [ 54.57080496898934, 24.4416737163564 ] [ 54.57080496898934, 24.441673716356398 ] false 8.69459468021838e-19 

// TODO {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } 3.1415926535835563 3.141592653574547
// [ 54.57204465994316, 24.442325298422087 ] [ 54.57114771720956, 24.441853864959285 ] [ 54.57080496898934, 24.441673716356398 ] [ 54.57080496898934, 24.441673716356398 ] true 8.69459468021838e-19 

// TODO {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } 3.1415926535835563 3.1415926535730576
// [ 54.57204465994316, 24.442325298422087 ] [ 54.57204465994316, 24.442325298422087 ] [ 54.57080496898934, 24.441673716356398 ] [ 54.57080496898934, 24.4416737163564 ] false 9.42015246719407e-19 

// CONNECT! [ 54.569778932416476, 24.441366817541834 ] [
//   [ 54.56977894449294, 24.441074136738756 ], [ 54.57000000000001, 24.441190327160086 ], [ 54.57084694057397, 24.440745161222193 ], [ 54.57084693745136, 24.44028034081218 ], [ 54.571147760242575, 24.44043845608456 ], [ 54.57114771720956, 24.441853864959285 ]
// ] [ 54.571070122708974, 24.441813081258456 ] [
//   [ 54.57080496898934, 24.4416737163564 ]
// ] [ 54.57080496898934, 24.441673716356398 ]


// CONNECT! [ 54.56795530519258, 24.44447467409078 ] [
//   [ 54.57000000000001, 24.4455493756693 ], [ 54.57204469480743, 24.44447467409078 ], [ 54.57204465994316, 24.442325298422087 ]
// ] [ 54.571070122708974, 24.441813081258456 ] [] [ 54.57080496898934, 24.441673716356398 ]