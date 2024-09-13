import { Relation } from './relation';
import { Way } from './way';
import { DenseNodes, Node } from './node';

import type { OSMReader } from '.';
import type { Pbf as Protobuf } from 'open-vector-tile';

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
   * @param pbf
   * @param reader
   */
  constructor(
    public pbf: Protobuf,
    public reader: OSMReader,
  ) {
    pbf.readFields(this.#readLayer, this, 0);
  }

  /**
   * @param index
   */
  getString(index: number): string {
    return this.stringtable.get(index);
  }

  /**
   * @param keys
   * @param values
   */
  tags(keys: number[], values: number[]): Record<string, string> {
    const res: Record<string, string> = {};
    for (let i = 0; i < keys.length; i++) {
      res[this.getString(keys[i])] = this.getString(values[i]);
    }
    return res;
  }

  /**
   * @param tag
   * @param primitiveBlock
   * @param pb
   * @param pbf
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
  nodes: Node[] = [];
  ways: Way[] = [];
  relations: Relation[] = [];
  changesets: ChangeSet[] = [];
  /**
   * @param primitiveBlock
   * @param pbf
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public pbf: Protobuf,
  ) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * @param tag
   * @param pg
   * @param pbf
   */
  #readLayer(tag: number, pg: PrimitiveGroup, pbf: Protobuf): void {
    const { nodes, ways, relations, primitiveBlock } = pg;
    const { reader } = primitiveBlock;
    const { skipNodes, skipWays, skipRelations } = reader;
    const skipWR = skipWays && skipRelations;

    if (tag === 1) {
      const node = new Node(primitiveBlock, reader, pbf);
      if (!skipWR) reader.nodes.set(node.id, node.toVectorGeometry());
      if (!node.isFilterable() || skipNodes) nodes.push(node);
    } else if (tag === 2) {
      const dn = new DenseNodes(primitiveBlock, reader, pbf);
      for (const node of dn.nodes()) {
        if (!skipWR) reader.nodes.set(node.id, node.toVectorGeometry());
        if (!node.isFilterable() || skipNodes) nodes.push(node);
      }
    } else if (tag === 3) {
      if (skipWR) return;
      const way = new Way(primitiveBlock, reader, pbf);
      const wayLine = way.toVectorGeometry();
      if (wayLine !== undefined) reader.ways.set(way.id, wayLine);
      if (!way.isFilterable() || skipWays) ways.push(way);
    } else if (tag === 4) {
      if (skipWR) return;
      const relation = new Relation(primitiveBlock, reader, pbf);
      if (!relation.isFilterable() || skipRelations) relations.push(relation);
    } else if (tag === 5) {
      this.changesets.push(new ChangeSet(pbf));
    }
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
   * @param pbf
   */
  constructor(pbf: Protobuf) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * @param index
   */
  get(index: number): string {
    return this.strings[index];
  }

  /**
   * @param tag
   * @param st
   * @param pbf
   */
  #readLayer(tag: number, st: StringTable, pbf: Protobuf): void {
    if (tag == 1) st.strings.push(pbf.readString());
    else throw new Error(`unknown tag ${tag}`);
  }
}

/** This is kept for backwards compatibility but not used anywhere. */
export class ChangeSet {
  id = 0;

  /**
   * @param pbf
   */
  constructor(pbf: Protobuf) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * @param tag
   * @param cs
   * @param pbf
   */
  #readLayer(tag: number, cs: ChangeSet, pbf: Protobuf): void {
    if (tag == 1) cs.id = pbf.readVarint();
    else throw new Error(`unknown tag ${tag}`);
  }
}
