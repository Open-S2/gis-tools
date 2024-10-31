import type { BBox } from '../../geometry';
import type { Pbf as Protobuf } from '../protobuf';

/** OSM Header Block */
export interface OSMHeader {
  bbox: BBox;
  required_features: string[];
  optional_features: string[];
  writingprogram?: string;
  source?: string;
  // Tags that allow continuing an Osmosis replication
  // Replication timestamp, expressed in seconds since the epoch,
  // otherwise the same value as in the "timestamp=..." field
  // in the state.txt file used by Osmosis.
  osmosis_replication_timestamp: number;
  // Replication sequence number (sequenceNumber in state.txt).
  osmosis_replication_sequence_number: number;
  // Replication base URL (from Osmosis' configuration.txt file).
  osmosis_replication_base_url?: string;
}

/**
 * The OSM Header Block
 * A block containing OSM header information that helps guide the parser
 * of the OSM data how to interpret the data.
 */
export class HeaderBlock {
  bbox = new HeaderBBox();
  // Additional tags to aid in parsing this dataset
  required_features: string[] = [];
  optional_features: string[] = [];
  writingprogram?: string;
  source?: string;
  // Tags that allow continuing an Osmosis replication
  // Replication timestamp, expressed in seconds since the epoch,
  // otherwise the same value as in the "timestamp=..." field
  // in the state.txt file used by Osmosis.
  osmosis_replication_timestamp = -1;
  // Replication sequence number (sequenceNumber in state.txt).
  osmosis_replication_sequence_number = -1;
  // Replication base URL (from Osmosis' configuration.txt file).
  osmosis_replication_base_url?: string;

  /** @param pbf - the Protobuf object to read from */
  constructor(pbf: Protobuf) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * Read the header block's contents into an object
   * @returns - the header object
   */
  toHeader(): OSMHeader {
    const {
      required_features,
      optional_features,
      writingprogram,
      source,
      osmosis_replication_timestamp,
      osmosis_replication_sequence_number,
      osmosis_replication_base_url,
    } = this;

    return {
      bbox: this.bbox.toBBox(),
      required_features,
      optional_features,
      writingprogram,
      source,
      osmosis_replication_timestamp,
      osmosis_replication_sequence_number,
      osmosis_replication_base_url,
    };
  }

  /**
   * Read in the contents of the header block
   * @param tag - the tag of the message
   * @param header - the header object to modify
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, header: HeaderBlock, pbf: Protobuf): void {
    if (tag === 1) header.bbox = pbf.readMessage(header.bbox.readLayer, header.bbox);
    else if (tag === 4) header.required_features.push(pbf.readString());
    else if (tag === 5) header.optional_features.push(pbf.readString());
    else if (tag === 16) header.writingprogram = pbf.readString();
    else if (tag === 17) header.source = pbf.readString();
    else if (tag === 32) header.osmosis_replication_timestamp = pbf.readVarint();
    else if (tag === 33) header.osmosis_replication_sequence_number = pbf.readVarint();
    else if (tag === 34) header.osmosis_replication_base_url = pbf.readString();
    // else throw new Error('unknown tag ' + tag);
  }
}

/**
 * The bounding box field in the OSM header. BBOX, as used in the OSM
 * header. Units are always in nanodegrees -- they do not obey
 * granularity rules.
 */
export class HeaderBBox {
  left = -1;
  right = -1;
  top = -1;
  bottom = -1;

  /**
   * Read the header block's contents into an object
   * @param tag - the tag of the message
   * @param bbox - the bbox object to modify
   * @param pbf - the Protobuf object to read from
   */
  readLayer(tag: number, bbox: HeaderBBox, pbf: Protobuf): void {
    if (tag === 1) bbox.left = pbf.readVarint();
    else if (tag === 2) bbox.right = pbf.readVarint();
    else if (tag === 3) bbox.top = pbf.readVarint();
    else if (tag === 4) bbox.bottom = pbf.readVarint();
    else throw new Error('unknown tag ' + tag);
  }

  /**
   * Returns the bounding box as a [left, bottom, right, top] array
   * @returns - [left, bottom, right, top]
   */
  toBBox(): [left: number, bottom: number, right: number, top: number] {
    return [this.left, this.bottom, this.right, this.top];
  }
}
