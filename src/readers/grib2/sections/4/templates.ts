import { grib2LookupTableA } from '../other/tables.js';
import {
  grib2LookupTable41,
  grib2LookupTable42,
  grib2LookupTable43,
  grib2LookupTable44,
  grib2LookupTable45,
  grib2LookupTable46,
  grib2LookupTable47,
} from './tables.js';

import type { Grib2Sections } from '../index.js';
import type { Reader } from '../../../index.js';

/** The output of `getGrib2Template4` */
export type Grib2ProductDefinition = ReturnType<typeof getGrib2Template4>;

/**
 * Returns a template generator for the given template number
 *
 * See all templates [here](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)
 * @param template - the template number to generate
 * @param reader - the byte data to read
 * @param sections - the sections of the GRIB2 message that have been parsed so far
 * @returns - generated template data
 */
export function getGrib2Template4(template: number, reader: Reader, sections: Grib2Sections) {
  switch (template) {
    case 0:
      return grib2Template40(reader, sections);
    case 1:
      return grib2Template41(reader, sections);
    case 2:
      return grib2Template42(reader, sections);
    case 8:
      return grib2Template48(reader, sections);

    default:
      throw new Error(`Template 4.${template} not defined`);
  }
}

/**
 * PRODUCT DEFINITION TEMPLATE 4.0
 *
 * Analysis or forecast at a horizontal level or in
 * a horizontal layer at a point in time.
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-0.shtml)
 * @param section - the byte data to read
 * @param sections - the sections of the GRIB2 message that have been parsed so far
 * @returns - the parsed template
 */
export function grib2Template40(section: Reader, sections: Grib2Sections) {
  const discipline = sections.indicator?.discipline.code ?? 0;
  const refTime = sections.identification?.refTime ?? new Date();
  const parameterCategory = section.getUint8(9);
  const parameterNumber = section.getUint8(10);
  const genProcessType = section.getUint8(11);
  const backgroundGenProcess = section.getUint8(12);
  const forecastGenProcess = section.getUint8(13);
  const hoursAfterRefTime = section.getUint16(14);
  const minAfterRefTime = section.getUint8(16);
  const unitOfTimeRangeIndicator = section.getUint8(17);
  const forecastTime = section.getUint32(18);
  const surface1Type = section.getUint8(22);
  const surface1Scale = section.getUint8(23);
  const surface1Value = section.getUint32(24);
  const surface2Type = section.getUint8(28);
  const surface2Scale = section.getUint8(29);
  const surface2Value = section.getUint32(30);
  const category = grib2LookupTable41[discipline]?.[parameterCategory];
  const values = grib2LookupTable42[discipline]?.[parameterCategory]?.[parameterNumber];
  const surface1 = {
    ...grib2LookupTable45[surface1Type],
    scale: surface1Scale,
    value: surface1Value,
  };
  const surface2 = {
    ...grib2LookupTable45[surface2Type],
    scale: surface2Scale,
    value: surface2Value,
  };
  const unitOfTimeRangeIndicatorLookup = grib2LookupTable44[unitOfTimeRangeIndicator];

  return {
    /** Paramater */
    paramater: { category, ...values },
    /** Parameter category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)) */
    parameterCategory,
    /** Parameter number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)) */
    parameterNumber,
    /** Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)) */
    genProcessType: {
      code: genProcessType,
      value: grib2LookupTable43[genProcessType],
    },
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess,
    /** Analysis or forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: {
      code: forecastGenProcess,
      value: grib2LookupTableA[forecastGenProcess],
    },
    /** Hours after reference time data cutoff (see Notes) */
    hoursAfterRefTime,
    /** Minutes after reference time data cutoff (see Notes) */
    minAfterRefTime,
    /** Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)) */
    unitOfTimeRangeIndicator: {
      code: unitOfTimeRangeIndicator,
      value: unitOfTimeRangeIndicatorLookup,
    },
    /** Forecast time in units defined by octet 18 */
    forecastTime: {
      code: forecastTime,
      value: calculateForecastTime(refTime, forecastTime, unitOfTimeRangeIndicatorLookup),
    },
    /** First fixed surface */
    surface1,
    /** Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface1Type,
    /** Scale factor of first fixed surface */
    surface1Scale,
    /** Scaled value of first fixed surface */
    surface1Value,
    /** Second fixed surface */
    surface2,
    /** Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface2Type,
    /** Scale factor of second fixed surface */
    surface2Scale,
    /** Scaled value of second fixed surface */
    surface2Value,
  };
}

/**
 * PRODUCT DEFINITION TEMPLATE 4.1
 *
 * Individual ensemble forecast, control and perturbed, at a horizontal
 * level or in a horizontal layer at a point in time.
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1.shtml)
 * @param section - the byte data to read
 * @param sections - the sections of the GRIB2 message that have been parsed so far
 * @returns - the parsed template
 */
export function grib2Template41(section: Reader, sections: Grib2Sections) {
  const discipline = sections.indicator?.discipline.code ?? 0;
  const refTime = sections.identification?.refTime ?? new Date();
  const parameterCategory = section.getUint8(9);
  const parameterNumber = section.getUint8(10);
  const genProcessType = section.getUint8(11);
  const backgroundGenProcess = section.getUint8(12);
  const forecastGenProcess = section.getUint8(13);
  const hoursAfterRefTime = section.getUint16(14);
  const minAfterRefTime = section.getUint8(16);
  const unitOfTimeRangeIndicator = section.getUint8(17);
  const forecastTime = section.getUint32(18);
  const surface1Type = section.getUint8(22);
  const surface1Scale = section.getUint8(23);
  const surface1Value = section.getUint32(24);
  const surface2Type = section.getUint8(28);
  const surface2Scale = section.getUint8(29);
  const surface2Value = section.getUint32(30);
  const ensembleForecastType = section.getUint8(34);
  const perturbationNumber = section.getUint8(35);
  const numForecastsInEnsemble = section.getUint8(36);
  const category = grib2LookupTable41[discipline][parameterCategory];
  const values = grib2LookupTable42[discipline][parameterCategory][parameterNumber];
  const surface1 = {
    ...grib2LookupTable45[surface1Type],
    scale: surface1Scale,
    value: surface1Value,
  };
  const surface2 = {
    ...grib2LookupTable45[surface2Type],
    scale: surface2Scale,
    value: surface2Value,
  };
  const unitOfTimeRangeIndicatorLookup = grib2LookupTable44[unitOfTimeRangeIndicator];

  return {
    /** Paramater */
    paramater: { category, ...values },
    /** Parameter category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)) */
    parameterCategory,
    /** Parameter number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)) */
    parameterNumber,
    /** Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)) */
    genProcessType: {
      code: genProcessType,
      value: grib2LookupTable43[genProcessType],
    },
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess,
    /** Forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: {
      code: forecastGenProcess,
      value: grib2LookupTableA[forecastGenProcess],
    },
    /** Hours after reference time data cutoff (see Notes) */
    hoursAfterRefTime,
    /** Minutes after reference time data cutoff (see Notes) */
    minAfterRefTime,
    /** Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)) */
    unitOfTimeRangeIndicator: {
      code: unitOfTimeRangeIndicator,
      value: unitOfTimeRangeIndicatorLookup,
    },
    /** Forecast time in units defined by octet 18 */
    forecastTime: {
      code: forecastTime,
      value: calculateForecastTime(refTime, forecastTime, unitOfTimeRangeIndicatorLookup),
    },
    /** First fixed surface */
    surface1,
    /** Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml), result stored in `surface1`) */
    surface1Type,
    /** Scale factor of first fixed surface */
    surface1Scale,
    /** Scaled value of first fixed surface */
    surface1Value,
    /** Second fixed surface */
    surface2,
    /** Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml) result stored in `surface2`) */
    surface2Type,
    /** Scale factor of second fixed surface */
    surface2Scale,
    /** Scaled value of second fixed surface */
    surface2Value,
    /** Type of ensemble forecast (see Code [Table 4.6](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-6.shtml)) */
    ensembleForecastType: {
      code: ensembleForecastType,
      value: grib2LookupTable46[ensembleForecastType],
    },
    /** Perturbation number */
    perturbationNumber,
    /** Number of forecasts in ensemble */
    numForecastsInEnsemble,
  };
}

/**
 * PRODUCT DEFINITION TEMPLATE 4.2
 *
 * Derived forecast, based on all ensemble members at a horizontal
 * level or in a horizontal layer at a point in time.
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-2.shtml)
 * @param section - the byte data to read
 * @param sections - the sections of the GRIB2 message that have been parsed so far
 * @returns - the parsed template
 */
export function grib2Template42(section: Reader, sections: Grib2Sections) {
  const discipline = sections.indicator?.discipline.code ?? 0;
  const refTime = sections.identification?.refTime ?? new Date();
  const parameterCategory = section.getUint8(9);
  const parameterNumber = section.getUint8(10);
  const genProcessType = section.getUint8(11);
  const backgroundGenProcess = section.getUint8(12);
  const forecastGenProcess = section.getUint8(13);
  const hoursAfterRefTime = section.getUint16(14);
  const minAfterRefTime = section.getUint8(16);
  const unitOfTimeRangeIndicator = section.getUint8(17);
  const forecastTime = section.getUint32(18);
  const surface1Type = section.getUint8(22);
  const surface1Scale = section.getUint8(23);
  const surface1Value = section.getUint32(24);
  const surface2Type = section.getUint8(28);
  const surface2Scale = section.getUint8(29);
  const surface2Value = section.getUint32(30);
  const derivedForecastType = section.getUint8(34);
  const numForecastsInEnsemble = section.getUint8(35);
  const category = grib2LookupTable41[discipline][parameterCategory];
  const values = grib2LookupTable42[discipline][parameterCategory][parameterNumber];
  const surface1 = {
    ...grib2LookupTable45[surface1Type],
    scale: surface1Scale,
    value: surface1Value,
  };
  const surface2 = {
    ...grib2LookupTable45[surface2Type],
    scale: surface2Scale,
    value: surface2Value,
  };
  const unitOfTimeRangeIndicatorLookup = grib2LookupTable44[unitOfTimeRangeIndicator];

  return {
    /** Paramater */
    paramater: { category, ...values },
    /** Parameter category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)) */
    parameterCategory,
    /** Parameter number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)) */
    parameterNumber,
    /** Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)) */
    genProcessType: {
      code: genProcessType,
      value: grib2LookupTable43[genProcessType],
    },
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess,
    /** Forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: {
      code: forecastGenProcess,
      value: grib2LookupTableA[forecastGenProcess],
    },
    /** Hours after reference time data cutoff (see Notes) */
    hoursAfterRefTime,
    /** Minutes after reference time data cutoff */
    minAfterRefTime,
    /** Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)) */
    unitOfTimeRangeIndicator: {
      code: unitOfTimeRangeIndicator,
      value: unitOfTimeRangeIndicatorLookup,
    },
    /** Forecast time in units defined by octet 18 */
    forecastTime: {
      code: forecastTime,
      value: calculateForecastTime(refTime, forecastTime, unitOfTimeRangeIndicatorLookup),
    },
    /** First fixed surface */
    surface1,
    /** Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface1Type,
    /** Scale factor of first fixed surface */
    surface1Scale,
    /** Scaled value of first fixed surface */
    surface1Value,
    /** Second fixed surface */
    surface2,
    /** Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface2Type,
    /** Scale factor of second fixed surface */
    surface2Scale,
    /** Scaled value of second fixed surface */
    surface2Value,
    /** Derived forecast type (see Code [Table 4.7](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-7.shtml)) */
    derivedForecastType: {
      code: derivedForecastType,
      value: grib2LookupTable47[derivedForecastType],
    },
    /** Number of forecasts in the ensemble */
    numForecastsInEnsemble,
  };
}

/**
 * PRODUCT DEFINITION TEMPLATE 4.8
 *
 * Average, Accumulation and/or Extreme values or other Statistically-processed values at a
 * horizontal level or in a horizontal layer in a continuous or non-continuous time interval
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml)
 * @param section - the byte data to read
 * @param sections - the sections of the GRIB2 message that have been parsed so far
 * @returns - the parsed template
 */
export function grib2Template48(section: Reader, sections: Grib2Sections) {
  const discipline = sections.indicator?.discipline.code ?? 0;
  const refTime = sections.identification?.refTime ?? new Date();
  const parameterCategory = section.getUint8(9);
  const parameterNumber = section.getUint8(10);
  const genProcessType = section.getUint8(11);
  const backgroundGenProcess = section.getUint8(12);
  const forecastGenProcess = section.getUint8(13);
  const hoursAfterRefTime = section.getUint16(14);
  const minAfterRefTime = section.getUint8(16);
  const unitOfTimeRangeIndicator = section.getUint8(17);
  const forecastTime = section.getUint32(18);
  const surface1Type = section.getUint8(22);
  const surface1Scale = section.getUint8(23);
  const surface1Value = section.getUint32(24);
  const surface2Type = section.getUint8(28);
  const surface2Scale = section.getUint8(29);
  const surface2Value = section.getUint32(30);
  // time
  const year = section.getUint16(34);
  const month = section.getUint8(36);
  const day = section.getUint8(37);
  const hour = section.getUint8(38);
  const minute = section.getUint8(39);
  const second = section.getUint8(40);
  // TODO: 41 onward

  const category = grib2LookupTable41[discipline][parameterCategory];
  const values = grib2LookupTable42[discipline][parameterCategory][parameterNumber];
  const surface1 = {
    ...grib2LookupTable45[surface1Type],
    scale: surface1Scale,
    value: surface1Value,
  };
  const surface2 = {
    ...grib2LookupTable45[surface2Type],
    scale: surface2Scale,
    value: surface2Value,
  };
  const unitOfTimeRangeIndicatorLookup = grib2LookupTable44[unitOfTimeRangeIndicator];

  return {
    /** Paramater */
    paramater: { category, ...values },
    /** Parameter category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)) */
    parameterCategory,
    /** Parameter number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)) */
    parameterNumber,
    /** Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)) */
    genProcessType: {
      code: genProcessType,
      value: grib2LookupTable43[genProcessType],
    },
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess,
    /** Forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: {
      code: forecastGenProcess,
      value: grib2LookupTableA[forecastGenProcess],
    },
    /** Hours after reference time data cutoff (see Notes) */
    hoursAfterRefTime,
    /** Minutes after reference time data cutoff */
    minAfterRefTime,
    /** Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)) */
    unitOfTimeRangeIndicator: {
      code: unitOfTimeRangeIndicator,
      value: unitOfTimeRangeIndicatorLookup,
    },
    /** Forecast time in units defined by octet 18 */
    forecastTime: {
      code: forecastTime,
      value: calculateForecastTime(refTime, forecastTime, unitOfTimeRangeIndicatorLookup),
    },
    /** First fixed surface */
    surface1,
    /** Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface1Type,
    /** Scale factor of first fixed surface */
    surface1Scale,
    /** Scaled value of first fixed surface */
    surface1Value,
    /** Second fixed surface */
    surface2,
    /** Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface2Type,
    /** Scale factor of second fixed surface */
    surface2Scale,
    /** Scaled value of second fixed surface */
    surface2Value,
    /** Year - Time of end of overall time interval */
    year,
    /** Month - Time of end of overall time interval */
    month,
    /** Day - Time of end of overall time interval */
    day,
    /** Hour - Time of end of overall time interval */
    hour,
    /** Minute - Time of end of overall time interval */
    minute,
    /** Second - Time of end of overall time interval */
    second,
    /** Set the time */
    time: new Date(year, month - 1, day, hour, minute, second),
  };
}

/**
 * Calculate Forecast Time
 * @param refTime Reference time of GRIB Packet
 * @param offset Number of units to offset the ref time by
 * @param unitOfTime unit of time of offset
 * @returns - the forecast time
 */
export function calculateForecastTime(refTime: Date, offset: number, unitOfTime: string) {
  switch (unitOfTime) {
    case 'Hour':
      return new Date(refTime.getTime() + offset * 1000 * 60 * 60);
    case 'Minute':
      return new Date(refTime.getTime() + offset * 1000 * 60);
    case 'Second':
      return new Date(refTime.getTime() + offset * 1000);
    default:
      console.warn(`Unable to calculate forecast time for unit: ${unitOfTime}`);
      return refTime;
  }
}
