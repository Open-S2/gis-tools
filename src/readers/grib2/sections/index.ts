import { BufferReader } from '../../';
import { parseSection0 } from './0';
import { parseSection1 } from './1';
import { parseSection2 } from './2';
import { parseSection3 } from './3';
import { parseSection4 } from './4';
import { parseSection5 } from './5';
import { parseSection6 } from './6';
import { parseSection7 } from './7';
import { parseSection8 } from './8';

import type { Reader } from '../..';
import type {
  BitMapSection,
  DataRepresentationSection,
  DataSection,
  EndSection,
  GridDefinitionSection,
  IdentificationSection,
  IndicatorSection,
  LocalUseSection,
  ProductDefinitionSection,
} from '.';

export * from './0';
export * from './1';
export * from './2';
export * from './3';
export * from './4';
export * from './5';
export * from './6';
export * from './7';
export * from './8';
export * from './other';

/** A parsed GRIB Section */
export type Sections = {
  indicator?: IndicatorSection;
  identification?: IdentificationSection;
  local?: LocalUseSection;
  gridDefinition?: GridDefinitionSection;
  productDefinition?: ProductDefinitionSection;
  dataRepresentation?: DataRepresentationSection;
  bitMap?: BitMapSection;
  data?: DataSection;
  end?: EndSection;
};

/**
 * @param gribChunk Buffer containing individual GRIB definition
 * @returns Array of Section Buffers where the index of the item corresponds to the section number. If a section is missing, it will be represented as null
 */
export function splitSectionChunks(gribChunk: Reader): Sections {
  const sections: Sections = {};

  let currentSection = gribChunk;
  // Split sections in file
  while (currentSection.byteLength !== 0) {
    const sectionNumber = getSectionNumber(currentSection);

    // First section length is always 16 bytes long and is identified by the first 4 bytes being 'GRIB'
    // const length = sectionNumber === 0 ? 16 : currentSection.getUint32(0);
    const length = sectionNumber === 0 ? 16 : currentSection.getUint32(0);
    const section = new BufferReader(currentSection.slice(0, length).buffer);
    currentSection = new BufferReader(currentSection.slice(length).buffer);

    parseSection(section, sections);
  }

  return sections;
}

/**
 * Parse the given section
 * @param reader - The section to parse
 * @param sections - The result to write to
 */
function parseSection(reader: Reader, sections: Sections): void {
  const sectionNumber = getSectionNumber(reader);

  switch (sectionNumber) {
    case 0:
      sections.indicator = parseSection0(reader);
      break;
    case 1:
      sections.identification = parseSection1(reader);
      break;
    case 2:
      sections.local = parseSection2(reader);
      break;
    case 3:
      sections.gridDefinition = parseSection3(reader);
      break;
    case 4:
      sections.productDefinition = parseSection4(reader, sections);
      break;
    case 5:
      sections.dataRepresentation = parseSection5(reader);
      break;
    case 6:
      sections.bitMap = parseSection6(reader);
      break;
    case 7:
      sections.data = parseSection7(reader, sections);
      break;
    case 8:
      sections.end = parseSection8(reader);
      break;
    default:
      throw new Error(`Unknown section number: ${sectionNumber}`);
  }
}

/**
 * @param section Buffer containing GRIB Section data
 * @returns Section number of the input GRIB Section data
 */
export function getSectionNumber(section: Reader): number {
  const first4ByteString = section.parseString(0, 4);

  if (first4ByteString === 'GRIB') return 0;
  if (first4ByteString === '7777') return 8;

  const sectionNumber = section.getUint8(4);

  return sectionNumber;
}
