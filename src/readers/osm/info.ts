import type { PrimitiveBlock } from './primitive';
import type { Pbf as Protobuf } from 'open-vector-tile';

/**
 *
 */
export interface InfoBlock {
  version?: number;
  timestamp?: number;
  changeset?: number;
  uid?: number;
  user?: string;
  visible?: boolean;
}

/** Optional metadata that may be included into each primitive. */
export class Info {
  #version?: number;
  #timestamp?: number; // (millisec_stamp = timestamp*dateGranularity.)
  #changeset?: number;
  #uid?: number;
  #userSid?: number; // String IDs for usernames.
  // The visible flag is used to store history information. It indicates that
  // the current object version has been created by a delete operation on the
  // OSM API.
  // When a writer sets this flag, it MUST add a required_features tag with
  // value "HistoricalInformation" to the HeaderBlock.
  // If this flag is not available for some object it MUST be assumed to be
  // true if the file has the required_features tag "HistoricalInformation"
  // set.
  #visible = true;

  /**
   * @param primitiveBlock
   * @param pbf
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    pbf?: Protobuf,
  ) {
    if (pbf) pbf.readMessage(this.#readLayer, this);
  }

  /**
   *
   */
  toBlock(): InfoBlock {
    return {
      version: this.#version,
      timestamp: this.timeStamp(),
      changeset: this.#changeset,
      uid: this.#uid,
      user: this.user(),
      visible: this.#visible,
    };
  }

  /**
   * @param primitiveBlock
   * @param version
   * @param timestamp
   * @param changeset
   * @param uid
   * @param userSid
   * @param visible
   */
  static fromDense(
    primitiveBlock: PrimitiveBlock,
    version: number,
    timestamp: number,
    changeset: number,
    uid: number,
    userSid: number,
    visible?: boolean,
  ): Info {
    const info = new Info(primitiveBlock);
    info.#version = version;
    info.#timestamp = timestamp;
    info.#changeset = changeset;
    info.#uid = uid;
    info.#userSid = userSid;
    info.#visible = visible ?? true;
    return info;
  }

  /**
   *
   */
  timeStamp(): number | undefined {
    if (this.#timestamp === undefined) return;
    return this.#timestamp * this.primitiveBlock.dateGranularity;
  }

  // /**
  //  *
  //  */
  // changeset(): string | undefined {
  //   if (this.#changeset === undefined) return;
  //   return this.primitiveBlock.getString(this.#changeset);
  // }

  /**
   *
   */
  user(): string | undefined {
    if (this.#userSid === undefined) return;
    return this.primitiveBlock.getString(this.#userSid);
  }

  /**
   * @param tag
   * @param info
   * @param pbf
   */
  #readLayer(tag: number, info: Info, pbf: Protobuf): void {
    if (tag === 1) info.#version = pbf.readSVarint();
    else if (tag === 2) info.#timestamp = pbf.readSVarint();
    else if (tag === 3) info.#changeset = pbf.readSVarint();
    else if (tag === 4) info.#uid = pbf.readSVarint();
    else if (tag === 5) info.#userSid = pbf.readVarint();
    else if (tag === 6) info.#visible = pbf.readBoolean();
    else throw new Error(`unknown tag ${tag}`);
  }
}

/** DenseInfo */
export class DenseInfo {
  version: number[] = [];
  timestamp: number[] = []; // DELTA coded (millisec_stamp = timestamp*dateGranularity.)
  changeset: number[] = [];
  uid: number[] = []; // DELTA coded
  userSid: number[] = []; // String IDs for usernames. DELTA coded
  // The visible flag is used to store history information. It indicates that
  // the current object version has been created by a delete operation on the
  // OSM API.
  // When a writer sets this flag, it MUST add a required_features tag with
  // value "HistoricalInformation" to the HeaderBlock.
  // If this flag is not available for some object it MUST be assumed to be
  // true if the file has the required_features tag "HistoricalInformation"
  // set.
  visible?: boolean[] = [];

  /**
   * @param primitiveBlock
   * @param pbf
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    pbf: Protobuf,
  ) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * @param allocator
   */
  infos(): Info[] {
    const res: Info[] = [];
    let curTimestamp = 0;
    let curUid = 0;
    for (let i = 0; i < this.version.length; i++) {
      curTimestamp += this.timestamp[i];
      curUid += this.uid[i];
      res[i] = Info.fromDense(
        this.primitiveBlock,
        this.version[i],
        curTimestamp,
        this.changeset[i],
        curUid,
        this.userSid[i],
        this.getVisible(i),
      );
    }
    return res;
  }

  /**
   * @param i
   */
  getVisible(i: number): undefined | boolean {
    if (this.visible === undefined) return;
    return this.visible?.[i];
  }

  /**
   * @param tag
   * @param di
   * @param pbf
   */
  #readLayer(tag: number, di: DenseInfo, pbf: Protobuf): void {
    if (tag === 1) di.version = pbf.readPackedSVarint();
    else if (tag === 2) di.timestamp = pbf.readPackedSVarint();
    else if (tag === 3) di.changeset = pbf.readPackedSVarint();
    else if (tag === 4) di.uid = pbf.readPackedSVarint();
    else if (tag === 5) di.userSid = pbf.readPackedVarint();
    else if (tag === 6) di.visible = pbf.readPackedBoolean();
    else throw new Error(`unknown tag ${tag}`);
  }
}
