// declare let self: Worker;

// import { FileMultiMap } from '../../../dataStore/multimap/file';
// import VectorTileWorker from './tileWorker';

// import type { FeatureMetadata } from './tileWorker';
// import type { VectorFeature } from '../../../geometry';

// /** Convert a vector feature to a collection of tiles and store each tile feature */
// class FileVectorTileWorker extends VectorTileWorker {
//   vectorStore = new FileMultiMap<VectorFeature<FeatureMetadata>>();
// }

// const vecWorker = new FileVectorTileWorker();
// self.onmessage = vecWorker.onmessage.bind(vecWorker);
