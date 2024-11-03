declare let self: Worker;

import VectorTileWorker from './vectorTileWorker';

const vecWorker = new VectorTileWorker();
self.onmessage = vecWorker.onmessage.bind(vecWorker);
