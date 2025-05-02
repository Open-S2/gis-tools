declare let self: DedicatedWorkerGlobalScope;

import VectorTileWorker from './tileWorker.js';

const vecWorker = new VectorTileWorker();
self.onmessage = vecWorker.onmessage.bind(vecWorker);
