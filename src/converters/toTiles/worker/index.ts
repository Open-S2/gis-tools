declare let self: Worker;

import VectorTileWorker from './tileWorker';

const vecWorker = new VectorTileWorker();
self.onmessage = vecWorker.onmessage.bind(vecWorker);
