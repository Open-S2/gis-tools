import { parseByteRanges, parseContentRange, parseContentType } from './httputils';
import { BaseSource } from './basesource';
import { BlockedSource } from './blockedsource';

import { FetchClient } from './fetch';

/**
 *
 */
class RemoteSource extends BaseSource {
  /**
   * @param client
   * @param headers
   * @param maxRanges
   * @param allowFullFile
   */
  constructor(client, headers, maxRanges, allowFullFile) {
    super();
    this.client = client;
    this.headers = headers;
    this.maxRanges = maxRanges;
    this.allowFullFile = allowFullFile;
    this._fileSize = null;
  }

  /**
   * @param slices
   * @param signal
   */
  async fetch(slices, signal) {
    // if we allow multi-ranges, split the incoming request into that many sub-requests
    // and join them afterwards
    if (this.maxRanges >= slices.length) {
      return this.fetchSlices(slices, signal);
    } else if (this.maxRanges > 0 && slices.length > 1) {
      // TODO: split into multiple multi-range requests
      // const subSlicesRequests = [];
      // for (let i = 0; i < slices.length; i += this.maxRanges) {
      //   subSlicesRequests.push(
      //     this.fetchSlices(slices.slice(i, i + this.maxRanges), signal),
      //   );
      // }
      // return (await Promise.all(subSlicesRequests)).flat();
    }

    // otherwise make a single request for each slice
    return Promise.all(slices.map((slice) => this.fetchSlice(slice, signal)));
  }

  /**
   * @param slices
   * @param signal
   */
  async fetchSlices(slices, signal) {
    const response = await this.client.request({
      headers: {
        ...this.headers,
        Range: `bytes=${slices
          .map(({ offset, length }) => `${offset}-${offset + length}`)
          .join(',')}`,
      },
      signal,
    });

    if (!response.ok) {
      throw new Error('Error fetching data.');
    } else if (response.status === 206) {
      const { type, params } = parseContentType(response.getHeader('content-type'));
      if (type === 'multipart/byteranges') {
        const byteRanges = parseByteRanges(await response.getData(), params.boundary);
        this._fileSize = byteRanges[0].fileSize || null;
        return byteRanges;
      }

      const data = await response.getData();

      const { start, end, total } = parseContentRange(response.getHeader('content-range'));
      this._fileSize = total || null;
      const first = [
        {
          data,
          offset: start,
          length: end - start,
        },
      ];

      if (slices.length > 1) {
        // we requested more than one slice, but got only the first
        // unfortunately, some HTTP Servers don't support multi-ranges
        // and return only the first

        // get the rest of the slices and fetch them iteratively
        const others = await Promise.all(
          slices.slice(1).map((slice) => this.fetchSlice(slice, signal)),
        );
        return first.concat(others);
      }
      return first;
    } else {
      if (!this.allowFullFile) {
        throw new Error('Server responded with full file');
      }
      const data = await response.getData();
      this._fileSize = data.byteLength;
      return [
        {
          data,
          offset: 0,
          length: data.byteLength,
        },
      ];
    }
  }

  /**
   * @param slice
   * @param signal
   */
  async fetchSlice(slice, signal) {
    const { offset, length } = slice;
    const response = await this.client.request({
      headers: {
        ...this.headers,
        Range: `bytes=${offset}-${offset + length}`,
      },
      signal,
    });

    // check the response was okay and if the server actually understands range requests
    if (!response.ok) {
      throw new Error('Error fetching data.');
    } else if (response.status === 206) {
      const data = await response.getData();

      const { total } = parseContentRange(response.getHeader('content-range'));
      this._fileSize = total || null;
      return {
        data,
        offset,
        length,
      };
    } else {
      if (!this.allowFullFile) {
        throw new Error('Server responded with full file');
      }

      const data = await response.getData();

      this._fileSize = data.byteLength;
      return {
        data,
        offset: 0,
        length: data.byteLength,
      };
    }
  }

  /**
   *
   */
  get fileSize() {
    return this._fileSize;
  }
}

/**
 * @param source
 * @param root0
 * @param root0.blockSize
 * @param root0.cacheSize
 */
function maybeWrapInBlockedSource(source, { blockSize, cacheSize }) {
  if (blockSize === null) {
    return source;
  }
  return new BlockedSource(source, { blockSize, cacheSize });
}

/**
 * @param url
 * @param root0
 * @param root0.headers
 * @param root0.credentials
 * @param root0.maxRanges
 * @param root0.allowFullFile
 */
export function makeFetchSource(
  url,
  { headers = {}, credentials, maxRanges = 0, allowFullFile = false, ...blockOptions } = {},
) {
  const client = new FetchClient(url, credentials);
  const source = new RemoteSource(client, headers, maxRanges, allowFullFile);
  return maybeWrapInBlockedSource(source, blockOptions);
}

/**
 * @param client
 * @param root0
 * @param root0.headers
 * @param root0.maxRanges
 * @param root0.allowFullFile
 */
export function makeCustomSource(
  client,
  { headers = {}, maxRanges = 0, allowFullFile = false, ...blockOptions } = {},
) {
  const source = new RemoteSource(client, headers, maxRanges, allowFullFile);
  return maybeWrapInBlockedSource(source, blockOptions);
}

/**
 * @param url
 * @param options
 * @param options.forceXHR
 * @param clientOptions
 */
export function makeRemoteSource(url, clientOptions = {}) {
  return makeFetchSource(url, clientOptions);
}
