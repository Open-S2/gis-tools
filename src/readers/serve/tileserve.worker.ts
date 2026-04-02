declare const self: DedicatedWorkerGlobalScope;

import { TileStore, fileTypeToReader } from '../../index.js';

import type { S2CellId, TileStoreOptions } from '../../index.js';

/** Initial message sent to the worker */
export interface InitMessage {
  type: 'init';
  url: string; // url to fetch
  inputType: string;
  options: TileStoreOptions;
}

/** Tile request message sent from the source worker to this server worker */
export interface TileRequestMessage {
  type: 'tilerequest';
  id: S2CellId;
}

/** Tile response message sent from this server worker to the source worker */
export interface TileResponseMessage {
  type: 'tileresponse';
  id: S2CellId;
  data: ArrayBufferLike;
}

/**
 * # Server Tile Worker
 *
 * A worker that, given a source and options, fetches the data, converts it to tiles,
 * and handles requests
 */
class TileWorker {
  #sourceWorker?: MessagePort;
  #store?: TileStore;
  #textEncoder: TextEncoder = new TextEncoder();

  /**
   * Tile-ize input vector features and store them
   * @param event - the init message or a feature message
   */
  onmessage(event: MessageEvent<InitMessage | TileRequestMessage>): void {
    const { data } = event;
    if (data.type === 'init') {
      if (event.ports !== undefined && event.ports.length !== 0)
        this.#loadWorkerPort(event.ports[0], event.ports[1]);
      void this.#handleInit(data);
    } else if (event.type === 'tilerequest') this.#handleRequest(data);
  }

  /**
   * Load worker. First message that comes in upon creation of this worker
   * @param messagePort - the communication port to talk listen to a source worker's messages
   * @param postPort - the communication port to send messages to the source worker
   */
  #loadWorkerPort(messagePort: MessageChannel['port1'], postPort: MessageChannel['port2']): void {
    this.#sourceWorker = postPort;
    messagePort.onmessage = this.onmessage.bind(this);
  }

  /**
   * Handle an init message
   * @param event - the init message
   */
  async #handleInit(event: InitMessage) {
    // setup reader and store, then build
    const reader = await fileTypeToReader(event.url, event.inputType);
    this.#store = new TileStore(undefined, event.options);
    await this.#store.buildReader(reader);
  }

  /**
   * Handle a tile request
   * @param event - the tile request
   */
  #handleRequest(event: TileRequestMessage): void {
    const { id } = event;
    const tile = this.#store?.getTile(id);
    const data = this.#textEncoder.encode(JSON.stringify(tile)).buffer;
    const message: TileResponseMessage = { type: 'tileresponse', id, data };
    if (this.#sourceWorker !== undefined) this.#sourceWorker.postMessage(message, [data]);
    else self.postMessage(message);
  }
}

const tileWorker = new TileWorker();
self.onmessage = tileWorker.onmessage.bind(tileWorker);
