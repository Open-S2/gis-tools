import {
  lookupTable32,
  lookupTable41,
  lookupTable42,
  lookupTable43,
  lookupTable44,
  lookupTable45,
  lookupTable47,
  lookupTable51,
  lookupTable540,
  lookupTableA,
} from './tables';

//!!! TEMPLATE 3

/**
 * @description Returns a template generator for the given template number
 * @param template Template number
 * @returns Template generator
 */
export function getTemplate3(template: number) {
  switch (template) {
    case 0:
      return template30;
    default:
      throw new Error(`Template 3.${template} not defined`);
  }
}

/**
 * @description Returns a template map for the given template number
 * @param template
 * @param table Template number
 * @returns Template Map
 */
export function lookupTemplate3(template: number) {
  switch (template) {
    case 0:
      return lookupTemplate30;

    default:
      throw new Error(`Template 3.${template} lookup table not defined`);
  }
}

/**
 * Grid Definition Template 3.0
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-0.shtml)
 * @param section
 */
function template30(section: Buffer) {
  const basicAngle = section.readUInt32BE(38);
  const subdivisions = section.readUInt32BE(42);

  const ratio = basicAngle === 0 ? 1e-6 : basicAngle / subdivisions;

  return {
    /** Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml) */
    shape: section.readUInt8(14),
    /** Number of points along a parallel (W-E) */
    ny: section.readUInt32BE(30),
    /** Number of points along a meridian (N-S) */
    nx: section.readUInt32BE(34),
    /** Basic angle of the initial production domain */
    basicAngle,
    /** Subdivisions of basic angle used to define extreme longitudes and latitudes, and direction increments */
    subdivisions,
    /** Latitude of first grid point */
    la1:
      (section.readInt32BE(46) < 0
        ? -(section.readInt32BE(46) ^ 0x80000000)
        : section.readInt32BE(46)) * ratio,
    /** Longitude of first grid point */
    lo1: section.readInt32BE(50) * ratio,
    /** Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml) */
    resolution: section.readUInt8(54),
    /** Latitude of last grid point */
    la2:
      (section.readInt32BE(55) < 0
        ? -(section.readInt32BE(55) ^ 0x80000000)
        : section.readInt32BE(55)) * ratio,
    /** Longitude of last grid point */
    lo2: section.readInt32BE(59) * ratio,
    /** i direction increment */
    dx: section.readInt32BE(63) * ratio,
    /** j direction increment */
    dy: section.readInt32BE(67) * ratio,
    /** Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml) */
    scanMode: section.readUInt8(71),
    /** Grid Units */
    gridUnits: 'degrees',
  };
}

/**
 * @param templateValues
 */
const lookupTemplate30 = (templateValues: ReturnType<typeof template30>) => {
  return {
    ...templateValues,
    /** Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml) */
    shape: lookupTable32[templateValues.shape],
  };
};

//!!! TEMPLATE 4

/**
 * @description Returns a template generator for the given template number
 * @param template Template number
 * @returns Template generator
 */
export function getTemplate4(template: number) {
  switch (template) {
    case 0:
      return template40;
    case 2:
      return template42;

    default:
      throw new Error(`Template 4.${template} not defined`);
  }
}

/**
 * @description Returns a template map for the given template number
 * @param template
 * @param table Template number
 * @returns Template Map
 */
export function lookupTemplate4(template: number) {
  switch (template) {
    case 0:
      return lookupTemplate40;
    case 2:
      return lookupTemplate42;

    default:
      throw new Error(`Template 4.${template} lookup table not defined`);
  }
}

/**
 * Product Definition Template 4.0
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-0.shtml)
 * @param section
 */
function template40(section: Buffer) {
  return {
    /** Paramater category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)) */
    parameterCategory: section.readUInt8(9),
    /** Paramater number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)) */
    parameterNumber: section.readUInt8(10),
    /** Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)) */
    genProcessType: section.readUInt8(11),
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess: section.readUInt8(12),
    /** Forecast generating process identified (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: section.readUInt8(13),
    /** Hours after reference time data cutoff */
    hoursAfterRefTime: section.readUInt16BE(14),
    /** Minutes after reference time data cutoff */
    minAfterRefTime: section.readUInt8(16),
    /** Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)) */
    unitOfTimeRangeIndicator: section.readUInt8(17),
    /** Forecast time offset */
    forecastTimeOffset: section.readUInt32BE(18),
    /** Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface1Type: section.readUInt8(22),
    /** Scale factor of first fixed surface */
    surface1Scale: section.readUInt8(23),
    /** Scaled value of first fixed surface */
    surface1Value: section.readUInt32BE(24),
    /** Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface2Type: section.readUInt8(28),
    /** Scale factor of second fixed surface */
    surface2Scale: section.readUInt8(29),
    /** Scaled value of second fixed surface */
    surface2Value: section.readUInt32BE(30),
  };
}

/**
 * @param discipline
 * @param refTime
 * @param templateValues
 */
function lookupTemplate40(
  discipline: number,
  refTime: Date,
  templateValues: ReturnType<typeof template40> | ReturnType<typeof template42>,
) {
  const {
    parameterCategory,
    parameterNumber,
    surface1Type,
    surface1Scale,
    surface1Value,
    surface2Type,
    surface2Scale,
    surface2Value,
    ...newValues
  } = templateValues;

  const category = lookupTable41[discipline][parameterCategory];
  const values = lookupTable42[discipline][parameterCategory][parameterNumber];

  const surface1 = { ...lookupTable45[surface1Type], scale: surface1Scale, value: surface1Value };
  const surface2 = { ...lookupTable45[surface2Type], scale: surface2Scale, value: surface2Value };

  const unitOfTimeRangeIndicator = lookupTable44[newValues.unitOfTimeRangeIndicator];

  return {
    ...newValues,
    /** Paramater */
    paramater: { category, ...values },
    /** Type of generating process */
    genProcessType: lookupTable43[newValues.genProcessType],
    /** Forecast generating process identified */
    forecastGenProcess: lookupTableA[newValues.forecastGenProcess],
    /** Indicator of unit of time range */
    unitOfTimeRangeIndicator,
    /** Forecast time */
    forecastTime: calculateForecastTime(
      refTime,
      newValues.forecastTimeOffset,
      unitOfTimeRangeIndicator,
    ),
    /** First fixed surface */
    surface1,
    /** Second fixed surface */
    surface2,
  };
}

/**
 * Product Definition Template 4.2
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-2.shtml)
 * @param section
 */
function template42(section: Buffer) {
  return {
    /** Paramater category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)) */
    parameterCategory: section.readUInt8(9),
    /** Paramater number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)) */
    parameterNumber: section.readUInt8(10),
    /** Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)) */
    genProcessType: section.readUInt8(11),
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess: section.readUInt8(12),
    /** Forecast generating process identified (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: section.readUInt8(13),
    /** Hours after reference time data cutoff */
    hoursAfterRefTime: section.readUInt16BE(14),
    /** Minutes after reference time data cutoff */
    minAfterRefTime: section.readUInt8(16),
    /** Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)) */
    unitOfTimeRangeIndicator: section.readUInt8(17),
    /** Forecast time offset */
    forecastTimeOffset: section.readUInt32BE(18),
    /** Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface1Type: section.readUInt8(22),
    /** Scale factor of first fixed surface */
    surface1Scale: section.readUInt8(23),
    /** Scaled value of first fixed surface */
    surface1Value: section.readUInt32BE(24),
    /** Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface2Type: section.readUInt8(28),
    /** Scale factor of second fixed surface */
    surface2Scale: section.readUInt8(29),
    /** Scaled value of second fixed surface */
    surface2Value: section.readUInt32BE(30),
    /** Derived forecast (see Code [Table 4.7](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-7.shtml)) */
    derivedForecast: section.readUInt8(34),
    /** Number of forecasts in the ensemble */
    numberOfForecast: section.readUInt8(35),
  };
}

/**
 * @param discipline
 * @param refTime
 * @param templateValues
 */
function lookupTemplate42(
  discipline: number,
  refTime: Date,
  templateValues: ReturnType<typeof template40> | ReturnType<typeof template42>,
) {
  const {
    parameterCategory,
    parameterNumber,
    surface1Type,
    surface1Scale,
    surface1Value,
    surface2Type,
    surface2Scale,
    surface2Value,
    ...newValues
  } = templateValues;

  const category = lookupTable41[discipline][parameterCategory];
  const values = lookupTable42[discipline][parameterCategory][parameterNumber];

  const surface1 = { ...lookupTable45[surface1Type], scale: surface1Scale, value: surface1Value };
  const surface2 = { ...lookupTable45[surface2Type], scale: surface2Scale, value: surface2Value };

  const unitOfTimeRangeIndicator = lookupTable44[newValues.unitOfTimeRangeIndicator];

  return {
    ...newValues,
    /** Paramater */
    paramater: { category, ...values },
    /** Type of generating process */
    genProcessType: lookupTable43[newValues.genProcessType],
    /** Forecast generating process identified */
    forecastGenProcess: lookupTableA[newValues.forecastGenProcess],
    /** Indicator of unit of time range */
    unitOfTimeRangeIndicator,
    /** Forecast time */
    forecastTime: calculateForecastTime(
      refTime,
      newValues.forecastTimeOffset,
      unitOfTimeRangeIndicator,
    ),
    /** First fixed surface */
    surface1,
    /** Second fixed surface */
    surface2,
    /** Derived forecast */
    derivedForecast: lookupTable47['derivedForecast' in newValues ? newValues.derivedForecast : 0],
  };
}

//!!! TEMPLATE 5

/**
 * @description Returns a template generator for the given template number
 * @param template Template number
 * @returns Template generator
 */
export function getTemplate5(template: number) {
  switch (template) {
    case 0:
      return template50;
    case 40:
      return template540;
    default:
      throw new Error(`Template 5.${template} not defined`);
  }
}

/**
 * @description Returns a template map for the given template number
 * @param template
 * @param table Template number
 * @returns Template Map
 */
export function lookupTemplate5(template: number) {
  switch (template) {
    case 0:
      return lookupTemplate50;
    case 40:
      return lookupTemplate540;
    default:
      throw new Error(`Template 5.${template} lookup table not defined`);
  }
}

/**
 * Data Representation Template 5.0
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-0.shtml)
 * @param section
 */
function template50(section: Buffer) {
  return {
    /** Reference value (R) (IEEE 32-bit floating-point value) */
    referenceValue: section.readFloatBE(11),
    /** Binary scale factor (E) */
    binaryScaleFactor: section.readInt16BE(15),
    /** Decimal scale factor (D) */
    decimalScaleFactor: section.readInt16BE(17),
    /** Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing */
    numberOfBits: section.readUInt8(19),
    /** Type of original field values (see Code [Table 5.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-1.shtml)) */
    originalType: section.readUInt8(20),
  };
}

/**
 * Build a lookup template for 5.0.
 * @param templateValues - a parsed template 5.0.
 * @returns - a template 5.0. lookup
 */
function lookupTemplate50(
  templateValues: ReturnType<typeof template50> | ReturnType<typeof template540>,
) {
  return {
    ...templateValues,
    /** Type of original field values */
    originalType: lookupTable51[templateValues.originalType],
  };
}

/**
 * Data Representation Template 5.40
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-40.shtml)
 * @param section
 */
function template540(section: Buffer) {
  return {
    /** Reference value (R) (IEEE 32-bit floating-point value) */
    referenceValue: section.readFloatBE(11),
    /** Binary scale factor (E) */
    binaryScaleFactor: section.readInt16BE(15),
    /** Decimal scale factor (D) */
    decimalScaleFactor: section.readInt16BE(17),
    /** Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing */
    numberOfBits: section.readUInt8(19),
    /** Type of original field values (see Code [Table 5.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-1.shtml)) */
    originalType: section.readUInt8(20),
    // Type of Compression used. (see [Code Table 5.40](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-40.shtml))
    compressionType: section.readUInt8(21),
    // Target compression ratio, M:1 (with respect to the bit-depth specified in octet 20),
    // when octet 22 indicates Lossy Compression. Otherwise, set to missing.
    compressionRatio: section.readUInt8(22),
  };
}

/**
 * @param templateValues
 */
function lookupTemplate540(
  templateValues: ReturnType<typeof template50> | ReturnType<typeof template540>,
) {
  return {
    ...lookupTemplate50(templateValues),
    // Type of Compression used
    compressionType:
      lookupTable540['compressionType' in templateValues ? templateValues.compressionType : 255],
  };
}

/**
 * Calculate Forecast Time
 * @param refTime Reference time of GRIB Packet
 * @param offset Number of units to offset the ref time by
 * @param unitOfTime unit of time of offset
 */
export function calculateForecastTime(refTime: Date, offset: number, unitOfTime: string) {
  switch (unitOfTime) {
    case 'Hour':
      return new Date(refTime.getTime() + offset * 1000 * 60 * 60);

    default:
      throw new Error(`Unable to calculate foercast time for unit: ${unitOfTime}`);
  }
}
