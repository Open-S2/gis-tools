import { BaseClient, BaseResponse } from './client/base';

/**
 *
 */
class FetchResponse extends BaseResponse {
  /**
   * BaseResponse facade for fetch API Response
   * @param response
   */
  constructor(response) {
    super();
    this.response = response;
  }

  /**
   *
   */
  get status() {
    return this.response.status;
  }

  /**
   * @param name
   */
  getHeader(name) {
    return this.response.headers.get(name);
  }

  /**
   *
   */
  async getData() {
    const data = this.response.arrayBuffer
      ? await this.response.arrayBuffer()
      : (await this.response.buffer()).buffer;
    return data;
  }
}

/**
 *
 */
export class FetchClient extends BaseClient {
  /**
   * @param url
   * @param credentials
   */
  constructor(url, credentials) {
    super(url);
    this.credentials = credentials;
  }

  /**
   * @param [options]
   * @param options.headers
   * @param options.signal
   * @returns
   */
  async request({ headers, signal } = {}) {
    const response = await fetch(this.url, {
      headers,
      credentials: this.credentials,
      signal,
    });
    return new FetchResponse(response);
  }
}
