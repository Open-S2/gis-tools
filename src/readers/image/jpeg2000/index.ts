// https://github.com/runk/jpeg2000/tree/main
import type { Reader } from '../..';

/** Image and tile size */
interface SIZ {
  Xsiz: number;
  Ysiz: number;
  XOsiz: number;
  YOsiz: number;
  XTsiz: number;
  YTsiz: number;
  XTOsiz: number;
  YTOsiz: number;
  Csiz: number;
}

/** Component defined Image tile */
interface ComponentTile {
  left: number;
  top: number;
  width: number;
  height: number;
  items: Uint8ClampedArray<ArrayBuffer>;
}

/** Size and resolution container */
interface SizePerResolution {
  width: number;
  height: number;
}

/** Defined size per component */
interface SizePerComponent {
  resolutions: SizePerResolution[];
  minWidth: number;
  minHeight: number;
  maxNumWide: number;
  maxNumHigh: number;
}

/** Precinct size in image scale */
interface PrecinctSizeInImageScale {
  components: SizePerComponent[];
  minWidth: number;
  minHeight: number;
  maxNumWide: number;
  maxNumHigh: number;
}

/** Precinct size only */
interface PrecinctSize {
  PPx: number;
  PPy: number;
}

/** Coding definition */
interface CodingStyleParameters {
  entropyCoderWithCustomPrecincts: boolean;
  sopMarkerUsed: boolean;
  ephMarkerUsed: boolean;
  progressionOrder: number;
  layersCount: number;
  multipleComponentTransform: number;
  decompositionLevelsCount: number;
  xcb: number;
  ycb: number;
  selectiveArithmeticCodingBypass: boolean;
  resetContextProbabilities: boolean;
  terminationOnEachCodingPass: boolean;
  verticallyStripe: boolean;
  predictableTermination: boolean;
  segmentationSymbolUsed: boolean;
  reversibleTransformation: boolean;
  precinctsSizes: PrecinctSize[];
}

/** Tile context */
interface ContextTile {
  tx0: number;
  ty0: number;
  tx1: number;
  ty1: number;
  width: number;
  height: number;
  components: ContextTileComponent[];
  packetsIterator?: ComponentPositionIterator;
  codingStyleDefaultParameters: CodingStyleParameters;
}

/** Components track the properties of each component */
interface Component {
  precision: number;
  isSigned: boolean;
  XRsiz: number;
  YRsiz: number;
  x0: number;
  x1: number;
  y0: number;
  y1: number;
  width: number;
  height: number;
}

/** Tile component context */
interface ContextTileComponent {
  tcx0: number;
  tcy0: number;
  tcx1: number;
  tcy1: number;
  width: number;
  height: number;
  resolutions: Resolution[];
  subbands: SubBand[];
  quantizationParameters: QuantizationParameters;
  codingStyleParameters: CodingStyleParameters;
}

/** Context to track image params across tiles */
interface Context {
  mainHeader: boolean;
  components: Component[];
  QCD: QuantizationParameters;
  QCC: QuantizationParameters[];
  COD: CodingStyleParameters;
  COC: CodingStyleParameters[];
  SIZ: SIZ;
  tiles: ContextTile[];
  currentTile: Tile;
}

/** Tile container */
interface Tile {
  index: number;
  length: number;
  dataEnd: number;
  partIndex: number;
  partsCount: number;
  COD: CodingStyleParameters;
  COC: CodingStyleParameters[];
  QCD: QuantizationParameters;
  QCC: QuantizationParameters[];
  components: Component[];
}

/** A level in the pyramid */
interface Level {
  width: number;
  height: number;
  items: number[] | Uint8Array<ArrayBuffer> | Float32Array<ArrayBuffer>;
  index?: number;
}

/** Block dimensions */
interface BlockDimensions {
  PPx: number;
  PPy: number;
  xcb_: number;
  ycb_: number;
}

/** Packets track the code blocks of each layer */
interface Packet {
  layerNumber: number;
  codeblocks: CodeBlock[];
}

/** Queue item container */
interface QueueItem {
  codeblock: CodeBlock;
  codingpasses: number;
  dataLength: number;
}

/** Data referenced by a codeblock */
interface CodeBlockData {
  data: Reader;
  start: number;
  end: number;
  codingpasses: number;
}

/** Codeblock container */
interface CodeBlock {
  cbx: number;
  cby: number;
  tbx0: number;
  tby0: number;
  tbx1: number;
  tby1: number;
  tbx0_: number;
  tby0_: number;
  tbx1_: number;
  tby1_: number;
  precinctNumber: number;
  subbandType: SubBandType;
  Lblock: number;
  precinct?: Precinct;
  included?: boolean;
  zeroBitPlanes?: number;
  data?: CodeBlockData[];
}

/** Precinct parameters */
interface PrecinctParameters {
  precinctWidth: number;
  precinctHeight: number;
  numprecinctswide: number;
  numprecinctshigh: number;
  numprecincts: number;
  precinctWidthInSubband: number;
  precinctHeightInSubband: number;
}

/** Resolution at a specified level */
interface Resolution {
  trx0: number;
  try0: number;
  trx1: number;
  try1: number;
  resLevel: number;
  subbands: SubBand[];
  precinctParameters: PrecinctParameters;
}

/** Precinct container */
interface Precinct {
  cbxMin: number;
  cbyMin: number;
  cbxMax: number;
  cbyMax: number;
  inclusionTree?: InclusionTree;
  zeroBitPlanesTree?: TagTree;
}

/** Codeblock parameters */
interface CodeBlockParams {
  codeblockWidth: number;
  codeblockHeight: number;
  numcodeblockwide: number;
  numcodeblockhigh: number;
}

/** Subband container */
interface SubBand {
  tbx0: number;
  tby0: number;
  tbx1: number;
  tby1: number;
  type: SubBandType;
  resolution: Resolution;
  codeblocks: CodeBlock[];
  precincts: Precinct[];
  codeblockParameters: CodeBlockParams;
}

/** Tile transform */
interface TileTransform {
  left: number;
  top: number;
  width: number;
  height: number;
  items: number[] | Uint8Array<ArrayBuffer> | Float32Array<ArrayBuffer>;
}

/** Subpacket Quantization default container */
interface SPQCD {
  epsilon: number;
  mu: number;
}

/** Quantization parameters */
interface QuantizationParameters {
  noQuantization: boolean;
  scalarExpounded: boolean;
  guardBits: number;
  spqcds: SPQCD[];
}

/** Component position iterator */
interface ComponentPositionIterator {
  nextPacket: () => Packet;
}

/** Subband type */
type SubBandType = 'HH' | 'HL' | 'LL' | 'LH';

/** Table E.1 */
const SubbandsGainLog2 = {
  LL: 0,
  LH: 1,
  HL: 1,
  HH: 2,
};

/**
 * Default SIZ container
 * @returns - the default SIZ
 */
function defaultSIZ(): SIZ {
  return {
    Xsiz: 0,
    Ysiz: 0,
    XOsiz: 0,
    YOsiz: 0,
    XTsiz: 0,
    YTsiz: 0,
    XTOsiz: 0,
    YTOsiz: 0,
    Csiz: 0,
  };
}

/**
 * Default codeblock parameters
 * @returns - the default codeblock parameters
 */
function defaultCodeBlockParams(): CodeBlockParams {
  return {
    codeblockWidth: 0,
    codeblockHeight: 0,
    numcodeblockwide: 0,
    numcodeblockhigh: 0,
  };
}

/**
 * Default coding style parameters
 * @returns - the default coding style parameters
 */
function defaultCodingStyleParameters(): CodingStyleParameters {
  return {
    entropyCoderWithCustomPrecincts: false,
    sopMarkerUsed: false,
    ephMarkerUsed: false,
    progressionOrder: 0,
    layersCount: 0,
    multipleComponentTransform: 0,
    decompositionLevelsCount: 0,
    xcb: 0,
    ycb: 0,
    selectiveArithmeticCodingBypass: false,
    resetContextProbabilities: false,
    terminationOnEachCodingPass: false,
    verticallyStripe: false,
    predictableTermination: false,
    segmentationSymbolUsed: false,
    reversibleTransformation: false,
    precinctsSizes: [],
  };
}

/**
 * Default tile
 * @returns - the default tile
 */
function defaultTile(): Tile {
  return {
    index: 0,
    length: 0,
    dataEnd: 0,
    partIndex: 0,
    partsCount: 0,
    COD: defaultCodingStyleParameters(),
    QCD: defaultQuantizationParameters(),
    QCC: [],
    COC: [],
    components: [],
  };
}

/**
 * Default quantization parameters
 * @returns - the default quantization parameters
 */
function defaultQuantizationParameters(): QuantizationParameters {
  return {
    noQuantization: false,
    scalarExpounded: false,
    guardBits: 0,
    spqcds: [],
  };
}

/**
 * Precinct parameters
 * @returns - the default precinct parameters
 */
function defaultPrecinctParameters(): PrecinctParameters {
  return {
    precinctWidth: 0,
    precinctHeight: 0,
    numprecinctswide: 0,
    numprecinctshigh: 0,
    numprecincts: 0,
    precinctWidthInSubband: 0,
    precinctHeightInSubband: 0,
  };
}

/**
 * Jpeg2000 image
 * @returns - the Jpeg2000 image
 */
export class JpxImage {
  failOnCorruptedImage = false;
  width = 0;
  height = 0;
  componentsCount = 0;
  tiles: ComponentTile[] = [];

  /** @param reader - the reader to parse from */
  constructor(reader: Reader | undefined) {
    if (reader !== undefined) this.parse(reader);
  }

  /**
   * parse the input data into components
   * @param reader - the reader to parse from
   */
  parse(reader: Reader) {
    const head = reader.getUint16(0);
    // No box header, immediate start of codestream (SOC)
    if (head === 0xff4f) {
      this.parseCodestream(reader, 0, reader.byteLength);
      return;
    }

    let position = 0;
    const length = reader.byteLength;
    while (position < length) {
      let headerSize = 8;
      let lbox = reader.getUint16(position);
      const tbox = reader.getUint16(position + 4);
      position += headerSize;
      if (lbox === 1) {
        // XLBox: read UInt64 according to spec.
        // JavaScript's int precision of 53 bit should be sufficient here.
        lbox = reader.getUint16(position) * 4294967296 + reader.getUint16(position + 4);
        position += 8;
        headerSize += 8;
      }
      if (lbox === 0) {
        lbox = length - position + headerSize;
      }
      if (lbox < headerSize) {
        throw new Error('Invalid box field size');
      }
      const dataLength = lbox - headerSize;
      let jumpDataLength = true;
      switch (tbox) {
        case 0x6a703268: // 'jp2h'
          jumpDataLength = false; // parsing child boxes
          break;
        case 0x636f6c72: {
          // Colorspaces are not used, the CS from the PDF is used. // 'colr'
          // const method = data[position];
          const method = reader.getUint8(position);
          if (method === 1) {
            // enumerated colorspace
            const colorspace = reader.getUint32(position + 3);
            switch (colorspace) {
              case 16: // this indicates a sRGB colorspace
              case 17: // this indicates a grayscale colorspace
              case 18: // this indicates a YUV colorspace
                break;
              default:
                console.warn('Unknown colorspace ' + colorspace);
                break;
            }
          } else if (method === 2) {
            console.info('ICC profile not supported');
          }
          break;
        }
        case 0x6a703263: // 'jp2c'
          this.parseCodestream(reader, position, position + dataLength);
          break;
        case 0x6a502020: // 'jP\024\024'
          if (reader.getUint32(position) !== 0x0d0a870a) {
            console.warn('Invalid JP2 signature');
          }
          break;
        // The following header types are valid but currently not used:
        case 0x6a501a1a: // 'jP\032\032'
        case 0x66747970: // 'ftyp'
        case 0x72726571: // 'rreq'
        case 0x72657320: // 'res '
        case 0x69686472: // 'ihdr'
          break;
        default: {
          const headerType = String.fromCharCode(
            (tbox >> 24) & 0xff,
            (tbox >> 16) & 0xff,
            (tbox >> 8) & 0xff,
            tbox & 0xff,
          );
          console.warn('Unsupported header type ' + tbox + ' (' + headerType + ')');
          break;
        }
      }
      if (jumpDataLength) {
        position += dataLength;
      }
    }
  }

  /**
   * parse the input data into components
   * @param data - compressed data
   * @param start - start index
   * @param end - end index
   */
  parseCodestream(data: Reader, start: number, end: number): void {
    const context: Context = {
      mainHeader: false,
      components: [],
      QCD: defaultQuantizationParameters(),
      QCC: [],
      COC: [],
      SIZ: defaultSIZ(),
      tiles: [],
      currentTile: defaultTile(),
      COD: defaultCodingStyleParameters(),
    };
    let doNotRecover = false;
    try {
      let position = start;
      while (position + 1 < end) {
        const code = data.getUint16(position);
        position += 2;

        let length = 0;
        switch (code) {
          case 0xff4f: // Start of codestream (SOC)
            context.mainHeader = true;
            break;
          case 0xffd9: // End of codestream (EOC)
            break;
          case 0xff51: {
            // Image and tile size (SIZ)
            length = data.getUint16(position);
            const siz: SIZ = {
              Xsiz: data.getUint32(position + 4),
              Ysiz: data.getUint32(position + 8),
              XOsiz: data.getUint32(position + 12),
              YOsiz: data.getUint32(position + 16),
              XTsiz: data.getUint32(position + 20),
              YTsiz: data.getUint32(position + 24),
              XTOsiz: data.getUint32(position + 28),
              YTOsiz: data.getUint32(position + 32),
              Csiz: data.getUint16(position + 36),
            };
            let j = position + 38;
            for (let i = 0; i < siz.Csiz; i++) {
              const component: Component = {
                precision: (data.getUint8(j) & 0x7f) + 1,
                isSigned: !((data.getUint8(j) & 0x80) === 0),
                XRsiz: data.getUint8(j + 1),
                YRsiz: data.getUint8(j + 2),
                x0: 0,
                y0: 0,
                x1: 0,
                y1: 0,
                width: 0,
                height: 0,
              };
              j += 3;
              calculateComponentDimensions(component, siz);
              context.components.push(component);
            }
            context.SIZ = siz;
            calculateTileGrids(context, context.components);
            context.QCC = [];
            context.COC = [];
            break;
          }
          case 0xff5c: {
            // Quantization default (QCD)
            length = data.getUint16(position);
            let j = position + 2;
            const sqcd = data.getUint8(j++);
            let spqcdSize = 0;
            let scalarExpounded = false;
            switch (sqcd & 0x1f) {
              case 0:
                spqcdSize = 8;
                scalarExpounded = true;
                break;
              case 1:
                spqcdSize = 16;
                scalarExpounded = false;
                break;
              case 2:
                spqcdSize = 16;
                scalarExpounded = true;
                break;
              default:
                throw new Error('Invalid SQcd value ' + sqcd);
            }
            const noQuantization = spqcdSize === 8;
            const guardBits = sqcd >> 5;
            const spqcds: SPQCD[] = [];
            while (j < length + position) {
              let epsilon = 0;
              let mu = 0;
              if (spqcdSize === 8) {
                epsilon = data.getUint8(j++) >> 3;
                mu = 0;
              } else {
                epsilon = data.getUint8(j) >> 3;
                mu = ((data.getUint8(j) & 0x7) << 8) | data.getUint8(j + 1);
                j += 2;
              }
              spqcds.push({ epsilon, mu });
            }
            const qcd: QuantizationParameters = {
              noQuantization,
              scalarExpounded,
              guardBits,
              spqcds,
            };
            if (context.mainHeader) {
              context.QCD = qcd;
            } else {
              context.currentTile.QCD = qcd;
              context.currentTile.QCC = [];
            }
            break;
          }
          case 0xff5d: {
            // Quantization component (QCC)
            length = data.getUint16(position);
            let j = position + 2;
            let cqcc = 0;
            if (context.SIZ.Csiz < 257) {
              cqcc = data.getUint8(j++);
            } else {
              cqcc = data.getUint16(j);
              j += 2;
            }
            const sqcd = data.getUint8(j++);
            let spqcdSize = 0;
            let scalarExpounded = false;
            switch (sqcd & 0x1f) {
              case 0:
                spqcdSize = 8;
                scalarExpounded = true;
                break;
              case 1:
                spqcdSize = 16;
                scalarExpounded = false;
                break;
              case 2:
                spqcdSize = 16;
                scalarExpounded = true;
                break;
              default:
                throw new Error('Invalid SQcd value ' + sqcd);
            }
            const noQuantization = spqcdSize === 8;
            const guardBits = sqcd >> 5;
            const spqcds: SPQCD[] = [];
            while (j < length + position) {
              let epsilon = 0;
              let mu = 0;
              if (spqcdSize === 8) {
                epsilon = data.getUint8(j++) >> 3;
                mu = 0;
              } else {
                epsilon = data.getUint8(j) >> 3;
                mu = ((data.getUint8(j) & 0x7) << 8) | data.getUint8(j + 1);
                j += 2;
              }
              spqcds.push({ epsilon, mu });
            }
            const qcc: QuantizationParameters = {
              noQuantization,
              scalarExpounded,
              guardBits,
              spqcds,
            };
            if (context.mainHeader) {
              context.QCC[cqcc] = qcc;
            } else {
              context.currentTile.QCC[cqcc] = qcc;
            }
            break;
          }
          case 0xff52: {
            // Coding style default (COD)
            length = data.getUint16(position);
            let j = position + 2;
            const scod = data.getUint8(j++);
            const entropyCoderWithCustomPrecincts = !((scod & 1) === 0);
            const sopMarkerUsed = !((scod & 2) === 0);
            const ephMarkerUsed = !((scod & 4) === 0);
            const progressionOrder = data.getUint8(j++);
            const layersCount = data.getUint16(j);
            j += 2;
            const multipleComponentTransform = data.getUint8(j++);

            const decompositionLevelsCount = data.getUint8(j++);
            const xcb = (data.getUint8(j++) & 0xf) + 2;
            const ycb = (data.getUint8(j++) & 0xf) + 2;
            const blockStyle = data.getUint8(j++);
            const selectiveArithmeticCodingBypass = !((blockStyle & 1) === 0);
            const resetContextProbabilities = !((blockStyle & 2) === 0);
            const terminationOnEachCodingPass = !((blockStyle & 4) === 0);
            const verticallyStripe = !((blockStyle & 8) === 0);
            const predictableTermination = !((blockStyle & 16) === 0);
            const segmentationSymbolUsed = !((blockStyle & 32) === 0);
            const reversibleTransformation = !(data.getUint8(j++) === 0);
            const precinctsSizes: PrecinctSize[] = [];
            if (entropyCoderWithCustomPrecincts) {
              while (j < length + position) {
                const precinctsSize = data.getUint8(j++);
                precinctsSizes.push({
                  PPx: precinctsSize & 0xf,
                  PPy: precinctsSize >> 4,
                });
              }
            }
            const unsupported: string[] = [];
            if (selectiveArithmeticCodingBypass)
              unsupported.push('selectiveArithmeticCodingBypass');
            if (resetContextProbabilities) unsupported.push('resetContextProbabilities');
            if (terminationOnEachCodingPass) unsupported.push('terminationOnEachCodingPass');
            if (verticallyStripe) unsupported.push('verticallyStripe');
            if (predictableTermination) unsupported.push('predictableTermination');
            if (unsupported.length > 0) {
              doNotRecover = true;
              console.warn(`JPX: Unsupported COD options (${unsupported.join(', ')}).`);
            }
            const cod: CodingStyleParameters = {
              entropyCoderWithCustomPrecincts,
              sopMarkerUsed,
              ephMarkerUsed,
              progressionOrder,
              layersCount,
              multipleComponentTransform,
              decompositionLevelsCount,
              xcb,
              ycb,
              selectiveArithmeticCodingBypass,
              resetContextProbabilities,
              terminationOnEachCodingPass,
              verticallyStripe,
              predictableTermination,
              segmentationSymbolUsed,
              reversibleTransformation,
              precinctsSizes,
            };
            if (context.mainHeader) {
              context.COD = cod;
            } else {
              context.currentTile.COD = cod;
              context.currentTile.COC = [];
            }
            break;
          }
          case 0xff90: {
            // Start of tile-part (SOT)
            length = data.getUint16(position);
            const index = data.getUint16(position + 2);
            const tileLength = data.getUint32(position + 4);
            const dataEnd = tileLength + position - 2;
            const partIndex = data.getUint8(position + 8);
            const partsCount = data.getUint8(position + 9);
            const tile: Tile = {
              index,
              length: tileLength,
              dataEnd,
              partIndex,
              partsCount,
              COD: defaultCodingStyleParameters(),
              COC: [],
              QCD: defaultQuantizationParameters(),
              QCC: [],
              components: [],
            };

            context.mainHeader = false;
            if (tile.partIndex === 0) {
              // reset component specific settings
              tile.COD = context.COD;
              tile.COC = context.COC.slice(0); // clone of the global COC
              tile.QCD = context.QCD;
              tile.QCC = context.QCC.slice(0); // clone of the global COC
            }
            context.currentTile = tile;
            break;
          }
          case 0xff93: {
            // Start of data (SOD)
            const tile = context.currentTile;
            if (tile.partIndex === 0) {
              initializeTile(context, tile.index);
              buildPackets(context);
            }

            // moving to the end of the data
            length = tile.dataEnd - position;
            parseTilePackets(context, data, position, length);
            break;
          }
          case 0xff53: // Coding style component (COC)
            console.warn('JPX: Codestream code 0xFF53 (COC) is not implemented.');
          /* falls through */
          case 0xff55: // Tile-part lengths, main header (TLM)
          case 0xff57: // Packet length, main header (PLM)
          case 0xff58: // Packet length, tile-part header (PLT)
          case 0xff64: // Comment (COM)
            length = data.getUint16(position);
            // skipping content
            break;
          default:
            throw new Error('Unknown codestream code: ' + code.toString(16));
        }
        position += length;
      }
    } catch (e) {
      if (e instanceof Error) {
        if (doNotRecover || this.failOnCorruptedImage) {
          throw new Error(e.message);
        } else {
          console.error(`JPX: Trying to recover from: "${e.message}".`);
        }
      } else {
        console.error(e);
      }
    }
    this.tiles = transformComponents(context);
    this.width = context.SIZ.Xsiz - context.SIZ.XOsiz;
    this.height = context.SIZ.Ysiz - context.SIZ.YOsiz;
    this.componentsCount = context.SIZ.Csiz;
  }
}

/**
 * @param component - component to update the dimensions
 * @param siz - SIZ parameters
 */
function calculateComponentDimensions(component: Component, siz: SIZ): void {
  const { ceil } = Math;
  // Section B.2 Component mapping
  component.x0 = ceil(siz.XOsiz / component.XRsiz);
  component.x1 = ceil(siz.Xsiz / component.XRsiz);
  component.y0 = ceil(siz.YOsiz / component.YRsiz);
  component.y1 = ceil(siz.Ysiz / component.YRsiz);
  component.width = component.x1 - component.x0;
  component.height = component.y1 - component.y0;
}

/**
 * Section B.3 Division into tile and tile-components
 * @param context - global context to extract parameters
 * @param components - components to update
 */
function calculateTileGrids(context: Context, components: Component[]): void {
  const { ceil, min, max } = Math;
  const siz = context.SIZ;
  const tiles: ContextTile[] = [];
  const numXtiles = ceil((siz.Xsiz - siz.XTOsiz) / siz.XTsiz);
  const numYtiles = ceil((siz.Ysiz - siz.YTOsiz) / siz.YTsiz);
  for (let q = 0; q < numYtiles; q++) {
    for (let p = 0; p < numXtiles; p++) {
      const tx0 = max(siz.XTOsiz + p * siz.XTsiz, siz.XOsiz);
      const ty0 = max(siz.YTOsiz + q * siz.YTsiz, siz.YOsiz);
      const tx1 = min(siz.XTOsiz + (p + 1) * siz.XTsiz, siz.Xsiz);
      const ty1 = min(siz.YTOsiz + (q + 1) * siz.YTsiz, siz.Ysiz);
      tiles.push({
        tx0,
        ty0,
        tx1,
        ty1,
        width: tx1 - tx0,
        height: ty1 - ty0,
        components: [],
        codingStyleDefaultParameters: defaultCodingStyleParameters(),
      });
    }
  }
  context.tiles = tiles;

  const componentsCount = siz.Csiz;
  for (let i = 0, ii = componentsCount; i < ii; i++) {
    const component = components[i];
    for (let j = 0, jj = tiles.length; j < jj; j++) {
      const tile = tiles[j];
      const tcx0 = ceil(tile.tx0 / component.XRsiz);
      const tcy0 = ceil(tile.ty0 / component.YRsiz);
      const tcx1 = ceil(tile.tx1 / component.XRsiz);
      const tcy1 = ceil(tile.ty1 / component.YRsiz);
      tile.components[i] = {
        tcx0,
        tcy0,
        tcx1,
        tcy1,
        width: tcx1 - tcx0,
        height: tcy1 - tcy0,
        resolutions: [],
        subbands: [],
        quantizationParameters: defaultQuantizationParameters(),
        codingStyleParameters: defaultCodingStyleParameters(),
      };
    }
  }
}

/**
 * @param component - component to update
 * @param r - resolution
 * @returns - block dimensions
 */
function getBlocksDimensions(component: ContextTileComponent, r: number): BlockDimensions {
  const { min } = Math;
  const codOrCoc = component.codingStyleParameters;
  let PPx = 0;
  let PPy = 0;
  let xcb_ = 0;
  let ycb_ = 0;
  if (!codOrCoc.entropyCoderWithCustomPrecincts) {
    PPx = 15;
    PPy = 15;
  } else {
    PPx = codOrCoc.precinctsSizes[r].PPx;
    PPy = codOrCoc.precinctsSizes[r].PPy;
  }
  // calculate codeblock size as described in section B.7
  xcb_ = r > 0 ? min(codOrCoc.xcb, PPx - 1) : min(codOrCoc.xcb, PPx);
  ycb_ = r > 0 ? min(codOrCoc.ycb, PPy - 1) : min(codOrCoc.ycb, PPy);

  return {
    PPx,
    PPy,
    xcb_,
    ycb_,
  };
}

/**
 * @param resolution - resolution to update precincts
 * @param dimensions - block dimensions
 */
function buildPrecincts(resolution: Resolution, dimensions: BlockDimensions): void {
  const { ceil, floor } = Math;
  // Section B.6 Division resolution to precincts
  const precinctWidth = 1 << dimensions.PPx;
  const precinctHeight = 1 << dimensions.PPy;
  // Jasper introduces codeblock groups for mapping each subband codeblocks
  // to precincts. Precinct partition divides a resolution according to width
  // and height parameters. The subband that belongs to the resolution level
  // has a different size than the level, unless it is the zero resolution.

  // From Jasper documentation: jpeg2000.pdf, section K: Tier-2 coding:
  // The precinct partitioning for a particular subband is derived from a
  // partitioning of its parent LL band (i.e., the LL band at the next higher
  // resolution level)... The LL band associated with each resolution level is
  // divided into precincts... Each of the resulting precinct regions is then
  // mapped into its child subbands (if any) at the next lower resolution
  // level. This is accomplished by using the coordinate transformation
  // (u, v) = (ceil(x/2), ceil(y/2)) where (x, y) and (u, v) are the
  // coordinates of a point in the LL band and child subband, respectively.
  const isZeroRes = resolution.resLevel === 0;
  const precinctWidthInSubband = 1 << (dimensions.PPx + (isZeroRes ? 0 : -1));
  const precinctHeightInSubband = 1 << (dimensions.PPy + (isZeroRes ? 0 : -1));
  const numprecinctswide =
    resolution.trx1 > resolution.trx0
      ? ceil(resolution.trx1 / precinctWidth) - floor(resolution.trx0 / precinctWidth)
      : 0;
  const numprecinctshigh =
    resolution.try1 > resolution.try0
      ? ceil(resolution.try1 / precinctHeight) - floor(resolution.try0 / precinctHeight)
      : 0;
  const numprecincts = numprecinctswide * numprecinctshigh;

  resolution.precinctParameters = {
    precinctWidth,
    precinctHeight,
    numprecinctswide,
    numprecinctshigh,
    numprecincts,
    precinctWidthInSubband,
    precinctHeightInSubband,
  };
}
/**
 * @param subband - subband to update
 * @param dimensions - block dimensions to use
 */
function buildCodeblocks(subband: SubBand, dimensions: BlockDimensions): void {
  const { max, min, floor } = Math;
  // Section B.7 Division sub-band into code-blocks
  const xcb_ = dimensions.xcb_;
  const ycb_ = dimensions.ycb_;
  const codeblockWidth = 1 << xcb_;
  const codeblockHeight = 1 << ycb_;
  const cbx0 = subband.tbx0 >> xcb_;
  const cby0 = subband.tby0 >> ycb_;
  const cbx1 = (subband.tbx1 + codeblockWidth - 1) >> xcb_;
  const cby1 = (subband.tby1 + codeblockHeight - 1) >> ycb_;
  const precinctParameters = subband.resolution.precinctParameters;
  const codeblocks: CodeBlock[] = [];
  const precincts: Precinct[] = [];
  for (let j = cby0; j < cby1; j++) {
    for (let i = cbx0; i < cbx1; i++) {
      const codeblock: CodeBlock = {
        cbx: i,
        cby: j,
        tbx0: codeblockWidth * i,
        tby0: codeblockHeight * j,
        tbx1: codeblockWidth * (i + 1),
        tby1: codeblockHeight * (j + 1),
        tbx0_: 0,
        tby0_: 0,
        tbx1_: 0,
        tby1_: 0,
        precinctNumber: 0,
        subbandType: 'HH',
        Lblock: 0,
      };

      codeblock.tbx0_ = max(subband.tbx0, codeblock.tbx0);
      codeblock.tby0_ = max(subband.tby0, codeblock.tby0);
      codeblock.tbx1_ = min(subband.tbx1, codeblock.tbx1);
      codeblock.tby1_ = min(subband.tby1, codeblock.tby1);

      // Calculate precinct number for this codeblock, codeblock position
      // should be relative to its subband, use actual dimension and position
      // See comment about codeblock group width and height
      const pi = floor(
        (codeblock.tbx0_ - subband.tbx0) / precinctParameters.precinctWidthInSubband,
      );
      const pj = floor(
        (codeblock.tby0_ - subband.tby0) / precinctParameters.precinctHeightInSubband,
      );
      const precinctNumber = pi + pj * precinctParameters.numprecinctswide;

      codeblock.precinctNumber = precinctNumber;
      codeblock.subbandType = subband.type;
      codeblock.Lblock = 3;

      if (codeblock.tbx1_ <= codeblock.tbx0_ || codeblock.tby1_ <= codeblock.tby0_) {
        continue;
      }
      codeblocks.push(codeblock);
      // building precinct for the sub-band
      let precinct = precincts[precinctNumber];
      if (precinct !== undefined) {
        if (i < precinct.cbxMin) {
          precinct.cbxMin = i;
        } else if (i > precinct.cbxMax) {
          precinct.cbxMax = i;
        }
        if (j < precinct.cbyMin) {
          precinct.cbxMin = j;
        } else if (j > precinct.cbyMax) {
          precinct.cbyMax = j;
        }
      } else {
        precincts[precinctNumber] = precinct = {
          cbxMin: i,
          cbyMin: j,
          cbxMax: i,
          cbyMax: j,
        };
      }
      codeblock.precinct = precinct;
    }
  }
  subband.codeblockParameters = {
    codeblockWidth: xcb_,
    codeblockHeight: ycb_,
    numcodeblockwide: cbx1 - cbx0 + 1,
    numcodeblockhigh: cby1 - cby0 + 1,
  };
  subband.codeblocks = codeblocks;
  subband.precincts = precincts;
}
/**
 * @param resolution - the resolution of the packet
 * @param precinctNumber - the precinct number in the resolution
 * @param layerNumber - the associated layer
 * @returns - the built packet
 */
function createPacket(resolution: Resolution, precinctNumber: number, layerNumber: number): Packet {
  const precinctCodeblocks: CodeBlock[] = [];
  // Section B.10.8 Order of info in packet
  // sub-bands already ordered in 'LL', 'HL', 'LH', and 'HH' sequence
  for (const { codeblocks } of resolution.subbands) {
    for (const codeblock of codeblocks) {
      if (codeblock.precinctNumber !== precinctNumber) continue;
      precinctCodeblocks.push(codeblock);
    }
  }
  return {
    layerNumber,
    codeblocks: precinctCodeblocks,
  };
}

/**
 * Layer Resolution Component Position Iterator
 */
class LayerResolutionComponentPositionIterator implements ComponentPositionIterator {
  layersCount: number;
  componentsCount: number;
  maxDecompositionLevelsCount = 0;
  tile: ContextTile;
  l = 0;
  r = 0;
  i = 0;
  k = 0;
  /**
   * @param context - global context to pull data from
   */
  constructor(context: Context) {
    const siz = context.SIZ;
    const tileIndex = context.currentTile.index;
    this.tile = context.tiles[tileIndex];
    this.layersCount = this.tile.codingStyleDefaultParameters.layersCount;
    this.componentsCount = siz.Csiz;
    for (let q = 0; q < this.componentsCount; q++) {
      this.maxDecompositionLevelsCount = Math.max(
        this.maxDecompositionLevelsCount,
        this.tile.components[q].codingStyleParameters.decompositionLevelsCount,
      );
    }
  }

  /**
   * Section B.12.1.1 Layer-resolution-component-position
   * @returns - the next packet in the layer resolution
   */
  nextPacket(): Packet {
    for (; this.l < this.layersCount; this.l++) {
      for (; this.r <= this.maxDecompositionLevelsCount; this.r++) {
        for (; this.i < this.componentsCount; this.i++) {
          const component = this.tile.components[this.i];
          if (this.r > component.codingStyleParameters.decompositionLevelsCount) {
            continue;
          }

          const resolution = component.resolutions[this.r];
          const numprecincts = resolution.precinctParameters.numprecincts;
          for (; this.k < numprecincts; ) {
            const packet = createPacket(resolution, this.k, this.l);
            this.k++;
            return packet;
          }
          this.k = 0;
        }
        this.i = 0;
      }
      this.r = 0;
    }
    throw new Error('Out of packets');
  }
}

/**
 * Resolution Layer Component Position Iterator
 */
class ResolutionLayerComponentPositionIterator implements ComponentPositionIterator {
  layersCount: number;
  componentsCount: number;
  maxDecompositionLevelsCount: number;
  tile: ContextTile;
  l = 0;
  r = 0;
  i = 0;
  k = 0;
  /**
   * @param context - global context to pull data from
   */
  constructor(context: Context) {
    const siz = context.SIZ;
    const tileIndex = context.currentTile.index;
    this.tile = context.tiles[tileIndex];
    this.layersCount = this.tile.codingStyleDefaultParameters.layersCount;
    this.componentsCount = siz.Csiz;
    this.maxDecompositionLevelsCount = 0;
    for (let q = 0; q < this.componentsCount; q++) {
      this.maxDecompositionLevelsCount = Math.max(
        this.maxDecompositionLevelsCount,
        this.tile.components[q].codingStyleParameters.decompositionLevelsCount,
      );
    }
  }

  /**
   * Section B.12.1.2 Resolution-layer-component-position
   * @returns - the next packet in the resolution layer
   */
  nextPacket(): Packet {
    // Section B.12.1.2 Resolution-layer-component-position
    for (; this.r <= this.maxDecompositionLevelsCount; this.r++) {
      for (; this.l < this.layersCount; this.l++) {
        for (; this.i < this.componentsCount; this.i++) {
          const component = this.tile.components[this.i];
          if (this.r > component.codingStyleParameters.decompositionLevelsCount) {
            continue;
          }

          const resolution = component.resolutions[this.r];
          const numprecincts = resolution.precinctParameters.numprecincts;
          for (; this.k < numprecincts; ) {
            const packet = createPacket(resolution, this.k, this.l);
            this.k++;
            return packet;
          }
          this.k = 0;
        }
        this.i = 0;
      }
      this.l = 0;
    }
    throw new Error('Out of packets');
  }
}
/**
 * Resolution Position Component Layer Iterator
 */
class ResolutionPositionComponentLayerIterator implements ComponentPositionIterator {
  maxDecompositionLevelsCount = 0;
  layersCount: number;
  componentsCount: number;
  maxNumPrecinctsInLevel: Int32Array;
  tile: ContextTile;
  l = 0;
  r = 0;
  c = 0;
  p = 0;

  /**
   * @param context - global context to pull data from
   */
  constructor(context: Context) {
    const { max } = Math;
    const siz = context.SIZ;
    const tileIndex = context.currentTile.index;
    this.tile = context.tiles[tileIndex];
    this.layersCount = this.tile.codingStyleDefaultParameters.layersCount;
    const componentsCount = (this.componentsCount = siz.Csiz);
    for (let c = 0; c < componentsCount; c++) {
      const component = this.tile.components[c];
      this.maxDecompositionLevelsCount = max(
        this.maxDecompositionLevelsCount,
        component.codingStyleParameters.decompositionLevelsCount,
      );
    }
    this.maxNumPrecinctsInLevel = new Int32Array(this.maxDecompositionLevelsCount + 1);
    for (let r = 0; r <= this.maxDecompositionLevelsCount; ++r) {
      let maxNumPrecincts = 0;
      for (let c = 0; c < componentsCount; ++c) {
        const resolutions = this.tile.components[c].resolutions;
        if (r < resolutions.length) {
          maxNumPrecincts = max(maxNumPrecincts, resolutions[r].precinctParameters.numprecincts);
        }
      }
      this.maxNumPrecinctsInLevel[r] = maxNumPrecincts;
    }
  }

  /**
   * Section B.12.1.3 Resolution-position-component-layer
   * @returns - the next packet in the resolution layer
   */
  nextPacket(): Packet {
    for (; this.r <= this.maxDecompositionLevelsCount; this.r++) {
      for (; this.p < this.maxNumPrecinctsInLevel[this.r]; this.p++) {
        for (; this.c < this.componentsCount; this.c++) {
          const component = this.tile.components[this.c];
          if (this.r > component.codingStyleParameters.decompositionLevelsCount) {
            continue;
          }
          const resolution = component.resolutions[this.r];
          const numprecincts = resolution.precinctParameters.numprecincts;
          if (this.p >= numprecincts) continue;
          for (; this.l < this.layersCount; ) {
            const packet = createPacket(resolution, this.p, this.l);
            this.l++;
            return packet;
          }
          this.l = 0;
        }
        this.c = 0;
      }
      this.p = 0;
    }
    throw new Error('Out of packets');
  }
}

/**
 * Position Component Resolution Layer Iterator
 */
class PositionComponentResolutionLayerIterator {
  layersCount: number;
  componentsCount: number;
  precinctsSizes: PrecinctSizeInImageScale;
  precinctsIterationSizes: PrecinctSizeInImageScale;
  tile: ContextTile;
  l = 0;
  r = 0;
  c = 0;
  px = 0;
  py = 0;
  /** @param context - global context to pull data from */
  constructor(context: Context) {
    const siz = context.SIZ;
    const tileIndex = context.currentTile.index;
    this.tile = context.tiles[tileIndex];
    this.layersCount = this.tile.codingStyleDefaultParameters.layersCount;
    this.componentsCount = siz.Csiz;
    this.precinctsSizes = getPrecinctSizesInImageScale(this.tile);
    this.precinctsIterationSizes = this.precinctsSizes;
  }

  /**
   * Section B.12.1.4 Position-component-resolution-layer
   * @returns - the next packet in the position layer
   */
  nextPacket(): Packet {
    for (; this.py < this.precinctsIterationSizes.maxNumHigh; this.py++) {
      for (; this.px < this.precinctsIterationSizes.maxNumWide; this.px++) {
        for (; this.c < this.componentsCount; this.c++) {
          const component = this.tile.components[this.c];
          const decompositionLevelsCount = component.codingStyleParameters.decompositionLevelsCount;
          for (; this.r <= decompositionLevelsCount; this.r++) {
            const resolution = component.resolutions[this.r];
            const sizeInImageScale = this.precinctsSizes.components[this.c].resolutions[this.r];
            const k = getPrecinctIndexIfExist(
              this.px,
              this.py,
              sizeInImageScale,
              this.precinctsIterationSizes,
              resolution,
            );
            if (k === null) continue;
            for (; this.l < this.layersCount; ) {
              const packet = createPacket(resolution, k, this.l);
              this.l++;
              return packet;
            }
            this.l = 0;
          }
          this.r = 0;
        }
        this.c = 0;
      }
      this.px = 0;
    }
    throw new Error('Out of packets');
  }
}
/**
 * Component Position Resolution Layer Iterator
 */
class ComponentPositionResolutionLayerIterator {
  layersCount: number;
  componentsCount: number;
  precinctsSizes: PrecinctSizeInImageScale;
  tile: ContextTile;
  l = 0;
  r = 0;
  c = 0;
  px = 0;
  py = 0;
  /**
   * @param context - global context to pull data from
   */
  constructor(context: Context) {
    const siz = context.SIZ;
    const tileIndex = context.currentTile.index;
    this.tile = context.tiles[tileIndex];
    this.layersCount = this.tile.codingStyleDefaultParameters.layersCount;
    this.componentsCount = siz.Csiz;
    this.precinctsSizes = getPrecinctSizesInImageScale(this.tile);
  }

  /**
   * Section B.12.1.5 Component-position-resolution-layer
   * @returns - the next packet in the position layer
   */
  nextPacket(): Packet {
    for (; this.c < this.componentsCount; ++this.c) {
      const component = this.tile.components[this.c];
      const precinctsIterationSizes = this.precinctsSizes.components[this.c];
      const decompositionLevelsCount = component.codingStyleParameters.decompositionLevelsCount;
      for (; this.py < precinctsIterationSizes.maxNumHigh; this.py++) {
        for (; this.px < precinctsIterationSizes.maxNumWide; this.px++) {
          for (; this.r <= decompositionLevelsCount; this.r++) {
            const resolution = component.resolutions[this.r];
            const sizeInImageScale = precinctsIterationSizes.resolutions[this.r];
            const k = getPrecinctIndexIfExist(
              this.px,
              this.py,
              sizeInImageScale,
              this.precinctsSizes,
              resolution,
            );
            if (k === null) continue;
            for (; this.l < this.layersCount; ) {
              const packet = createPacket(resolution, k, this.l);
              this.l++;
              return packet;
            }
            this.l = 0;
          }
          this.r = 0;
        }
        this.px = 0;
      }
      this.py = 0;
    }
    throw new Error('Out of packets');
  }
}

/**
 * @param pxIndex - x position index
 * @param pyIndex - y position index
 * @param sizeInImageScale - size in image scale
 * @param precinctIterationSizes - precinct iteration sizes
 * @param resolution - resolution to check row size
 * @returns - precinct index if it exists
 */
function getPrecinctIndexIfExist(
  pxIndex: number,
  pyIndex: number,
  sizeInImageScale: SizePerResolution,
  precinctIterationSizes: PrecinctSizeInImageScale,
  resolution: Resolution,
): number | null {
  const posX = pxIndex * precinctIterationSizes.minWidth;
  const posY = pyIndex * precinctIterationSizes.minHeight;
  if (posX % sizeInImageScale.width !== 0 || posY % sizeInImageScale.height !== 0) {
    return null;
  }
  const startPrecinctRowIndex =
    (posY / sizeInImageScale.width) * resolution.precinctParameters.numprecinctswide;
  return posX / sizeInImageScale.height + startPrecinctRowIndex;
}

/**
 * @param tile - tile to get precinct sizes from
 * @returns - precinct sizes
 */
function getPrecinctSizesInImageScale(tile: ContextTile): PrecinctSizeInImageScale {
  const { min, max } = Math;
  const { components } = tile;
  const componentsCount = components.length;
  let minWidth = Number.MAX_VALUE;
  let minHeight = Number.MAX_VALUE;
  let maxNumWide = 0;
  let maxNumHigh = 0;
  const sizePerComponent: SizePerComponent[] = new Array(componentsCount);
  for (let c = 0; c < componentsCount; c++) {
    const component = components[c];
    const decompositionLevelsCount = component.codingStyleParameters.decompositionLevelsCount;
    const sizePerResolution: SizePerResolution[] = new Array(decompositionLevelsCount + 1);
    let minWidthCurrentComponent = Number.MAX_VALUE;
    let minHeightCurrentComponent = Number.MAX_VALUE;
    let maxNumWideCurrentComponent = 0;
    let maxNumHighCurrentComponent = 0;
    let scale = 1;
    for (let r = decompositionLevelsCount; r >= 0; --r) {
      const resolution = component.resolutions[r];
      const widthCurrentResolution = scale * resolution.precinctParameters.precinctWidth;
      const heightCurrentResolution = scale * resolution.precinctParameters.precinctHeight;
      minWidthCurrentComponent = min(minWidthCurrentComponent, widthCurrentResolution);
      minHeightCurrentComponent = min(minHeightCurrentComponent, heightCurrentResolution);
      maxNumWideCurrentComponent = max(
        maxNumWideCurrentComponent,
        resolution.precinctParameters.numprecinctswide,
      );
      maxNumHighCurrentComponent = max(
        maxNumHighCurrentComponent,
        resolution.precinctParameters.numprecinctshigh,
      );
      sizePerResolution[r] = {
        width: widthCurrentResolution,
        height: heightCurrentResolution,
      };
      scale <<= 1;
    }
    minWidth = min(minWidth, minWidthCurrentComponent);
    minHeight = min(minHeight, minHeightCurrentComponent);
    maxNumWide = max(maxNumWide, maxNumWideCurrentComponent);
    maxNumHigh = max(maxNumHigh, maxNumHighCurrentComponent);
    sizePerComponent[c] = {
      resolutions: sizePerResolution,
      minWidth: minWidthCurrentComponent,
      minHeight: minHeightCurrentComponent,
      maxNumWide: maxNumWideCurrentComponent,
      maxNumHigh: maxNumHighCurrentComponent,
    };
  }
  return {
    components: sizePerComponent,
    minWidth,
    minHeight,
    maxNumWide,
    maxNumHigh,
  };
}

/**
 * Build packets from our global context tile data
 * @param context - context to build packets from
 */
function buildPackets(context: Context): void {
  const { ceil } = Math;
  const siz = context.SIZ;
  const tileIndex = context.currentTile.index;
  const tile = context.tiles[tileIndex];
  const componentsCount = siz.Csiz;
  // Creating resolutions and sub-bands for each component
  for (let c = 0; c < componentsCount; c++) {
    const component = tile.components[c];
    const decompositionLevelsCount = component.codingStyleParameters.decompositionLevelsCount;
    // Section B.5 Resolution levels and sub-bands
    const resolutions: Resolution[] = [];
    const subbands: SubBand[] = [];
    for (let r = 0; r <= decompositionLevelsCount; r++) {
      const blocksDimensions = getBlocksDimensions(component, r);
      const scale = 1 << (decompositionLevelsCount - r);
      const resolution: Resolution = {
        trx0: ceil(component.tcx0 / scale),
        try0: ceil(component.tcy0 / scale),
        trx1: ceil(component.tcx1 / scale),
        try1: ceil(component.tcy1 / scale),
        resLevel: r,
        precinctParameters: defaultPrecinctParameters(),
        subbands: [],
      };
      buildPrecincts(resolution, blocksDimensions);
      resolutions.push(resolution);

      if (r === 0) {
        // one sub-band (LL) with last decomposition
        const subband: SubBand = {
          type: 'LL',
          tbx0: ceil(component.tcx0 / scale),
          tby0: ceil(component.tcy0 / scale),
          tbx1: ceil(component.tcx1 / scale),
          tby1: ceil(component.tcy1 / scale),
          resolution,
          codeblocks: [],
          precincts: [],
          codeblockParameters: defaultCodeBlockParams(),
        };
        buildCodeblocks(subband, blocksDimensions);
        subbands.push(subband);
        resolution.subbands = [subband];
      } else {
        const bscale = 1 << (decompositionLevelsCount - r + 1);
        const resolutionSubbands = [];
        // three sub-bands (HL, LH and HH) with rest of decompositions
        const subbandHL: SubBand = {
          type: 'HL',
          tbx0: ceil(component.tcx0 / bscale - 0.5),
          tby0: ceil(component.tcy0 / bscale),
          tbx1: ceil(component.tcx1 / bscale - 0.5),
          tby1: ceil(component.tcy1 / bscale),
          resolution,
          codeblocks: [],
          precincts: [],
          codeblockParameters: defaultCodeBlockParams(),
        };
        buildCodeblocks(subbandHL, blocksDimensions);
        subbands.push(subbandHL);
        resolutionSubbands.push(subbandHL);

        const subbandLH: SubBand = {
          type: 'LH',
          tbx0: ceil(component.tcx0 / bscale),
          tby0: ceil(component.tcy0 / bscale - 0.5),
          tbx1: ceil(component.tcx1 / bscale),
          tby1: ceil(component.tcy1 / bscale - 0.5),
          resolution,
          codeblocks: [],
          precincts: [],
          codeblockParameters: defaultCodeBlockParams(),
        };
        buildCodeblocks(subbandLH, blocksDimensions);
        subbands.push(subbandLH);
        resolutionSubbands.push(subbandLH);

        const subbandHH: SubBand = {
          type: 'HH',
          tbx0: ceil(component.tcx0 / bscale - 0.5),
          tby0: ceil(component.tcy0 / bscale - 0.5),
          tbx1: ceil(component.tcx1 / bscale - 0.5),
          tby1: ceil(component.tcy1 / bscale - 0.5),
          resolution,
          codeblocks: [],
          precincts: [],
          codeblockParameters: defaultCodeBlockParams(),
        };
        buildCodeblocks(subbandHH, blocksDimensions);
        subbands.push(subbandHH);
        resolutionSubbands.push(subbandHH);

        resolution.subbands = resolutionSubbands;
      }
    }
    component.resolutions = resolutions;
    component.subbands = subbands;
  }
  // Generate the packets sequence
  const progressionOrder = tile.codingStyleDefaultParameters.progressionOrder;
  switch (progressionOrder) {
    case 0:
      tile.packetsIterator = new LayerResolutionComponentPositionIterator(context);
      break;
    case 1:
      tile.packetsIterator = new ResolutionLayerComponentPositionIterator(context);
      break;
    case 2:
      tile.packetsIterator = new ResolutionPositionComponentLayerIterator(context);
      break;
    case 3:
      tile.packetsIterator = new PositionComponentResolutionLayerIterator(context);
      break;
    case 4:
      tile.packetsIterator = new ComponentPositionResolutionLayerIterator(context);
      break;
    default:
      throw new Error(`Unsupported progression order ${progressionOrder}`);
  }
}
/**
 * Parse tile packets
 * @param context - global context to pull data from
 * @param data - the raw data to be parsed
 * @param offset - the offset in the raw data
 * @param dataLength - the length of the raw data
 * @returns - the number of parsed packets
 */
function parseTilePackets(
  context: Context,
  data: Reader,
  offset: number,
  dataLength: number,
): number {
  let position = 0,
    buffer = 0,
    bufferSize = 0,
    skipNextBit = false;
  /**
   * Reads the specified number of bits
   * @param count - the number of bits to read
   * @returns - the buffer size
   */
  function readBits(count: number): number {
    while (bufferSize < count) {
      const b = data.getUint8(offset + position);
      position++;
      if (skipNextBit) {
        buffer = (buffer << 7) | b;
        bufferSize += 7;
        skipNextBit = false;
      } else {
        buffer = (buffer << 8) | b;
        bufferSize += 8;
      }
      if (b === 0xff) skipNextBit = true;
    }
    bufferSize -= count;
    return (buffer >>> bufferSize) & ((1 << count) - 1);
  }
  /**
   * Skips the marker if it is equal to the specified value
   * @param value - the value to skip
   * @returns - true if the marker was skipped
   */
  function skipMarkerIfEqual(value: number): boolean {
    if (
      data.getUint8(offset + position - 1) === 0xff &&
      data.getUint8(offset + position) === value
    ) {
      skipBytes(1);
      return true;
    } else if (
      data.getUint8(offset + position) === 0xff &&
      data.getUint8(offset + position + 1) === value
    ) {
      skipBytes(2);
      return true;
    }
    return false;
  }
  /**
   * Skips the specified number of bytes
   * @param count - the number of bytes to skip
   */
  function skipBytes(count: number): void {
    position += count;
  }
  /**
   * Aligns the buffer to the next byte
   */
  function alignToByte(): void {
    bufferSize = 0;
    if (skipNextBit) {
      position++;
      skipNextBit = false;
    }
  }
  /**
   * Reads the number of coding passes
   * @returns - the number of coding passes
   */
  function readCodingpasses(): number {
    if (readBits(1) === 0) return 1;
    if (readBits(1) === 0) return 2;
    let value = readBits(2);
    if (value < 3) return value + 3;
    value = readBits(5);
    if (value < 31) return value + 6;
    value = readBits(7);
    return value + 37;
  }
  const tileIndex = context.currentTile.index;
  const tile = context.tiles[tileIndex];
  const { sopMarkerUsed, ephMarkerUsed } = context.COD;
  const { packetsIterator } = tile;
  if (packetsIterator === undefined) throw new Error('Packets iterator not initialized');
  while (position < dataLength) {
    alignToByte();
    // Skip also marker segment length and packet sequence ID
    if (sopMarkerUsed && skipMarkerIfEqual(0x91)) skipBytes(4);
    const { layerNumber, codeblocks } = packetsIterator.nextPacket();
    if (readBits(1) === 0) continue;
    const queue: QueueItem[] = [];
    for (const codeblock of codeblocks) {
      const { precinct, cbx, cby, included } = codeblock;
      if (precinct === undefined) throw new Error('Precinct not defined');
      const { cbxMin, cbyMin, cbxMax, cbyMax } = precinct;
      const codeblockColumn = cbx - cbxMin;
      const codeblockRow = cby - cbyMin;
      let codeblockIncluded = false;
      let firstTimeInclusion = false;
      if (included !== undefined) {
        codeblockIncluded = !(readBits(1) === 0);
      } else {
        // reading inclusion tree
        if (precinct.inclusionTree === undefined) {
          // building inclusion and zero bit-planes trees
          const width = cbxMax - cbxMin + 1;
          const height = cbyMax - cbyMin + 1;
          precinct.inclusionTree = new InclusionTree(width, height, layerNumber);
          precinct.zeroBitPlanesTree = new TagTree(width, height);
        }

        const inclusionTree = precinct.inclusionTree;
        if (inclusionTree.reset(codeblockColumn, codeblockRow, layerNumber)) {
          while (true) {
            if (readBits(1) !== 0) {
              if (!inclusionTree.nextLevel()) {
                codeblock.included = true;
                codeblockIncluded = firstTimeInclusion = true;
                break;
              }
            } else {
              inclusionTree.incrementValue(layerNumber);
              break;
            }
          }
        }
      }
      // codedDataLength A 40 80 94
      // HERHERHEHRHERHERHEHRHERHHREHRHRH A
      // codedDataLength A 6 82 94
      // codeblocks [
      //   undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined,
      //   undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined,
      //   undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined,
      //   undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined
      // ]

      // codedDataLength B 40 80 94 6
      // HERHERHEHRHERHERHEHRHERHHREHRHRH B
      // codedDataLength B 6 82 94 6
      // codeblocks [
      //   true, true, true, true, true, true, true, true, true, true, true, true, true, true,
      //   true, true, true, true, true, true, true, true, true, true, true, true, true, true,
      //   true, true, true, true
      // ]
      if (!codeblockIncluded) continue;
      if (firstTimeInclusion) {
        const { zeroBitPlanesTree } = precinct;
        if (zeroBitPlanesTree === undefined) throw new Error('Zero bit-planes tree not defined');
        zeroBitPlanesTree.reset(codeblockColumn, codeblockRow);
        while (true) {
          if (readBits(1) !== 0) {
            if (!zeroBitPlanesTree.nextLevel()) break;
          } else {
            zeroBitPlanesTree.incrementValue();
          }
        }
        codeblock.zeroBitPlanes = zeroBitPlanesTree.value;
      }
      const codingpasses = readCodingpasses();
      while (readBits(1) !== 0) {
        codeblock.Lblock++;
      }
      const codingpassesLog2 = log2(codingpasses);
      // rounding down log2
      const bits =
        (codingpasses < 1 << codingpassesLog2 ? codingpassesLog2 - 1 : codingpassesLog2) +
        codeblock.Lblock;
      const codedDataLength = readBits(bits);
      queue.push({
        codeblock,
        codingpasses,
        dataLength: codedDataLength,
      });
    }
    alignToByte();
    if (ephMarkerUsed) skipMarkerIfEqual(0x92);
    while (queue.length > 0) {
      const packetItem = queue.shift();
      if (packetItem === undefined) break;
      const { codeblock } = packetItem;
      if (codeblock.data === undefined) codeblock.data = [];
      codeblock.data.push({
        data,
        start: offset + position,
        end: offset + position + packetItem.dataLength,
        codingpasses: packetItem.codingpasses,
      });
      position += packetItem.dataLength;
    }
  }
  return position;
}

/**
 * Copies the subband coefficients
 * @param coefficients - array of coefficients
 * @param levelWidth - width of the level
 * @param subband - subband to copy from
 * @param delta - quantization delta
 * @param mb - block size
 * @param reversible - true if the transform is reversible
 * @param segmentationSymbolUsed - true if the segmentation symbol is used
 */
function copyCoefficients(
  coefficients: Float32Array,
  levelWidth: number,
  subband: SubBand,
  delta: number,
  mb: number,
  reversible: boolean,
  segmentationSymbolUsed: boolean,
) {
  const x0 = subband.tbx0;
  const y0 = subband.tby0;
  const width = subband.tbx1 - subband.tbx0;
  const codeblocks = subband.codeblocks;
  const right = subband.type.charAt(0) === 'H' ? 1 : 0;
  const bottom = subband.type.charAt(1) === 'H' ? levelWidth : 0;

  for (let i = 0, ii = codeblocks.length; i < ii; ++i) {
    const codeblock = codeblocks[i];
    const blockWidth = codeblock.tbx1_ - codeblock.tbx0_;
    const blockHeight = codeblock.tby1_ - codeblock.tby0_;
    if (blockWidth === 0 || blockHeight === 0) continue;
    if (codeblock.data === undefined) continue;

    const bitModel = new BitModel(
      blockWidth,
      blockHeight,
      codeblock.subbandType,
      codeblock.zeroBitPlanes ?? 0,
      mb,
    );
    let currentCodingpassType = 2; // first bit plane starts from cleanup

    // collect data
    const { data } = codeblock;
    let totalLength = 0;
    let codingpasses = 0;
    for (const dataItem of data) {
      totalLength += dataItem.end - dataItem.start;
      codingpasses += dataItem.codingpasses;
    }
    const encodedData = new Uint8Array(totalLength);
    let position = 0;
    for (const dataItem of data) {
      const chunk = dataItem.data.slice(dataItem.start, dataItem.end);
      const uint8Chunk = new Uint8Array(chunk.buffer, chunk.byteOffset, chunk.byteLength);
      encodedData.set(uint8Chunk, position);
      position += uint8Chunk.length;
    }
    // decoding the item
    const decoder = new ArithmeticDecoder(encodedData, 0, totalLength);
    bitModel.setDecoder(decoder);

    for (let j = 0; j < codingpasses; j++) {
      switch (currentCodingpassType) {
        case 0:
          bitModel.runSignificancePropagationPass();
          break;
        case 1:
          bitModel.runMagnitudeRefinementPass();
          break;
        case 2:
          bitModel.runCleanupPass();
          if (segmentationSymbolUsed) {
            bitModel.checkSegmentationSymbol();
          }
          break;
      }
      currentCodingpassType = (currentCodingpassType + 1) % 3;
    }

    let offset = codeblock.tbx0_ - x0 + (codeblock.tby0_ - y0) * width;
    const sign = bitModel.coefficentsSign;
    const magnitude = bitModel.coefficentsMagnitude;
    const bitsDecoded = bitModel.bitsDecoded;
    const magnitudeCorrection = reversible ? 0 : 0.5;
    position = 0;
    // Do the interleaving of Section F.3.3 here, so we do not need
    // to copy later. LL level is not interleaved, just copied.
    const interleave = subband.type !== 'LL';
    for (let j = 0; j < blockHeight; j++) {
      const row = (offset / width) | 0; // row in the non-interleaved subband
      const levelOffset = 2 * row * (levelWidth - width) + right + bottom;
      for (let k = 0; k < blockWidth; k++) {
        let n = magnitude[position];
        if (n !== 0) {
          n = (n + magnitudeCorrection) * delta;
          if (sign[position] !== 0) {
            n = -n;
          }
          const nb = bitsDecoded[position];
          const pos = interleave ? levelOffset + (offset << 1) : offset;
          if (reversible && nb >= mb) {
            coefficients[pos] = n;
          } else {
            coefficients[pos] = n * (1 << (mb - nb));
          }
        }
        offset++;
        position++;
      }
      offset += width - blockWidth;
    }
  }
}

/**
 * Transforms a tile to a 2D array of coefficients
 * @param context - global context to pull data from
 * @param tile - tile to transform
 * @param c - component index
 * @returns - the transformed tile
 */
function transformTile(context: Context, tile: ContextTile, c: number): TileTransform {
  const component = tile.components[c];
  const { codingStyleParameters, quantizationParameters } = component;
  const { decompositionLevelsCount, segmentationSymbolUsed } = codingStyleParameters;
  const { spqcds, scalarExpounded, guardBits } = quantizationParameters;
  const precision = context.components[c].precision;

  const reversible = codingStyleParameters.reversibleTransformation;
  const transform = reversible ? new ReversibleTransform() : new IrreversibleTransform();

  const subbandCoefficients: Level[] = [];
  let b = 0;
  for (let i = 0; i <= decompositionLevelsCount; i++) {
    const resolution = component.resolutions[i];

    const width = resolution.trx1 - resolution.trx0;
    const height = resolution.try1 - resolution.try0;
    // Allocate space for the whole sublevel.
    const coefficients = new Float32Array(width * height);

    for (let j = 0, jj = resolution.subbands.length; j < jj; j++) {
      let mu, epsilon;
      if (!scalarExpounded) {
        // formula E-5
        mu = spqcds[0].mu;
        epsilon = spqcds[0].epsilon + (i > 0 ? 1 - i : 0);
      } else {
        mu = spqcds[b].mu;
        epsilon = spqcds[b].epsilon;
        b++;
      }

      const subband = resolution.subbands[j];
      const gainLog2 = SubbandsGainLog2[subband.type];

      // calculate quantization coefficient (Section E.1.1.1)
      const delta = reversible ? 1 : 2 ** (precision + gainLog2 - epsilon) * (1 + mu / 2048);
      const mb = guardBits + epsilon - 1;

      // In the first resolution level, copyCoefficients will fill the
      // whole array with coefficients. In the succeeding passes,
      // copyCoefficients will consecutively fill in the values that belong
      // to the interleaved positions of the HL, LH, and HH coefficients.
      // The LL coefficients will then be interleaved in Transform.iterate().
      copyCoefficients(coefficients, width, subband, delta, mb, reversible, segmentationSymbolUsed);
    }
    subbandCoefficients.push({
      width,
      height,
      items: coefficients,
    });
  }

  const result = transform.calculate(subbandCoefficients, component.tcx0, component.tcy0);
  return {
    left: component.tcx0,
    top: component.tcy0,
    width: result.width,
    height: result.height,
    items: result.items,
  };
}
/**
 * Transforms a tile to a 2D array of coefficients
 * @param context - global context to pull data from
 * @returns - the transformed tiles
 */
function transformComponents(context: Context): ComponentTile[] {
  const {
    tiles,
    components,
    SIZ: { Csiz: componentsCount },
  } = context;
  const resultImages = [];
  for (let i = 0, ii = tiles.length; i < ii; i++) {
    const tile = tiles[i];
    const transformedTiles: TileTransform[] = [];
    for (let c = 0; c < componentsCount; c++) {
      transformedTiles[c] = transformTile(context, tile, c);
    }
    const tile0 = transformedTiles[0];
    const out = new Uint8ClampedArray(tile0.items.length * componentsCount);
    const result = {
      left: tile0.left,
      top: tile0.top,
      width: tile0.width,
      height: tile0.height,
      items: out,
    };

    // Section G.2.2 Inverse multi component transform
    let shift,
      offset,
      pos = 0,
      j,
      jj,
      y0,
      y1,
      y2;
    if (tile.codingStyleDefaultParameters.multipleComponentTransform !== 0) {
      const fourComponents = componentsCount === 4;
      const y0items = transformedTiles[0].items;
      const y1items = transformedTiles[1].items;
      const y2items = transformedTiles[2].items;
      const y3items = fourComponents ? transformedTiles[3].items : null;

      // HACK: The multiple component transform formulas below assume that
      // all components have the same precision. With this in mind, we
      // compute shift and offset only once.
      shift = components[0].precision - 8;
      offset = (128 << shift) + 0.5;

      const component0 = tile.components[0];
      const alpha01 = componentsCount - 3;
      jj = y0items.length;
      if (component0.codingStyleParameters.reversibleTransformation) {
        // inverse irreversible multiple component transform
        for (j = 0; j < jj; j++, pos += alpha01) {
          y0 = y0items[j] + offset;
          y1 = y1items[j];
          y2 = y2items[j];
          out[pos++] = (y0 + 1.402 * y2) >> shift;
          out[pos++] = (y0 - 0.34413 * y1 - 0.71414 * y2) >> shift;
          out[pos++] = (y0 + 1.772 * y1) >> shift;
        }
      } else {
        // inverse reversible multiple component transform
        for (j = 0; j < jj; j++, pos += alpha01) {
          y0 = y0items[j] + offset;
          y1 = y1items[j];
          y2 = y2items[j];
          const g = y0 - ((y2 + y1) >> 2);

          out[pos++] = (g + y2) >> shift;
          out[pos++] = g >> shift;
          out[pos++] = (g + y1) >> shift;
        }
      }
      if (fourComponents && y3items !== null) {
        for (j = 0, pos = 3; j < jj; j++, pos += 4) {
          out[pos] = (y3items[j] + offset) >> shift;
        }
      }
    } else {
      // no multi-component transform
      for (let c = 0; c < componentsCount; c++) {
        const items = transformedTiles[c].items;
        shift = components[c].precision - 8;
        offset = (128 << shift) + 0.5;
        for (pos = c, j = 0, jj = items.length; j < jj; j++) {
          out[pos] = (items[j] + offset) >> shift;
          pos += componentsCount;
        }
      }
    }
    resultImages.push(result);
  }
  return resultImages;
}
/**
 * Initializes a tile
 * @param context - global context to pull data from
 * @param tileIndex - tile index
 */
function initializeTile(context: Context, tileIndex: number): void {
  const {
    SIZ: { Csiz: componentsCount },
    currentTile,
    tiles,
  } = context;
  const tile = tiles[tileIndex];
  for (let c = 0; c < componentsCount; c++) {
    const component = tile.components[c];
    const qcdOrQcc = currentTile.QCC[c] !== undefined ? currentTile.QCC[c] : currentTile.QCD;
    component.quantizationParameters = qcdOrQcc;
    const codOrCoc = currentTile.COC[c] !== undefined ? currentTile.COC[c] : currentTile.COD;
    component.codingStyleParameters = codOrCoc;
  }
  tile.codingStyleDefaultParameters = currentTile.COD;
}

/**
 * Section B.10.2 Tag trees
 */
class TagTree {
  currentLevel = 0;
  levels: Level[] = [];
  value: undefined | number;

  /**
   * @param width - width of the tree
   * @param height - height of the tree
   */
  constructor(width: number, height: number) {
    const { max, ceil } = Math;
    const levelsLength = log2(max(width, height)) + 1;
    for (let i = 0; i < levelsLength; i++) {
      this.levels.push({
        width,
        height,
        items: [],
      });
      width = ceil(width / 2);
      height = ceil(height / 2);
    }
  }
  /**
   * Resets the tree
   * @param i - i coordinate
   * @param j - j coordinate
   */
  reset(i: number, j: number): void {
    let currentLevel = 0;
    let value = 0;
    while (currentLevel < this.levels.length) {
      const level = this.levels[currentLevel];
      const index = i + j * level.width;
      if (level.items[index] !== undefined) {
        value = level.items[index];
        break;
      }
      level.index = index;
      i >>= 1;
      j >>= 1;
      currentLevel++;
    }
    currentLevel--;
    const level = this.levels[currentLevel];
    level.items[level.index ?? 0] = value;
    this.currentLevel = currentLevel;
    this.value = undefined;
  }
  /**
   * Increment value of the current level
   */
  incrementValue(): void {
    const level = this.levels[this.currentLevel];
    level.items[level.index ?? 0]++;
  }
  /**
   * Moves the cursor to the next level
   * @returns - true if there is a next level
   */
  nextLevel(): boolean {
    let currentLevel = this.currentLevel;
    let level = this.levels[currentLevel];
    const value = level.items[level.index ?? 0];
    currentLevel--;
    if (currentLevel < 0) {
      this.value = value;
      return false;
    }

    this.currentLevel = currentLevel;
    level = this.levels[currentLevel];
    level.items[level.index ?? 0] = value;
    return true;
  }
}

/**
 * Section B.10.3 Inclusion trees
 */
class InclusionTree {
  currentLevel = 0;
  levels: Level[] = [];
  /**
   * @param width - width of the tree
   * @param height - height of the tree
   * @param defaultValue - default value
   */
  constructor(width: number, height: number, defaultValue: number) {
    const { max, ceil } = Math;
    const levelsLength = log2(max(width, height)) + 1;
    for (let i = 0; i < levelsLength; i++) {
      const items = new Uint8Array(width * height);
      for (let j = 0, jj = items.length; j < jj; j++) items[j] = defaultValue;
      this.levels.push({
        width,
        height,
        items,
      });

      width = ceil(width / 2);
      height = ceil(height / 2);
    }
  }

  /**
   * Resets the tree
   * @param i - i coordinate
   * @param j - j coordinate
   * @param stopValue - stop value
   * @returns - true if there is a next level
   */
  reset(i: number, j: number, stopValue: number): boolean {
    let currentLevel = 0;
    while (currentLevel < this.levels.length) {
      const level = this.levels[currentLevel];
      const index = i + j * level.width;
      level.index = index;
      const value = level.items[index];

      if (value === 0xff) break;

      if (value > stopValue) {
        this.currentLevel = currentLevel;
        // already know about this one, propagating the value to top levels
        this.propagateValues();
        return false;
      }

      i >>= 1;
      j >>= 1;
      currentLevel++;
    }
    this.currentLevel = currentLevel - 1;
    return true;
  }
  /**
   * Increments the value of the current level
   * @param stopValue - stop value
   */
  incrementValue(stopValue: number): void {
    const level = this.levels[this.currentLevel];
    level.items[level.index ?? 0] = stopValue + 1;
    this.propagateValues();
  }
  /** Propagates the value to the top levels */
  propagateValues(): void {
    let levelIndex = this.currentLevel;
    let level = this.levels[levelIndex];
    const currentValue = level.items[level.index ?? 0];
    while (--levelIndex >= 0) {
      level = this.levels[levelIndex];
      level.items[level.index ?? 0] = currentValue;
    }
  }
  /**
   * Moves the cursor to the next level
   * @returns - true if there is a next level
   */
  nextLevel(): boolean {
    let currentLevel = this.currentLevel;
    let level = this.levels[currentLevel];
    const value = level.items[level.index ?? 0];
    level.items[level.index ?? 0] = 0xff;
    currentLevel--;
    if (currentLevel < 0) return false;

    this.currentLevel = currentLevel;
    level = this.levels[currentLevel];
    level.items[level.index ?? 0] = value;
    return true;
  }
}

const UNIFORM_CONTEXT = 17;
const RUNLENGTH_CONTEXT = 18;

/**
 * Section D. Coefficient bit modeling
 */
class BitModel {
  neighborsSignificance: Uint8Array<ArrayBuffer>;
  coefficentsSign: Uint8Array<ArrayBuffer>;
  contextLabelTable: Uint8Array<ArrayBuffer>;
  coefficentsMagnitude: Uint8Array<ArrayBuffer> | Uint16Array | Uint32Array;
  processingFlags: Uint8Array<ArrayBuffer>;
  bitsDecoded: Uint8Array<ArrayBuffer>;
  contexts = new Int8Array(0);
  decoder!: ArithmeticDecoder;

  /**
   * @param width - width
   * @param height - height
   * @param subband - subband
   * @param zeroBitPlanes - zero bit planes size
   * @param mb - block size
   */
  constructor(
    readonly width: number,
    readonly height: number,
    subband: SubBandType,
    zeroBitPlanes: number,
    mb: number,
  ) {
    let contextLabelTable;
    // Table D-1
    // The index is binary presentation: 0dddvvhh, ddd - sum of Di (0..4),
    // vv - sum of Vi (0..2), and hh - sum of Hi (0..2)
    if (subband === 'HH') {
      contextLabelTable = new Uint8Array([
        0, 1, 2, 0, 1, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 3, 4, 5, 0, 4, 5, 5, 0, 5, 5, 5, 0, 0, 0, 0,
        0, 6, 7, 7, 0, 7, 7, 7, 0, 7, 7, 7, 0, 0, 0, 0, 0, 8, 8, 8, 0, 8, 8, 8, 0, 8, 8, 8, 0, 0, 0,
        0, 0, 8, 8, 8, 0, 8, 8, 8, 0, 8, 8, 8,
      ]);
    } else if (subband === 'HL') {
      contextLabelTable = new Uint8Array([
        0, 3, 4, 0, 5, 7, 7, 0, 8, 8, 8, 0, 0, 0, 0, 0, 1, 3, 4, 0, 6, 7, 7, 0, 8, 8, 8, 0, 0, 0, 0,
        0, 2, 3, 4, 0, 6, 7, 7, 0, 8, 8, 8, 0, 0, 0, 0, 0, 2, 3, 4, 0, 6, 7, 7, 0, 8, 8, 8, 0, 0, 0,
        0, 0, 2, 3, 4, 0, 6, 7, 7, 0, 8, 8, 8,
      ]);
    } else {
      contextLabelTable = new Uint8Array([
        0, 5, 8, 0, 3, 7, 8, 0, 4, 7, 8, 0, 0, 0, 0, 0, 1, 6, 8, 0, 3, 7, 8, 0, 4, 7, 8, 0, 0, 0, 0,
        0, 2, 6, 8, 0, 3, 7, 8, 0, 4, 7, 8, 0, 0, 0, 0, 0, 2, 6, 8, 0, 3, 7, 8, 0, 4, 7, 8, 0, 0, 0,
        0, 0, 2, 6, 8, 0, 3, 7, 8, 0, 4, 7, 8,
      ]);
    }
    this.contextLabelTable = contextLabelTable;

    const coefficientCount = width * height;

    // coefficients outside the encoding region treated as insignificant
    // add border state cells for significanceState
    this.neighborsSignificance = new Uint8Array(coefficientCount);
    this.coefficentsSign = new Uint8Array(coefficientCount);
    let coefficentsMagnitude;
    if (mb > 14) {
      coefficentsMagnitude = new Uint32Array(coefficientCount);
    } else if (mb > 6) {
      coefficentsMagnitude = new Uint16Array(coefficientCount);
    } else {
      coefficentsMagnitude = new Uint8Array(coefficientCount);
    }
    this.coefficentsMagnitude = coefficentsMagnitude;
    this.processingFlags = new Uint8Array(coefficientCount);

    const bitsDecoded = new Uint8Array(coefficientCount);
    if (zeroBitPlanes !== 0) {
      for (let i = 0; i < coefficientCount; i++) {
        bitsDecoded[i] = zeroBitPlanes;
      }
    }
    this.bitsDecoded = bitsDecoded;

    this.reset();
  }

  /**
   * Set decoder
   * @param decoder - decoder to use
   */
  setDecoder(decoder: ArithmeticDecoder): void {
    this.decoder = decoder;
  }

  /**
   * Reset the tree
   */
  reset(): void {
    // We have 17 contexts that are accessed via context labels,
    // plus the uniform and runlength context.
    this.contexts = new Int8Array(19);

    // Contexts are packed into 1 byte:
    // highest 7 bits carry the index, lowest bit carries mps
    this.contexts[0] = (4 << 1) | 0;
    this.contexts[UNIFORM_CONTEXT] = (46 << 1) | 0;
    this.contexts[RUNLENGTH_CONTEXT] = (3 << 1) | 0;
  }
  /**
   * Set neighbors significance
   * @param row - row
   * @param column - column
   * @param index - index to set
   */
  setNeighborsSignificance(row: number, column: number, index: number): void {
    const { width, height, neighborsSignificance } = this;
    const left = column > 0;
    const right = column + 1 < width;
    let i;

    if (row > 0) {
      i = index - width;
      if (left) {
        neighborsSignificance[i - 1] += 0x10;
      }
      if (right) {
        neighborsSignificance[i + 1] += 0x10;
      }
      neighborsSignificance[i] += 0x04;
    }

    if (row + 1 < height) {
      i = index + width;
      if (left) neighborsSignificance[i - 1] += 0x10;
      if (right) neighborsSignificance[i + 1] += 0x10;
      neighborsSignificance[i] += 0x04;
    }

    if (left) neighborsSignificance[index - 1] += 0x01;
    if (right) neighborsSignificance[index + 1] += 0x01;
    neighborsSignificance[index] |= 0x80;
  }
  /** Run significance propagation */
  runSignificancePropagationPass(): void {
    const {
      decoder,
      width,
      height,
      processingFlags,
      coefficentsMagnitude,
      coefficentsSign,
      neighborsSignificance,
      contextLabelTable: labels,
      contexts,
      bitsDecoded,
    } = this;
    const processedInverseMask = ~1;
    const processedMask = 1;
    const firstMagnitudeBitMask = 2;

    for (let i0 = 0; i0 < height; i0 += 4) {
      for (let j = 0; j < width; j++) {
        let index = i0 * width + j;
        for (let i1 = 0; i1 < 4; i1++, index += width) {
          const i = i0 + i1;
          if (i >= height) {
            break;
          }
          // clear processed flag first
          processingFlags[index] &= processedInverseMask;

          if (coefficentsMagnitude[index] !== 0 || neighborsSignificance[index] === 0) {
            continue;
          }

          const contextLabel = labels[neighborsSignificance[index]];
          const decision = decoder.readBit(contexts, contextLabel);
          if (decision !== 0) {
            const sign = this.decodeSignBit(i, j, index);
            coefficentsSign[index] = sign;
            coefficentsMagnitude[index] = 1;
            this.setNeighborsSignificance(i, j, index);
            processingFlags[index] |= firstMagnitudeBitMask;
          }
          bitsDecoded[index]++;
          processingFlags[index] |= processedMask;
        }
      }
    }
  }
  /**
   * Decode sign bit
   * @param row - row size
   * @param column - column size
   * @param index - index to decode
   * @returns - sign
   */
  decodeSignBit(row: number, column: number, index: number): number {
    const { decoder, width, height, coefficentsMagnitude, coefficentsSign, contexts } = this;
    let contribution: number,
      sign0: number,
      sign1: number,
      significance1: boolean,
      contextLabel: number,
      decoded: number;

    // calculate horizontal contribution
    significance1 = column > 0 && coefficentsMagnitude[index - 1] !== 0;
    if (column + 1 < width && coefficentsMagnitude[index + 1] !== 0) {
      sign1 = coefficentsSign[index + 1];
      if (significance1) {
        sign0 = coefficentsSign[index - 1];
        contribution = 1 - sign1 - sign0;
      } else {
        contribution = 1 - sign1 - sign1;
      }
    } else if (significance1) {
      sign0 = coefficentsSign[index - 1];
      contribution = 1 - sign0 - sign0;
    } else {
      contribution = 0;
    }
    const horizontalContribution = 3 * contribution;

    // calculate vertical contribution and combine with the horizontal
    significance1 = row > 0 && coefficentsMagnitude[index - width] !== 0;
    if (row + 1 < height && coefficentsMagnitude[index + width] !== 0) {
      sign1 = coefficentsSign[index + width];
      if (significance1) {
        sign0 = coefficentsSign[index - width];
        contribution = 1 - sign1 - sign0 + horizontalContribution;
      } else {
        contribution = 1 - sign1 - sign1 + horizontalContribution;
      }
    } else if (significance1) {
      sign0 = coefficentsSign[index - width];
      contribution = 1 - sign0 - sign0 + horizontalContribution;
    } else {
      contribution = horizontalContribution;
    }

    if (contribution >= 0) {
      contextLabel = 9 + contribution;
      decoded = decoder.readBit(contexts, contextLabel) ?? 0;
    } else {
      contextLabel = 9 - contribution;
      decoded = (decoder.readBit(contexts, contextLabel) ?? 0) ^ 1;
    }
    return decoded;
  }
  /** Run magnitude refinement */
  runMagnitudeRefinementPass(): void {
    const {
      decoder,
      contexts,
      width,
      height,
      bitsDecoded,
      processingFlags,
      coefficentsMagnitude,
      neighborsSignificance,
    } = this;
    const processedMask = 1;
    const firstMagnitudeBitMask = 2;
    const length = width * height;
    const width4 = width * 4;

    for (let index0 = 0, indexNext: number; index0 < length; index0 = indexNext) {
      indexNext = Math.min(length, index0 + width4);
      for (let j = 0; j < width; j++) {
        for (let index = index0 + j; index < indexNext; index += width) {
          // significant but not those that have just become
          if (coefficentsMagnitude[index] === 0 || (processingFlags[index] & processedMask) !== 0) {
            continue;
          }

          let contextLabel = 16;
          if ((processingFlags[index] & firstMagnitudeBitMask) !== 0) {
            processingFlags[index] ^= firstMagnitudeBitMask;
            // first refinement
            const significance = neighborsSignificance[index] & 127;
            contextLabel = significance === 0 ? 15 : 14;
          }

          const bit = decoder.readBit(contexts, contextLabel) ?? 0;
          coefficentsMagnitude[index] = (coefficentsMagnitude[index] << 1) | bit;
          bitsDecoded[index]++;
          processingFlags[index] |= processedMask;
        }
      }
    }
  }
  /** Run cleanup pass */
  runCleanupPass(): void {
    const {
      decoder,
      contexts,
      width,
      height,
      neighborsSignificance,
      coefficentsMagnitude,
      coefficentsSign,
      contextLabelTable: labels,
      bitsDecoded,
      processingFlags,
    } = this;
    const processedMask = 1;
    const firstMagnitudeBitMask = 2;
    const oneRowDown = width;
    const twoRowsDown = width * 2;
    const threeRowsDown = width * 3;
    let iNext;
    for (let i0 = 0; i0 < height; i0 = iNext) {
      iNext = Math.min(i0 + 4, height);
      const indexBase = i0 * width;
      const checkAllEmpty = i0 + 3 < height;
      for (let j = 0; j < width; j++) {
        const index0 = indexBase + j;
        // using the property: labels[neighborsSignificance[index]] === 0
        // when neighborsSignificance[index] === 0
        const allEmpty =
          checkAllEmpty &&
          processingFlags[index0] === 0 &&
          processingFlags[index0 + oneRowDown] === 0 &&
          processingFlags[index0 + twoRowsDown] === 0 &&
          processingFlags[index0 + threeRowsDown] === 0 &&
          neighborsSignificance[index0] === 0 &&
          neighborsSignificance[index0 + oneRowDown] === 0 &&
          neighborsSignificance[index0 + twoRowsDown] === 0 &&
          neighborsSignificance[index0 + threeRowsDown] === 0;
        let i1 = 0;
        let index = index0;
        let i = i0;
        let sign: number;
        if (allEmpty) {
          const hasSignificantCoefficent = decoder.readBit(contexts, RUNLENGTH_CONTEXT);
          if (hasSignificantCoefficent === 0) {
            bitsDecoded[index0]++;
            bitsDecoded[index0 + oneRowDown]++;
            bitsDecoded[index0 + twoRowsDown]++;
            bitsDecoded[index0 + threeRowsDown]++;
            continue; // next column
          }
          i1 =
            (decoder.readBit(contexts, UNIFORM_CONTEXT) << 1) |
            decoder.readBit(contexts, UNIFORM_CONTEXT);
          if (i1 !== 0) {
            i = i0 + i1;
            index += i1 * width;
          }

          sign = this.decodeSignBit(i, j, index);
          coefficentsSign[index] = sign;
          coefficentsMagnitude[index] = 1;
          this.setNeighborsSignificance(i, j, index);
          processingFlags[index] |= firstMagnitudeBitMask;

          index = index0;
          for (let i2 = i0; i2 <= i; i2++, index += width) {
            bitsDecoded[index]++;
          }

          i1++;
        }
        for (i = i0 + i1; i < iNext; i++, index += width) {
          if (coefficentsMagnitude[index] !== 0 || (processingFlags[index] & processedMask) !== 0) {
            continue;
          }

          const contextLabel = labels[neighborsSignificance[index]];
          const decision = decoder.readBit(contexts, contextLabel);
          if (decision === 1) {
            sign = this.decodeSignBit(i, j, index);
            coefficentsSign[index] = sign;
            coefficentsMagnitude[index] = 1;
            this.setNeighborsSignificance(i, j, index);
            processingFlags[index] |= firstMagnitudeBitMask;
          }
          bitsDecoded[index]++;
        }
      }
    }
  }
  /** Check segmentation symbol */
  checkSegmentationSymbol(): void {
    const decoder = this.decoder;
    const contexts = this.contexts;
    const symbol =
      (decoder.readBit(contexts, UNIFORM_CONTEXT) << 3) |
      (decoder.readBit(contexts, UNIFORM_CONTEXT) << 2) |
      (decoder.readBit(contexts, UNIFORM_CONTEXT) << 1) |
      decoder.readBit(contexts, UNIFORM_CONTEXT);
    if (symbol !== 0xa) {
      throw new Error('Invalid segmentation symbol');
    }
  }
}

/**
 * Section F, Discrete wavelet transformation
 */
class Transform {
  /**
   * Default filter function to fill with parent classes
   * @param _x - buffer
   * @param _offset - offset in the buffer to start
   * @param _length - length of the buffer to read
   */
  filter(_x: Float32Array, _offset: number, _length: number): void {}

  /**
   * Calculate
   * @param subbands - subbands
   * @param u0 - u
   * @param v0 - v
   * @returns - the calculated subbands
   */
  calculate(subbands: Level[], u0: number, v0: number): Level {
    let ll = subbands[0];
    for (let i = 1, ii = subbands.length; i < ii; i++) {
      ll = this.iterate(ll, subbands[i], u0, v0);
    }
    return ll;
  }
  /**
   * Extend the buffer
   * @param buffer - the buffer
   * @param offset - the offset
   * @param size - the size to extend by
   */
  extend(buffer: Float32Array, offset: number, size: number): void {
    // Section F.3.7 extending... using max extension of 4
    let i1 = offset - 1,
      j1 = offset + 1;
    let i2 = offset + size - 2,
      j2 = offset + size;
    buffer[i1--] = buffer[j1++];
    buffer[j2++] = buffer[i2--];
    buffer[i1--] = buffer[j1++];
    buffer[j2++] = buffer[i2--];
    buffer[i1--] = buffer[j1++];
    buffer[j2++] = buffer[i2--];
    buffer[i1] = buffer[j1];
    buffer[j2] = buffer[i2];
  }
  /**
   * Level Iterator
   * @param ll - level
   * @param hl_lh_hh - boxed level
   * @param u0 - u
   * @param v0 - v
   * @returns - the level
   */
  iterate(ll: Level, hl_lh_hh: Level, u0: number, v0: number): Level {
    const llWidth = ll.width,
      llHeight = ll.height,
      llItems = ll.items;
    const width = hl_lh_hh.width;
    const height = hl_lh_hh.height;
    // fix items if needed
    if (Array.isArray(hl_lh_hh.items)) hl_lh_hh.items = new Uint8Array(hl_lh_hh.items);
    const items = hl_lh_hh.items;
    let i, j, k, l, u, v;

    // Interleave LL according to Section F.3.3
    for (k = 0, i = 0; i < llHeight; i++) {
      l = i * 2 * width;
      for (j = 0; j < llWidth; j++, k++, l += 2) {
        items[l] = llItems[k];
      }
    }
    // The LL band is not needed anymore.
    // llItems = ll.items = null;

    const bufferPadding = 4;
    const rowBuffer = new Float32Array(width + 2 * bufferPadding);

    // Section F.3.4 HOR_SR
    if (width === 1) {
      // if width = 1, when u0 even keep items as is, when odd divide by 2
      if ((u0 & 1) !== 0) {
        for (v = 0, k = 0; v < height; v++, k += width) {
          items[k] *= 0.5;
        }
      }
    } else {
      for (v = 0, k = 0; v < height; v++, k += width) {
        rowBuffer.set(items.subarray(k, k + width), bufferPadding);

        this.extend(rowBuffer, bufferPadding, width);
        this.filter(rowBuffer, bufferPadding, width);

        items.set(rowBuffer.subarray(bufferPadding, bufferPadding + width), k);
      }
    }

    // Accesses to the items array can take long, because it may not fit into
    // CPU cache and has to be fetched from main memory. Since subsequent
    // accesses to the items array are not local when reading columns, we
    // have a cache miss every time. To reduce cache misses, get up to
    // 'numBuffers' items at a time and store them into the individual
    // buffers. The colBuffers should be small enough to fit into CPU cache.
    let numBuffers = 16;
    const colBuffers = [];
    for (i = 0; i < numBuffers; i++) {
      colBuffers.push(new Float32Array(height + 2 * bufferPadding));
    }
    let b: number,
      currentBuffer = 0;
    const ll2 = bufferPadding + height;

    // Section F.3.5 VER_SR
    if (height === 1) {
      // if height = 1, when v0 even keep items as is, when odd divide by 2
      if ((v0 & 1) !== 0) {
        for (u = 0; u < width; u++) {
          items[u] *= 0.5;
        }
      }
    } else {
      for (u = 0; u < width; u++) {
        // if we ran out of buffers, copy several image columns at once
        if (currentBuffer === 0) {
          numBuffers = Math.min(width - u, numBuffers);
          for (k = u, l = bufferPadding; l < ll2; k += width, l++) {
            for (b = 0; b < numBuffers; b++) {
              colBuffers[b][l] = items[k + b];
            }
          }
          currentBuffer = numBuffers;
        }

        currentBuffer--;
        const buffer = colBuffers[currentBuffer];
        this.extend(buffer, bufferPadding, height);
        this.filter(buffer, bufferPadding, height);

        // If this is last buffer in this group of buffers, flush all buffers.
        if (currentBuffer === 0) {
          k = u - numBuffers + 1;
          for (l = bufferPadding; l < ll2; k += width, l++) {
            for (b = 0; b < numBuffers; b++) {
              items[k + b] = colBuffers[b][l];
            }
          }
        }
      }
    }

    return {
      width,
      height,
      items,
    };
  }
}

/**
 * Section 3.8.2 Irreversible 9-7 filter
 */
class IrreversibleTransform extends Transform {
  /**
   * Filter function
   * @param x - buffer
   * @param offset - offset
   * @param length - length
   */
  filter(x: Float32Array, offset: number, length: number): void {
    const len = length >> 1;
    offset = offset | 0;
    let j: number, current: number, next: number;

    const alpha = -1.586134342059924;
    const beta = -0.052980118572961;
    const gamma = 0.882911075530934;
    const delta = 0.443506852043971;
    const K = 1.230174104914001;
    const K_ = 1 / K;

    // step 1 is combined with step 3

    // step 2
    j = offset - 3;
    for (let n = len + 4; n-- !== 0; j += 2) {
      x[j] *= K_;
    }

    // step 1 & 3
    j = offset - 2;
    current = delta * x[j - 1];
    for (let n = len + 3; n-- !== 0; j += 2) {
      next = delta * x[j + 1];
      x[j] = K * x[j] - current - next;
      if (n-- !== 0) {
        j += 2;
        current = delta * x[j + 1];
        x[j] = K * x[j] - current - next;
      } else {
        break;
      }
    }

    // step 4
    j = offset - 1;
    current = gamma * x[j - 1];
    for (let n = len + 2; n-- !== 0; j += 2) {
      next = gamma * x[j + 1];
      x[j] -= current + next;
      if (n-- !== 0) {
        j += 2;
        current = gamma * x[j + 1];
        x[j] -= current + next;
      } else {
        break;
      }
    }

    // step 5
    j = offset;
    current = beta * x[j - 1];
    for (let n = len + 1; n-- !== 0; j += 2) {
      next = beta * x[j + 1];
      x[j] -= current + next;
      if (n-- !== 0) {
        j += 2;
        current = beta * x[j + 1];
        x[j] -= current + next;
      } else {
        break;
      }
    }

    // step 6
    if (len !== 0) {
      j = offset + 1;
      current = alpha * x[j - 1];
      for (let n = len; n-- !== 0; j += 2) {
        next = alpha * x[j + 1];
        x[j] -= current + next;
        if (n-- !== 0) {
          j += 2;
          current = alpha * x[j + 1];
          x[j] -= current + next;
        } else {
          break;
        }
      }
    }
  }
}

/**
 * Section 3.8.1 Reversible 5-3 filter
 */
class ReversibleTransform extends Transform {
  /**
   * Filter
   * @param x - buffer
   * @param offset - offset
   * @param length - length
   */
  filter(x: Float32Array, offset: number, length: number): void {
    const len = length >> 1;
    offset = offset | 0;
    let j: number, n: number;

    for (j = offset, n = len + 1; n-- !== 0; j += 2) {
      x[j] -= (x[j - 1] + x[j + 1] + 2) >> 2;
    }

    for (j = offset + 1, n = len; n-- !== 0; j += 2) {
      x[j] += (x[j - 1] + x[j + 1]) >> 1;
    }
  }
}

// Table C-2
const QeTable = [
  { qe: 0x5601, nmps: 1, nlps: 1, switchFlag: 1 },
  { qe: 0x3401, nmps: 2, nlps: 6, switchFlag: 0 },
  { qe: 0x1801, nmps: 3, nlps: 9, switchFlag: 0 },
  { qe: 0x0ac1, nmps: 4, nlps: 12, switchFlag: 0 },
  { qe: 0x0521, nmps: 5, nlps: 29, switchFlag: 0 },
  { qe: 0x0221, nmps: 38, nlps: 33, switchFlag: 0 },
  { qe: 0x5601, nmps: 7, nlps: 6, switchFlag: 1 },
  { qe: 0x5401, nmps: 8, nlps: 14, switchFlag: 0 },
  { qe: 0x4801, nmps: 9, nlps: 14, switchFlag: 0 },
  { qe: 0x3801, nmps: 10, nlps: 14, switchFlag: 0 },
  { qe: 0x3001, nmps: 11, nlps: 17, switchFlag: 0 },
  { qe: 0x2401, nmps: 12, nlps: 18, switchFlag: 0 },
  { qe: 0x1c01, nmps: 13, nlps: 20, switchFlag: 0 },
  { qe: 0x1601, nmps: 29, nlps: 21, switchFlag: 0 },
  { qe: 0x5601, nmps: 15, nlps: 14, switchFlag: 1 },
  { qe: 0x5401, nmps: 16, nlps: 14, switchFlag: 0 },
  { qe: 0x5101, nmps: 17, nlps: 15, switchFlag: 0 },
  { qe: 0x4801, nmps: 18, nlps: 16, switchFlag: 0 },
  { qe: 0x3801, nmps: 19, nlps: 17, switchFlag: 0 },
  { qe: 0x3401, nmps: 20, nlps: 18, switchFlag: 0 },
  { qe: 0x3001, nmps: 21, nlps: 19, switchFlag: 0 },
  { qe: 0x2801, nmps: 22, nlps: 19, switchFlag: 0 },
  { qe: 0x2401, nmps: 23, nlps: 20, switchFlag: 0 },
  { qe: 0x2201, nmps: 24, nlps: 21, switchFlag: 0 },
  { qe: 0x1c01, nmps: 25, nlps: 22, switchFlag: 0 },
  { qe: 0x1801, nmps: 26, nlps: 23, switchFlag: 0 },
  { qe: 0x1601, nmps: 27, nlps: 24, switchFlag: 0 },
  { qe: 0x1401, nmps: 28, nlps: 25, switchFlag: 0 },
  { qe: 0x1201, nmps: 29, nlps: 26, switchFlag: 0 },
  { qe: 0x1101, nmps: 30, nlps: 27, switchFlag: 0 },
  { qe: 0x0ac1, nmps: 31, nlps: 28, switchFlag: 0 },
  { qe: 0x09c1, nmps: 32, nlps: 29, switchFlag: 0 },
  { qe: 0x08a1, nmps: 33, nlps: 30, switchFlag: 0 },
  { qe: 0x0521, nmps: 34, nlps: 31, switchFlag: 0 },
  { qe: 0x0441, nmps: 35, nlps: 32, switchFlag: 0 },
  { qe: 0x02a1, nmps: 36, nlps: 33, switchFlag: 0 },
  { qe: 0x0221, nmps: 37, nlps: 34, switchFlag: 0 },
  { qe: 0x0141, nmps: 38, nlps: 35, switchFlag: 0 },
  { qe: 0x0111, nmps: 39, nlps: 36, switchFlag: 0 },
  { qe: 0x0085, nmps: 40, nlps: 37, switchFlag: 0 },
  { qe: 0x0049, nmps: 41, nlps: 38, switchFlag: 0 },
  { qe: 0x0025, nmps: 42, nlps: 39, switchFlag: 0 },
  { qe: 0x0015, nmps: 43, nlps: 40, switchFlag: 0 },
  { qe: 0x0009, nmps: 44, nlps: 41, switchFlag: 0 },
  { qe: 0x0005, nmps: 45, nlps: 42, switchFlag: 0 },
  { qe: 0x0001, nmps: 45, nlps: 43, switchFlag: 0 },
  { qe: 0x5601, nmps: 46, nlps: 46, switchFlag: 0 },
];

/**
 * Calculate the base 2 logarithm of the number `x`. This differs from the
 * native function in the sense that it returns the ceiling value and that it
 * returns 0 instead of `Infinity`/`NaN` for `x` values smaller than/equal to 0.
 * @param x - the number to calculate the logarithm of
 * @returns the base 2 logarithm
 */
function log2(x: number): number {
  if (x <= 0) return 0;
  return Math.ceil(Math.log2(x));
}

/**
 * This class implements the QM Coder decoding as defined in
 *   JPEG 2000 Part I Final Committee Draft Version 1.0
 *   Annex C.3 Arithmetic decoding procedure
 * available at http://www.jpeg.org/public/fcd15444-1.pdf
 *
 * The arithmetic decoder is used in conjunction with context models to decode
 * JPEG2000 and JBIG2 streams.
 */
export class ArithmeticDecoder {
  a: number;
  chigh: number;
  ct = 0;
  clow = 0;
  /**
   * C.3.5 Initialisation of the decoder (INITDEC)
   * @param data - compressed data
   * @param start - start index
   * @param end - end index
   */
  constructor(
    public data: Uint8Array,
    public start: number,
    public end: number,
  ) {
    this.chigh = data[start];
    this.byteIn();

    this.chigh = ((this.chigh << 7) & 0xffff) | ((this.clow >> 9) & 0x7f);
    this.clow = (this.clow << 7) & 0xffff;
    this.ct -= 7;
    this.a = 0x8000;
  }

  /** C.3.4 Compressed data input (BYTEIN) */
  byteIn(): void {
    const data = this.data;
    let bp = this.start;

    if (data[bp] === 0xff) {
      if (data[bp + 1] > 0x8f) {
        this.clow += 0xff00;
        this.ct = 8;
      } else {
        bp++;
        this.clow += data[bp] << 9;
        this.ct = 7;
        this.start = bp;
      }
    } else {
      bp++;
      this.clow += bp < this.end ? data[bp] << 8 : 0xff00;
      this.ct = 8;
      this.start = bp;
    }
    if (this.clow > 0xffff) {
      this.chigh += this.clow >> 16;
      this.clow &= 0xffff;
    }
  }

  /**
   * C.3.2 Decoding a decision (DECODE)
   * @param contexts - context models
   * @param pos - context model index
   * @returns 0 or 1
   */
  readBit(contexts: number[] | Int8Array, pos: number): number {
    // Contexts are packed into 1 byte:
    // highest 7 bits carry cx.index, lowest bit carries cx.mps
    let cx_index = contexts[pos] >> 1,
      cx_mps = contexts[pos] & 1;
    const qeTableIcx = QeTable[cx_index];
    const qeIcx = qeTableIcx.qe;
    let d;
    let a = this.a - qeIcx;

    if (this.chigh < qeIcx) {
      // exchangeLps
      if (a < qeIcx) {
        a = qeIcx;
        d = cx_mps;
        cx_index = qeTableIcx.nmps;
      } else {
        a = qeIcx;
        d = 1 ^ cx_mps;
        if (qeTableIcx.switchFlag === 1) {
          cx_mps = d;
        }
        cx_index = qeTableIcx.nlps;
      }
    } else {
      this.chigh -= qeIcx;
      if ((a & 0x8000) !== 0) {
        this.a = a;
        return cx_mps;
      }
      // exchangeMps
      if (a < qeIcx) {
        d = 1 ^ cx_mps;
        if (qeTableIcx.switchFlag === 1) {
          cx_mps = d;
        }
        cx_index = qeTableIcx.nlps;
      } else {
        d = cx_mps;
        cx_index = qeTableIcx.nmps;
      }
    }
    // C.3.3 renormD;
    do {
      if (this.ct === 0) this.byteIn();

      a <<= 1;
      this.chigh = ((this.chigh << 1) & 0xffff) | ((this.clow >> 15) & 1);
      this.clow = (this.clow << 1) & 0xffff;
      this.ct--;
    } while ((a & 0x8000) === 0);
    this.a = a;

    contexts[pos] = (cx_index << 1) | cx_mps;
    return d;
  }
}
