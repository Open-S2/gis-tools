import proj4 from './proj4js-master/lib/index.js';
import { Transformer } from './src/proj4/index.js'

proj4.defs([
  // ["EPSG:102018", "+proj=gnom +lat_0=90 +lon_0=0 +x_0=6300000 +y_0=6300000 +ellps=WGS84 +datum=WGS84 +units=m +no_defs"],
  ["testmerc", "+proj=merc +lon_0=5.937 +lat_ts=45.027 +ellps=sphere"],
  // ["testmerc2", "+proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +units=m +k=1.0 +nadgrids=@null +no_defs"]
]);
// proj4.defs('esriOnline', 'PROJCS["WGS_1984_Web_Mercator_Auxiliary_Sphere",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Mercator_Auxiliary_Sphere"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",0.0],PARAMETER["Standard_Parallel_1",0.0],PARAMETER["Auxiliary_Sphere_Type",0.0],UNIT["Meter",1.0]]');

const testPoint = {
  code: 'testmerc',
  xy: [-45007.0787624, 4151725.59875],
  ll: [5.364315,46.623154]
}
const proj = new proj4.Proj(testPoint.code);
console.log('proj', proj)
const xy = proj4.transform(proj4.WGS84, proj, proj4.toPoint(testPoint.ll));
const ll = proj4.transform(proj, proj4.WGS84, proj4.toPoint(testPoint.xy));
// console.log('xy', xy)
// console.log('ll', ll)




console.log('\n\n\n\n\n\n\n');



const transform = new Transformer();
transform.setSource('+proj=merc +lon_0=5.937 +lat_ts=45.027 +ellps=sphere')

const result = transform.forward({ x: -45007.0787624, y: 4151725.59875 })
console.log('result', result)
const backwards = transform.inverse(result)
console.log('backwards', backwards)









// transform.insertDefinition()

// const defData = '+proj=gnom +lat_0=90 +lon_0=0 +x_0=6300000 +y_0=6300000 +ellps=WGS84 +datum=WGS84 +units=m +no_defs'
// const defData = '+proj=merc +lon_0=5.937 +lat_ts=45.027 +ellps=sphere'

// interface Obj {
//   [key: string]: string | true
// }

// const paramObj: Obj = defData
//   .split('+')
//   .map((v) => v.trim())
//   .filter((a) => a.length > 0)
//   .reduce((res, a) => {
//     const [key, value] = a.split('=');
//     res[key.toLowerCase()] = value !== undefined && value.length > 0 ? value : true;
//     return res;
//   }, {} as Obj);

// console.log('paramObj', paramObj)
