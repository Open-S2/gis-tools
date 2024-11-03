declare let self: Worker;

import { FileMultiMap } from '../../../dataStore/multimap/file';
import VectorTileWorker from './vectorTileWorker';

import type { VectorFeature } from '../../../geometry';

/** Convert a vector feature to a collection of tiles and store each tile feature */
class FileVectorTileWorker extends VectorTileWorker {
  writer = new FileMultiMap<VectorFeature>();
}

const vecWorker = new FileVectorTileWorker();
self.onmessage = vecWorker.onmessage.bind(vecWorker);
