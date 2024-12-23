import { lookupTableA } from '../other/tables';
import {
  lookupTable41,
  lookupTable42,
  lookupTable43,
  lookupTable44,
  lookupTable45,
  lookupTable46,
  lookupTable47,
} from './tables';

import type { Reader } from '../../..';
import type { Sections } from '..';

/** The output of `getTemplate4` */
export type ProductDefinition = ReturnType<typeof getTemplate4>;

/**
 * Returns a template generator for the given template number
 * @param template - the template number to generate
 * @param reader - the byte data to read
 * @param sections - the sections of the GRIB2 message that have been parsed so far
 * @returns - generated template data
 */
export function getTemplate4(template: number, reader: Reader, sections: Sections) {
  switch (template) {
    case 0:
      return template40(reader, sections);
    case 1:
      return template41(reader, sections);
    case 2:
      return template42(reader, sections);

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
function template40(section: Reader, sections: Sections) {
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
  const category = lookupTable41[discipline][parameterCategory];
  const values = lookupTable42[discipline][parameterCategory][parameterNumber];
  const surface1 = { ...lookupTable45[surface1Type], scale: surface1Scale, value: surface1Value };
  const surface2 = { ...lookupTable45[surface2Type], scale: surface2Scale, value: surface2Value };
  const unitOfTimeRangeIndicatorLookup = lookupTable44[unitOfTimeRangeIndicator];

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
      value: lookupTable43[genProcessType],
    },
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess,
    /** Analysis or forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: {
      code: forecastGenProcess,
      value: lookupTableA[forecastGenProcess],
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
function template41(section: Reader, sections: Sections) {
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
  const category = lookupTable41[discipline][parameterCategory];
  const values = lookupTable42[discipline][parameterCategory][parameterNumber];
  const surface1 = { ...lookupTable45[surface1Type], scale: surface1Scale, value: surface1Value };
  const surface2 = { ...lookupTable45[surface2Type], scale: surface2Scale, value: surface2Value };
  const unitOfTimeRangeIndicatorLookup = lookupTable44[unitOfTimeRangeIndicator];

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
      value: lookupTable43[genProcessType],
    },
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess,
    /** Forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: {
      code: forecastGenProcess,
      value: lookupTableA[forecastGenProcess],
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
      value: lookupTable46[ensembleForecastType],
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
function template42(section: Reader, sections: Sections) {
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
  const category = lookupTable41[discipline][parameterCategory];
  const values = lookupTable42[discipline][parameterCategory][parameterNumber];
  const surface1 = { ...lookupTable45[surface1Type], scale: surface1Scale, value: surface1Value };
  const surface2 = { ...lookupTable45[surface2Type], scale: surface2Scale, value: surface2Value };
  const unitOfTimeRangeIndicatorLookup = lookupTable44[unitOfTimeRangeIndicator];

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
      value: lookupTable43[genProcessType],
    },
    /** Background generating process identifier (defined by originating centre) */
    backgroundGenProcess,
    /** Forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecastGenProcess: {
      code: forecastGenProcess,
      value: lookupTableA[forecastGenProcess],
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
      value: lookupTable47[derivedForecastType],
    },
    /** Number of forecasts in the ensemble */
    numForecastsInEnsemble,
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

    default:
      throw new Error(`Unable to calculate foercast time for unit: ${unitOfTime}`);
  }
}
