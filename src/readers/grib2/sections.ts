import { convertData } from './data';
import {
  getTemplate3,
  getTemplate4,
  getTemplate5,
  lookupTemplate3,
  lookupTemplate4,
  lookupTemplate5,
} from './templates';
import {
  lookupTable0,
  lookupTable00,
  lookupTable12,
  lookupTable14,
  lookupTable31,
  lookupTable40,
  lookupTable50,
  lookupTableC,
} from './tables';

/**
 *
 */
export type SectionValues = [
  IndicatorSectionValues,
  IdentificationSectionValues,
  LocalUseSectionValues,
  GridDefinitionSectionValues,
  ProductDefinitionSectionValues,
  DataRepresentationSectionValues,
  BitMapSectionValues,
  DataSectionValues,
  EndSectionValues,
];

/**
 *
 */
export type Sections = [
  IndicatorSection,
  IdentificationSection,
  LocalUseSection,
  GridDefinitionSection,
  ProductDefinitionSection,
  DataRepresentationSection,
  BitMapSection,
  DataSection,
  EndSection,
];

/**
 * @param gribChunk Buffer containing individual GRIB definition
 * @returns Array of Section Buffers where the index of the item corresponds to the section number. If a section is missing, it will be represented as null
 */
export function splitSectionChunks(gribChunk: Buffer): Array<Buffer | null> {
  const sections: Array<Buffer | null> = new Array(9).fill(null);

  let currentSection = gribChunk;
  // Split sections in file
  while (currentSection.length !== 0) {
    const sectionNumber = getSectionNumber(currentSection);

    // First section length is always 16 bytes long and is identified by the first 4 bytes being 'GRIB'
    const length = sectionNumber === 0 ? 16 : currentSection.readUInt32BE(0);

    const section = currentSection.subarray(0, length);
    currentSection = currentSection.subarray(length);

    sections[sectionNumber] = section;
  }

  return sections;
}

/**
 * @param sections Array of GRIB Section Buffers
 * @returns Array of Parsed Sections where index corresponds to section number
 */
export function parseSections(sections: Buffer[]): SectionValues {
  return sections.map(parseSection) as SectionValues;
}

/**
 * @param section Buffer containing GRIB Section data
 * @returns Parsed Section Values
 */
function parseSection(section: Buffer) {
  const sectionNumber = getSectionNumber(section);

  switch (sectionNumber) {
    case 0:
      return parseSection0(section);
    case 1:
      return parseSection1(section);
    case 2:
      return parseSection2(section);
    case 3:
      return parseSection3(section);
    case 4:
      return parseSection4(section);
    case 5:
      return parseSection5(section);
    case 6:
      return parseSection6(section);
    case 7:
      return parseSection7(section);
    case 8:
      return parseSection8(section);
    default:
      throw new Error(`Unknown section number: ${sectionNumber}`);
  }
}

/**
 * @param parsedSections Array of Parsed Sections
 * @returns Array of Sections with values looked up in tables
 */
export function lookupSections(parsedSections: SectionValues): Sections {
  const [ins, ids, lus, gds, pds, drs, bms, ds, es] = parsedSections;

  return [
    lookupSection0(ins),
    lookupSection1(ids),
    lus,
    lookupSection3(gds),
    lookupSection4(pds, ins, ids),
    lookupSection5(drs),
    bms,
    lookupSection7(ds, drs, bms),
    es,
  ];
}

/**
 * @param section Buffer containing GRIB Section data
 * @returns Section number of the input GRIB Section data
 */
export function getSectionNumber(section: Buffer): number {
  const first4Bytes = section.subarray(0, 4);

  if (first4Bytes.toString() === 'GRIB') return 0;
  if (first4Bytes.toString() === '7777') return 8;

  const sectionNumber = section.readUInt8(4);

  return sectionNumber;
}

//!!! SECTION 0

/** The output of `parseSection0` */
export type IndicatorSectionValues = ReturnType<typeof parseSection0>;
/** The output of `lookupSection0` */
export type IndicatorSection = ReturnType<typeof lookupSection0>;

/**
 *  Indicator Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect0.shtml)
 * @param section
 */
export function parseSection0(section: Buffer) {
  return {
    /** Number of GRIB section */
    sectionNumber: 0,
    /** Name of Grib section */
    sectionName: 'Indicator Section',
    /** Length of GRIB section (Always 16 for Section 0)*/
    length: 16,
    /** Section 0 Contents */
    contents: {
      /** GRIB string encoded */
      gribEncoded: section.subarray(0, 4).toString(),
      /** Discipline [Table 0.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table0-0.shtml) */
      discipline: section.readUInt8(6),
      /** Edition number - 2 for GRIB2 */
      gribEdition: section.readUInt8(7),
      /** Total length of GRIB message in octets (All sections) */
      gribLength: Number(section.readBigUInt64BE(8)),
    },
  };
}

/**
 * @param ins Indictor Section
 * @returns Indicator Section with corresponding string values
 */
export function lookupSection0(ins: IndicatorSectionValues) {
  return {
    ...ins,
    contents: {
      ...ins.contents,
      /** Discipline */
      discipline: lookupTable00[ins.contents.discipline],
    },
  };
}

//!!! SECTION 1

/** The output of `parseSection1` */
export type IdentificationSectionValues = ReturnType<typeof parseSection1>;
/** The output of `lookupSection1` */
export type IdentificationSection = ReturnType<typeof lookupSection1>;

/**
 *  Identification Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect1.shtml)
 * @param section
 */
export function parseSection1(section: Buffer) {
  // Ref Time values
  const year = section.readUInt16BE(12);
  const month = section.readUInt8(14);
  const day = section.readUInt8(15);
  const hours = section.readUInt8(16);
  const minutes = section.readUInt8(17);
  const seconds = section.readUInt8(18);

  const refTime = new Date(year, month - 1, day);
  refTime.setUTCHours(hours);
  refTime.setUTCMinutes(minutes);
  refTime.setUTCSeconds(seconds);

  return {
    /** Number of GRIB section */
    sectionNumber: section.readUInt8(4),
    /** Name of Grib section */
    sectionName: 'Identification Section',
    /** Length of GRIB section */
    length: section.readUInt32BE(0),
    /** Section 1 Contents */
    contents: {
      /** Identification of originating/generating center [Table 0](https://www.nco.ncep.noaa.gov/pmb/docs/on388/table0.html) */
      center: section.readUInt16BE(5),
      /** Identification of originating/generating subcenter [Table C](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablec.html) */
      subcenter: section.readUInt16BE(7),
      /** GRIB master tables version number [Table 1.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-0.shtml) */
      gribMasterTablesVersion: section.readUInt8(9),
      /** Version number of GRIB local tables used to augment Master Tables [Table 1.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-1.shtml) */
      gribLocalTablesVersion: section.readUInt8(10),
      /** Significance of reference time [Table 1.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-2.shtml) */
      significanceOfRT: section.readUInt8(11),
      /** Reference Time */
      refTime,
      /** Type of processed data in this GRIB message [Table 1.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-4.shtml) */
      typeOfProcessedData: section.readUInt8(20),
    },
  };
}

/**
 * @param ids Identification Section
 * @returns Identification Section with corresponding string values
 */
export function lookupSection1(ids: IdentificationSectionValues) {
  return {
    ...ids,
    contents: {
      ...ids.contents,
      /** Identification of originating/generating center */
      center: lookupTable0[ids.contents.center],
      /** Identification of originating/generating subcenter */
      subcenter: lookupTableC[ids.contents.subcenter],
      /** Significance of reference time */
      significanceOfRT: lookupTable12[ids.contents.subcenter],
      /** Type of processed data in this GRIB message */
      typeOfProcessedData: lookupTable14[ids.contents.typeOfProcessedData],
    },
  };
}

//!!! SECTION 2

/** The output of `parseSection2` */
export type LocalUseSectionValues = ReturnType<typeof parseSection2>;
/** The output of `lookupSection2` */
export type LocalUseSection = ReturnType<typeof parseSection2>;

/**
 *  Local Use Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect2.shtml)
 * @param _section
 * @param section
 */
export function parseSection2(section: Buffer) {
  return {
    /** Number of GRIB section */
    sectionNumber: section.readUInt8(4),
    /** Name of Grib section */
    sectionName: 'Local Use Section',
    /** Length of GRIB section */
    length: section.readUInt32BE(0),
    /** Section 2 Contents */
    contents: { data: section.subarray(5) },
  };
}

/**
 * @param _ids Local Use Section
 * @returns Local Use Section with corresponding string values
 */
export function lookupSection2(_ids: LocalUseSectionValues) {
  throw new Error('Section 2 is not supported');
}

//!!! SECTION 3

/** The output of `parseSection3` */
export type GridDefinitionSectionValues = ReturnType<typeof parseSection3>;
/** The output of `lookupSection3` */
export type GridDefinitionSection = ReturnType<typeof lookupSection3>;

/**
 *  Grid Definition Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect3.shtml)
 * @param section
 */
export function parseSection3(section: Buffer) {
  const gridDefinitionTemplate = section.readUInt16BE(12);
  const gridDefinition = getTemplate3(gridDefinitionTemplate)(section);

  return {
    /** Number of GRIB section */
    sectionNumber: section.readUInt8(4),
    /** Name of Grib section */
    sectionName: 'Grid Definition Section',
    /** Length of GRIB section */
    length: section.readUInt32BE(0),
    /** Section 3 Contents */
    contents: {
      // Source of grid definition
      definitionSource: section.readUInt8(5),
      /** Number of data points */
      numberOfPoints: section.readUInt32BE(6),
      // Number of octets for optional list of numbers defining number of points
      numberOfOctets: section.readUInt8(10),
      // Interpetation of list of numbers defining number of points [Table 3.11](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-11.shtml)
      interpretation: section.readUInt8(11),
      /** Grid definition template number [Table 3.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml) */
      gridDefinitionTemplate,
      /** Grid definition values */
      gridDefinition,
    },
  };
}

/**
 * @param gds Grid Definition Section
 * @returns Grid Definition Section with corresponding string values
 */
export function lookupSection3(gds: GridDefinitionSectionValues) {
  const { gridDefinitionTemplate } = gds.contents;

  return {
    ...gds,
    contents: {
      ...gds.contents,
      /** Grid definition template */
      gridDefinitionTemplate: lookupTable31[gridDefinitionTemplate],
      /** Grid definition values */
      gridDefinition: lookupTemplate3(gridDefinitionTemplate)(gds.contents.gridDefinition),
    },
  };
}

//!!! SECTION 4

/** The output of `parseSection4` */
export type ProductDefinitionSectionValues = ReturnType<typeof parseSection4>;
/** The output of `lookupSection4` */
export type ProductDefinitionSection = ReturnType<typeof lookupSection4>;

/**
 *  Product Definition Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect4.shtml)
 * @param section
 */
export function parseSection4(section: Buffer) {
  const productDefinitionTemplate = section.readUInt16BE(7);
  const productDefinition = getTemplate4(productDefinitionTemplate)(section);

  return {
    /** Number of GRIB section */
    sectionNumber: section.readUInt8(4),
    /** Name of Grib section */
    sectionName: 'Product Definition Section',
    /** Length of GRIB section */
    length: section.readUInt32BE(0),
    /** Section 4 Contents */
    contents: {
      /** Number of coordinate values after template */
      coordinateValues: section.readUInt16BE(5),
      /** Product definition template number [Table 4.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml) */
      productDefinitionTemplate,
      /** Product definition */
      productDefinition,
    },
  };
}

/**
 * @param pds Product Definition Section
 * @param ins Indicator Section
 * @param ids
 * @returns Product Definition Section with corresponding string values
 */
export function lookupSection4(
  pds: ProductDefinitionSectionValues,
  ins: IndicatorSectionValues,
  ids: IdentificationSectionValues,
) {
  const { productDefinitionTemplate, productDefinition } = pds.contents;

  const { discipline } = ins.contents;
  const { refTime } = ids.contents;

  return {
    ...pds,
    contents: {
      ...pds.contents,
      /** Product definition template */
      productDefinitionTemplate: lookupTable40[productDefinitionTemplate],
      /** Product definition */
      productDefinition: lookupTemplate4(productDefinitionTemplate)(
        discipline,
        refTime,
        productDefinition,
      ),
    },
  };
}

//!!! SECTION 5

/** The output of `parseSection5` */
export type DataRepresentationSectionValues = ReturnType<typeof parseSection5>;
/** The output of `lookupSection5` */
export type DataRepresentationSection = ReturnType<typeof lookupSection5>;

/**
 *  Data Representation Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect5.shtml)
 * @param section
 */
export function parseSection5(section: Buffer) {
  const dataRepresentationTemplate = section.readUInt16BE(9);
  const dataRepresentation = getTemplate5(dataRepresentationTemplate)(section);

  return {
    /** Number of GRIB section */
    sectionNumber: section.readUInt8(4),
    /** Name of Grib section */
    sectionName: 'Data Representation Section',
    /** Length of GRIB section */
    length: section.readUInt32BE(0),
    /** Section 5 Contents */
    contents: {
      /** Number of data points where one or more values are specified in Section 7 when a bit map is present, total number of data points when a bit map is absent. */
      numberOfDataPoints: section.readUInt32BE(5),
      /** Data representation template number (See [Table 5.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml)) */
      dataRepresentationTemplate,
      /** Data representation */
      dataRepresentation,
    },
  };
}

/**
 * @param drs Data Representation Section
 * @returns Data Representation Section with corresponding string values
 */
export function lookupSection5(drs: DataRepresentationSectionValues) {
  const { dataRepresentationTemplate, dataRepresentation } = drs.contents;

  return {
    ...drs,
    contents: {
      ...drs.contents,
      /** Data representation template */
      dataRepresentationTemplate: lookupTable50[dataRepresentationTemplate],
      /** Data representation */
      dataRepresentation: lookupTemplate5(dataRepresentationTemplate)(dataRepresentation),
    },
  };
}

//!!! SECTION 6

/** The output of `parseSection6` */
export type BitMapSectionValues = ReturnType<typeof parseSection6>;
/** The output of `lookupSection6` */
export type BitMapSection = ReturnType<typeof parseSection6>;

/**
 *  Bit-Map Section
 *
 * Consult with [this page](https://confluence.ecmwf.int/display/UDOC/What+is+the+GRIB+bitmap+-+ecCodes+GRIB+FAQ) to understand their purpose.
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect6.shtml).
 * @param section
 */
export function parseSection6(section: Buffer) {
  const bitMapIndicator = section.readUInt8(5);

  if (![0, 255].includes(bitMapIndicator)) {
    throw new Error('BitMap Indicator not supported: ' + String(bitMapIndicator));
  }

  return {
    /** Number of GRIB section */
    sectionNumber: section.readUInt8(4),
    /** Name of Grib section */
    sectionName: 'Bit-Map Section',
    /** Length of GRIB section */
    length: section.readUInt32BE(0),
    /** Section 6 Contents */
    contents: {
      /** Bit-map indicator (See [Table 6.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table6-0.shtml)) */
      bitMapIndicator,
      // Bit-map
      bitMap: bitMapIndicator === 0 ? section.subarray(6) : null,
    },
  };
}

//!!! SECTION 7

/**
 *
 */
export type DataSectionValues = ReturnType<typeof parseSection7>;
/**
 *
 */
export type DataSection = ReturnType<typeof lookupSection7>;

/**
 *  Data Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect7.shtml)
 * @param section
 */
export function parseSection7(section: Buffer) {
  return {
    /** Number of GRIB section */
    sectionNumber: section.readUInt8(4),
    /** Name of Grib section */
    sectionName: 'Data Section',
    /** Length of GRIB section */
    length: section.readUInt32BE(0),
    /** Section 7 Contents */
    contents: { data: section.subarray(5) },
  };
}

/**
 * @param ds Data Section
 * @param drs Data Representation Section
 * @param bms
 * @returns Data Section with corresponding string values
 */
export function lookupSection7(
  ds: DataSectionValues,
  drs: DataRepresentationSectionValues,
  bms: BitMapSectionValues,
) {
  return {
    ...ds,
    contents: {
      data: convertData(drs, bms, ds.contents.data),
    },
  };
}

//!!! SECTION 8

/**
 *
 */
export type EndSectionValues = ReturnType<typeof parseSection8>;
/**
 *
 */
export type EndSection = ReturnType<typeof parseSection8>;

/**
 *  End Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect8.shtml)
 * @param section
 */
export function parseSection8(section: Buffer) {
  return {
    /** Name of Grib section */
    sectionName: 'End Section',
    /** Section 8 Contents */
    contents: {
      /** "7777" - Coded according to the International Alphabet Number 5 */
      endEncoded: section.toString(),
    },
  };
}
