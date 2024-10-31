import { Way } from './way';
import { DenseNodes, Node } from './node';
import { Relation, getNodeRelationPairs } from './relation';

import type { IntermediateNodeMember } from './relation';
import type { Pbf as Protobuf } from '../../readers/protobuf';
import type { InfoBlock, OSMReader } from '.';

/** The expected metadata in the VectorFeature for all types (node, way, relation) */
export interface Metadata {
  info: InfoBlock;
  nodes?: IntermediateNodeMember[];
  relation?: {
    role: string;
    properties: Record<string, string>;
  };
}

/**
 * NOTE: currently relations are stored, but we don't wait for the Block to store all relations
 * before we start testing primtiveHandle against the data. This is a problem because
 * relations reference eachother at times, and we need to be able to resolve those references
 * before we can run relationHandle against the data. This isn't an important issue since
 * in practice, all relations that reference eachother often produce garbage or unusable data.
 * But it would be *nice* to fix this. Morbidly enough, the "BEST" solution is to treat relations
 * like we do nodes and ways since relations could possibly reference eachother outside their own block.
 * From a practical standpoint, I can't see this being worth the effort or memory/time cost.
 */
export class PrimitiveBlock {
  stringtable!: StringTable;
  primitiveGroups: PrimitiveGroup[] = [];
  // Granularity, units of nanodegrees, used to store coordinates in this block.
  granularity = 100;
  // Offset value between the output coordinates and the granularity grid in units of nanodegrees.
  latOffset = 0;
  lonOffset = 0;
  // Granularity of dates, normally represented in units of milliseconds since the 1970 epoch.
  dateGranularity = 1000;

  /**
   * @param pbf - the Protobuf object to read from
   * @param reader - the OSMReader to modify
   */
  constructor(
    public pbf: Protobuf,
    public reader: OSMReader,
  ) {
    pbf.readFields(this.#readLayer, this, 0);
  }

  /**
   * Get a string from the string table at the given index
   * @param index - the index of the string in the string table
   * @returns - the string
   */
  getString(index: number): string {
    return this.stringtable.get(index);
  }

  /**
   * Get a record of strings from the string table
   * @param keys - list of indices for the keys
   * @param values - list of indices for the values
   * @returns - the record or object containing the key-value pairs
   */
  tags(keys: number[], values: number[]): Record<string, string> {
    const res: Record<string, string> = {};
    for (let i = 0; i < keys.length; i++) {
      res[this.getString(keys[i])] = this.getString(values[i]);
    }
    return res;
  }

  /**
   * Read the primitive block's contents into an object
   * @param tag - the tag of the message
   * @param pb - the primitive block to modify
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, pb: PrimitiveBlock, pbf: Protobuf): void {
    if (tag === 1) pb.stringtable = new StringTable(pbf);
    else if (tag === 2) pb.primitiveGroups.push(new PrimitiveGroup(pb, pbf));
    else if (tag === 17) pb.granularity = pbf.readVarint();
    else if (tag === 18) pb.dateGranularity = pbf.readVarint();
    else if (tag === 19) pb.latOffset = pbf.readVarint();
    else if (tag === 20) pb.lonOffset = pbf.readVarint();
    else throw new Error(`unknown tag ${tag}`);
  }
}

/** Group of OSMPrimitives. All primitives in a group must be the same type. */
export class PrimitiveGroup {
  // changesets: ChangeSet[] = [];
  /**
   * @param primitiveBlock - the parent PrimitiveBlock
   * @param pbf - the Protobuf object to read from
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public pbf: Protobuf,
  ) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * @param tag - the tag of the message
   * @param pg - the primitive group to modify
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, pg: PrimitiveGroup, pbf: Protobuf): void {
    const { primitiveBlock } = pg;
    const { reader } = primitiveBlock;
    const { skipWays, skipRelations } = reader;
    const skipWR = skipWays && skipRelations;

    if (tag === 1) {
      const node = new Node(primitiveBlock, reader, pbf);
      if (!node.isFilterable()) reader.nodes.set(node.id, node.toVectorFeature());
      if (!skipWR) reader.nodeGeometry.set(node.id, node.toVectorGeometry());
    } else if (tag === 2) {
      const dn = new DenseNodes(primitiveBlock, reader, pbf);
      for (const node of dn.nodes()) {
        if (!node.isFilterable()) reader.nodes.set(node.id, node.toVectorFeature());
        if (!skipWR) reader.nodeGeometry.set(node.id, node.toVectorGeometry());
      }
    } else if (tag === 3) {
      if (skipWR) return;
      const way = new Way(primitiveBlock, reader, pbf);
      const refs = way.nodeRefs();
      const ivf = way.toVectorFeature();
      if (refs.length >= 2) reader.wayGeometry.set(way.id, refs);
      if (ivf !== undefined && !way.isFilterable()) reader.ways.set(way.id, ivf);
    } else if (tag === 4) {
      if (skipRelations) return;
      const relation = new Relation(primitiveBlock, reader, pbf);
      const ivf = relation.toIntermediateFeature();
      if (ivf === undefined) return;
      for (const member of getNodeRelationPairs(ivf.members)) {
        reader.nodeRelationPairs.set(member.node, member);
      }
      if (!relation.isFilterable()) reader.relations.set(relation.id, ivf);
    }
    // else if (tag === 5) {
    //   this.changesets.push(new ChangeSet(pbf));
    // }
  }
}

/**
 * String table, contains the common strings in each block.
 * Note that we reserve index '0' as a delimiter, so the entry at that
 * index in the table is ALWAYS blank and unused.
 * NOTE: OSM isn't safe and allows " inside of strings, so we have to replace them with '
 * NOTE: OSM isn't safe and allows \ at the end of strings, so we have to remove them so it can be properly parsed.
 */
export class StringTable {
  strings: string[] = [];

  /**
   * @param pbf - the Protobuf object to read from
   */
  constructor(pbf: Protobuf) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * @param index - the index of the string
   * @returns - the string
   */
  get(index: number): string {
    return this.strings[index];
  }

  /**
   * @param tag - the tag of the message
   * @param st - the string table to modify
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, st: StringTable, pbf: Protobuf): void {
    if (tag === 1) st.strings.push(pbf.readString());
    else throw new Error(`unknown tag ${tag}`);
  }
}

/** This is kept for backwards compatibility but not used anywhere. */
// export class ChangeSet {
//   id = 0;

//   /**
//    * @param pbf
//    */
//   constructor(pbf: Protobuf) {
//     pbf.readMessage(this.#readLayer, this);
//   }

//   /**
//    * @param tag
//    * @param cs
//    * @param pbf
//    */
//   #readLayer(tag: number, cs: ChangeSet, pbf: Protobuf): void {
//     if (tag === 1) cs.id = pbf.readVarint();
//     else throw new Error(`unknown tag ${tag}`);
//   }
// }
