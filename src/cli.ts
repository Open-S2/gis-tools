#!/usr/bin/env node
import { FileWriter } from './writers/file.js';
import { fileTypeToReader } from './file.js';
import { hideBin } from 'yargs/helpers';
import yargs from 'yargs';
import { toJSON, toJSONLD } from './writers/index.js';

import type { CommandModule } from 'yargs';

/** Arguments used by the `convert` command */
interface ConvertArgs {
  input: string;
  output: string;
  inputFormat?: string;
  outputFormat?: string;
  verbose?: boolean;
}

// TODO:
// - support flags:
// - - makevalid (optional)
// - - filter (optional)
// - - inputFileType (optional)
// - - outputFileType (optional)
// - - inputProjection (optional)
// - - outputProjection (optional)

const commandConvert: CommandModule<object, ConvertArgs> = {
  command: 'convert <input>',
  describe: 'Convert input file to another file format',
  /**
   * Parse the input arguments
   * @param yargs - yargs instance to configure
   * @returns - resultant arguments
   */
  builder: (yargs) =>
    yargs
      .example('$0 convert -i input.gpx -o output.geojson', 'Convert a GPX file to a GeoJSON file')
      .option('input', {
        alias: 'i',
        type: 'string',
        describe: 'Input file path',
        nargs: 1,
        demandOption: true,
      })
      .option('inputFormat', {
        alias: 'f',
        type: 'string',
        describe: 'Input file type',
        nargs: 1,
        choices: [
          'csv',
          'tif',
          'tiff',
          'geotif',
          'geotiff',
          'gpx',
          'grib',
          'grib2',
          'gtfs',
          'json',
          'geojson',
          's2json',
          'jsonld',
          'geojsonld',
          's2jsonld',
          'json-ld',
          'geojson-ld',
          's2json-ld',
          'jsonsq',
          'geojsonsq',
          's2jsonsq',
          'json-sq',
          'geojson-sq',
          's2json-sq',
          'las',
          'laz',
          'nc4',
          'cdf',
          'nc',
          'netcdf',
          'osm',
          'raster',
          'shapefile',
          'wkt',
        ],
        demandOption: true,
      })
      .option('output', {
        alias: 'o',
        type: 'string',
        describe: 'Output file path',
        nargs: 1,
        demandOption: true,
      })
      .option('outputFormat', {
        alias: 'of',
        type: 'string',
        describe: 'Output file type',
        choices: [
          'json',
          'geojson',
          's2json',
          'jsonld',
          'geojsonld',
          's2jsonld',
          'json-ld',
          'geojson-ld',
          's2json-ld',
        ],
        nargs: 1,
        demandOption: true,
      })
      .option('verbose', {
        alias: 'v',
        type: 'boolean',
        default: false,
        describe: 'Enable verbose output',
      })
      .option('append', {
        alias: 'a',
        type: 'boolean',
        default: false,
        describe: 'Append to output file',
      }),

  /**
   * Handle the input arguments and do something with them
   * @param argv - parsed arguments
   */
  async handler(argv): Promise<void> {
    const { input, output, inputFormat, verbose, outputFormat } = argv;

    if (verbose === true) console.info(`Converting: "${input}"`);

    // prep the reader and writer
    const readerIterator = await fileTypeToReader(input, inputFormat);
    const writer = new FileWriter(`${__dirname}/fixtures/points.geojsonld`);
    // find the output format
    const outFormat = (outputFormat ?? inputFormat?.split('.').pop() ?? '').toLowerCase();

    // Given the format type, write appropriately
    if (['json', 's2json'].includes(outFormat)) {
      await toJSON(writer, [readerIterator], { geojson: false });
    } else if (outFormat === 'geojson') {
      await toJSON(writer, [readerIterator], { geojson: true });
    } else if (['jsonld', 's2jsonld', 'json-ld', 's2json-ld'].includes(outFormat)) {
      await toJSONLD(writer, [readerIterator], { geojson: false });
    } else if (['geojsonld', 'geojson-ld'].includes(outFormat)) {
      await toJSONLD(writer, [readerIterator], { geojson: true });
    } else {
      throw new Error(`Unknown output format: ${outFormat}`);
    }

    // output the result
    if (verbose === true) console.info(`Output written to "${output}"`);
  },
};

// TODO:
// - convert
// - convert-many
// - tileize
// - OSM PBF parsing directly

void yargs(hideBin(process.argv))
  .scriptName('gis-tools-ts')
  .usage('Usage: $0 <command> [options]')
  /** Convert command START */
  .command(commandConvert)
  .demandCommand(1, 'You must specify a command')
  /** Convert command END */
  .strict()
  .help('h')
  .alias('h', 'help')
  .epilog(
    'Found an issue? Please open an issue on GitHub at:\nhttps://github.com/Open-S2/gis-tools/issues',
  )
  .parse();
