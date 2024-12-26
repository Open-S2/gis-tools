import { BufferReader } from '../../';
import { parseGrib2Section0 } from './0';
import { parseGrib2Section1 } from './1';
import { parseGrib2Section2 } from './2';
import { parseGrib2Section3 } from './3';
import { parseGrib2Section4 } from './4';
import { parseGrib2Section5 } from './5';
import { parseGrib2Section6 } from './6';
import { parseGrib2Section7 } from './7';
import { parseGrib2Section8 } from './8';

import type { Reader } from '../..';
import type {
  Grib2BitMapSection,
  Grib2DataRepresentationSection,
  Grib2DataSection,
  Grib2EndSection,
  Grib2IdentificationSection,
  Grib2IndicatorSection,
  Grib2LocalUseSection,
  Grib2ProductDefinitionSection,
  GridDefinitionSection,
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
export type Grib2Sections = {
  indicator?: Grib2IndicatorSection;
  identification?: Grib2IdentificationSection;
  local?: Grib2LocalUseSection;
  gridDefinition?: GridDefinitionSection;
  productDefinition?: Grib2ProductDefinitionSection;
  dataRepresentation?: Grib2DataRepresentationSection;
  bitMap?: Grib2BitMapSection;
  data?: Grib2DataSection;
  end?: Grib2EndSection;
};

/**
 * @param gribChunk Buffer containing individual GRIB definition
 * @returns Array of Section Buffers where the index of the item corresponds to the section number. If a section is missing, it will be represented as null
 */
export function splitSectionChunks(gribChunk: Reader): Grib2Sections {
  const sections: Grib2Sections = {};

  let currentSection = gribChunk;
  // Split sections in file
  while (currentSection.byteLength !== 0) {
    const sectionNumber = getSectionNumber(currentSection);

    // First section length is always 16 bytes long and is identified by the first 4 bytes being 'GRIB'
    // const length = sectionNumber === 0 ? 16 : currentSection.getUint32(0);
    const length = sectionNumber === 0 ? 16 : currentSection.getUint32(0);
    const section = new BufferReader(currentSection.slice(0, length).buffer);
    currentSection = new BufferReader(currentSection.slice(length).buffer);

    parseGrib2Section(section, sections);
  }

  return sections;
}

/**
 * Parse the given section
 * @param reader - The section to parse
 * @param sections - The result to write to
 */
function parseGrib2Section(reader: Reader, sections: Grib2Sections): void {
  const sectionNumber = getSectionNumber(reader);

  switch (sectionNumber) {
    case 0:
      sections.indicator = parseGrib2Section0(reader);
      break;
    case 1:
      sections.identification = parseGrib2Section1(reader);
      break;
    case 2:
      sections.local = parseGrib2Section2(reader);
      break;
    case 3:
      sections.gridDefinition = parseGrib2Section3(reader);
      break;
    case 4:
      sections.productDefinition = parseGrib2Section4(reader, sections);
      break;
    case 5:
      sections.dataRepresentation = parseGrib2Section5(reader);
      break;
    case 6:
      sections.bitMap = parseGrib2Section6(reader);
      break;
    case 7:
      sections.data = parseGrib2Section7(reader, sections);
      break;
    case 8:
      sections.end = parseGrib2Section8(reader);
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
