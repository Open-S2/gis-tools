import proj4 from './proj4js-master/lib/index';
// import proj4 from './proj4js-master/dist/proj4-src.js';
import { Transformer, injectAllDefinitions } from './src/proj4'

// proj4.defs([
//   ["EPSG:102018", "+proj=gnom +lat_0=90 +lon_0=0 +x_0=6300000 +y_0=6300000 +ellps=WGS84 +datum=WGS84 +units=m +no_defs"],
//   ["testmerc", "+proj=merc +lon_0=5.937 +lat_ts=45.027 +ellps=sphere"],
//   ["testmerc2", "+proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +units=m +k=1.0 +nadgrids=@null +no_defs"]
// ]);
// proj4.defs('esriOnline', 'PROJCS["WGS_1984_Web_Mercator_Auxiliary_Sphere",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Mercator_Auxiliary_Sphere"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",0.0],PARAMETER["Standard_Parallel_1",0.0],PARAMETER["Auxiliary_Sphere_Type",0.0],UNIT["Meter",1.0]]');
// proj4.defs('testings', 'GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]]')






// const PROJECTION_STRING = '+proj=gstmerc +lon_0=0 +x_0=0 +y_0=0 +R=6371008.7714 +datum=WGS84 +units=m +no_defs'

// proj4.defs('testings', PROJECTION_STRING)
// const testCase = {
//   code: 'testings',
//     ll: [-112.50042920000004, 42.036926809999976],
//     xy: [-5380950.902080743, -7434667.860047833],
// }
// // const proj = new proj4.Proj(testCase.code);
// // const rsltA = proj4.transform(proj4.WGS84, proj, testCase.ll);
// // const rsltA = proj4.transform(proj, proj4.WGS84, testCase.xy);
// // console.log('OLD: ', rsltA)

// console.log('\n\n')
// console.log('-----------------------------------------------------------------')
// console.log('\n\n')

// const transform = new Transformer();
// injectAllDefinitions(transform);
// transform.setSource(PROJECTION_STRING)
// const rsltB = transform.inverse({ x: testCase.ll[0], y: testCase.ll[1], z: testCase.ll[2] });
// // const rsltB = transform.forward({ x: testCase.xy[0], y: testCase.xy[1], z: testCase.xy[2] });
// console.log('NEW: ', rsltB)

// console.log('\n\n\n\n\n\n')

// console.log('OLD: ', rsltA)
// console.log('NEW: ', rsltB)

const enu = '+proj=longlat +axis=enu';
const esu = '+proj=longlat +axis=esu';
const wnu = '+proj=longlat +axis=wnu';
const transform = new Transformer(enu, esu);
injectAllDefinitions(transform);
// var result = proj4(enu, esu).forward({ x: 40, y: 50 }, true);
const result = transform.forward({ x: 40, y: 50 }, true);
console.log(result)

// {
//   projName: "cea",
//   datumCode: "WGS84",
//   units: "m",
//   no_defs: true,
//   type: "crs",
//   datum_params: [ "0", "0", "0" ],
//   ellps: "WGS84",
//   datumName: "WGS84",
//   k0: 0.8667510025721987,
//   axis: "enu",
//   init: [Function: init],
//   forward: [Function: forward],
//   inverse: [Function: inverse],
//   names: [ "cea" ],
//   a: 6378137,
//   b: 6356752.314245179,
//   rf: 298.257223563,
//   sphere: undefined,
//   es: 0.006694379990141316,
//   e: 0.08181919084262149,
//   ep2: 0.006739496742276434,
//   datum: {
//     datum_type: 4,
//     datum_params: [ 0, 0, 0 ],
//     a: 6378137,
//     b: 6356752.314245179,
//     es: 0.006694379990141316,
//     ep2: 0.006739496742276434,
//   },
// }

// // ------------------------------------

// {
//   name: "Equal_Area_Cylindrical",
//   projName: undefined,
//   datumCode: "WGS84",
//   datumType: 4,
//   datumParams: [ 0, 0, 0, 0, 0, 0, 0 ],
//   srsCode: "",
//   lon0: 0,
//   lon1: 0,
//   lon2: 0,
//   long0: 0,
//   long1: 0,
//   longc: 0,
//   lat0: 0,
//   lat1: 0,
//   lat2: 0,
//   latTs: 0.5235987755982988,
//   a: 6378137,
//   b: 6356752.314245179,
//   e: 0.08181919084262149,
//   x0: 0,
//   y0: 0,
//   k: undefined,
//   k0: 1,
//   rf: 298.257223563,
//   rA: false,
//   rc: undefined,
//   es: 0.006694379990141316,
//   ep2: 0.006739496742276434,
//   alpha: undefined,
//   gamma: undefined,
//   zone: undefined,
//   rectifiedGridAngle: undefined,
//   utmSouth: false,
//   toMeter: undefined,
//   units: "m",
//   fromGreenwich: 0,
//   approx: false,
//   axis: "enu",
//   nadgrids: "@null",
//   sphere: false,
//   ellps: "WGS84",
//   noDefs: true,
//   type: "crs",
//   forward: [Function: forward],
//   inverse: [Function: inverse],
// }






// const Rn = 6378137 / (Math.sqrt(1.0e0 - es * Sin2_Lat));
//   console.log('SUB A', a, Sin_Lat, Cos_Lat, Sin2_Lat, Rn)
//   // 6378137 0.8453038511165344 0.5342858778664806 0.7145386007124441 6393446.513163418

// es, f 0.006694380022900686 0.003352810681182268
// N NP K0  0.0016792203946287192 0.0000028197811337370315 0.9996 0.997924968703673
// es, f 0.006674372231802045 0.0033427731821747556
// N NP K0  0.0016741848011149636 0.0000028028947482843505 1 0.998329312961542

// es, f 0.006694380022900686 0.003352810681182268
// N NP K0  0.0016792203946287192 0.0000028197811337370315 1 0.9983242984230422

// es, f 0.006694380022900686
// N NP K0  1 0.9983242984230422

// console.log('\n\n\n\n\n\n\n');



// const transform = new Transformer();
// transform.setSource('+proj=merc +lon_0=5.937 +lat_ts=45.027 +ellps=sphere')

// const result = transform.forward({ x: -45007.0787624, y: 4151725.59875 })
// console.log('result', result)
// const backwards = transform.inverse(result)
// console.log('backwards', backwards)


// Benchmarking:

// create n number of points as both an array and vectorpoint

// const n = 50_000_000

// const vectorPoints = Array.from({ length: n }, (_, i) => {
//   // all the x-y values have to be between -180 and 180 and -80 and 80
//   const x = Math.random() * 360 - 180
//   const y = Math.min(Math.max(Math.random() * 180 - 90, -80), 80)
//   return { x, y }
// })
// const arrayPoints = vectorPoints.map((p) => [p.x, p.y])

// let start = Bun.nanoseconds()
// for (const point of arrayPoints) {
//   proj4.transform(proj4.WGS84, proj, proj4.toPoint(point));
// }
// let end = Bun.nanoseconds()
// let seconds = (end - start) / 1_000_000_000
// console.log('Proj4JS', seconds)

// start = Bun.nanoseconds()
// for (const point of vectorPoints) {
//   transform.inverse(point)
// }
// end = Bun.nanoseconds()
// seconds = (end - start) / 1_000_000_000
// console.log('Transformer', seconds)
