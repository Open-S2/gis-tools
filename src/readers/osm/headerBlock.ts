import type { BBox } from 's2-tools/geometry';
import type { Pbf as Protobuf } from 'open-vector-tile';

/**
 *
 */
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
 *
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

  /**
   * @param pbf
   */
  constructor(pbf: Protobuf) {
    pbf.readMessage(this.readLayer, this);
  }

  /**
   *
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
   * @param tag
   * @param header
   * @param pbf
   */
  readLayer(tag: number, header: HeaderBlock, pbf: Protobuf): void {
    if (tag === 1) header.bbox = pbf.readMessage(header.bbox.readLayer, header.bbox);
    else if (tag === 4) header.required_features.push(pbf.readString());
    else if (tag === 5) header.optional_features.push(pbf.readString());
    else if (tag === 16) header.writingprogram = pbf.readString();
    else if (tag === 17) header.source = pbf.readString();
    else if (tag === 32) header.osmosis_replication_timestamp = pbf.readVarint();
    else if (tag === 33) header.osmosis_replication_sequence_number = pbf.readVarint();
    else if (tag === 34) header.osmosis_replication_base_url = pbf.readString();
    else throw new Error('unknown tag ' + tag);
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
   * @param tag
   * @param bbox
   * @param pbf
   */
  readLayer(tag: number, bbox: HeaderBBox, pbf: Protobuf): void {
    if (tag === 1) bbox.left = pbf.readVarint();
    else if (tag === 2) bbox.right = pbf.readVarint();
    else if (tag === 3) bbox.top = pbf.readVarint();
    else if (tag === 4) bbox.bottom = pbf.readVarint();
    else throw new Error('unknown tag ' + tag);
  }

  /**
   *
   */
  toBBox(): [left: number, bottom: number, right: number, top: number] {
    return [this.left, this.bottom, this.right, this.top];
  }
}
