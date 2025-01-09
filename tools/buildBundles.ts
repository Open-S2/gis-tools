import { build } from 'esbuild';
import { makeBadge } from 'badge-maker';
import { brotliCompressSync, gzipSync } from 'node:zlib';
import { mkdir, readdir, rm, stat } from 'node:fs/promises';

await mkdir(`${__dirname}/../build_bundle_tmp`, { recursive: true });

await build({
  bundle: true,
  platform: 'browser', // Treat Node.js built-in modules as external
  external: ['os', 'path', 'fs', 'node:zlib', 'util', 'node:*', 'child_process', 'sharp'], // Exclude built-in and other unwanted modules
  format: 'esm', // ESM output format
  treeShaking: true, // Ensure unused code is removed
  entryPoints: [
    // CONSTANTS
    `${__dirname}/builds/constants/index.ts`,
    // CONVERTERS
    `${__dirname}/builds/converters/toJSON.ts`,
    `${__dirname}/builds/converters/toTiles.ts`,
    // DATA STORES
    `${__dirname}/builds/dataStore/externalSort.ts`,
    `${__dirname}/builds/dataStore/kd.ts`,
    `${__dirname}/builds/dataStore/kv.ts`,
    `${__dirname}/builds/dataStore/multimap.ts`,
    `${__dirname}/builds/dataStore/vector.ts`,
    // DATA STRUCTURES
    `${__dirname}/builds/dataStructures/cache.ts`,
    `${__dirname}/builds/dataStructures/pointCluster.ts`,
    `${__dirname}/builds/dataStructures/pointIndex.ts`,
    `${__dirname}/builds/dataStructures/pointIndexFast.ts`,
    `${__dirname}/builds/dataStructures/priorityQueue.ts`,
    `${__dirname}/builds/dataStructures/dataTile.ts`,
    // GEOMETRY
    `${__dirname}/builds/geometry/angles.ts`,
    `${__dirname}/builds/geometry/bbox.ts`,
    `${__dirname}/builds/geometry/id.ts`,
    `${__dirname}/builds/geometry/lonlat.ts`,
    `${__dirname}/builds/geometry/planets.ts`,
    `${__dirname}/builds/geometry/predicates.ts`,
    `${__dirname}/builds/geometry/s2.ts`,
    `${__dirname}/builds/geometry/tools.ts`,
    `${__dirname}/builds/geometry/wm.ts`,
    // PROJ4
    `${__dirname}/builds/proj4/datum.ts`,
    `${__dirname}/builds/proj4/mgrs.ts`,
    `${__dirname}/builds/proj4/projections.ts`,
    `${__dirname}/builds/proj4/transformer.ts`,
    // READERS
    `${__dirname}/builds/readers/csv.ts`,
    `${__dirname}/builds/readers/gbfs.ts`,
    `${__dirname}/builds/readers/geotiff.ts`,
    `${__dirname}/builds/readers/grib2.ts`,
    `${__dirname}/builds/readers/gtfs.ts`,
    `${__dirname}/builds/readers/jpeg.ts`,
    `${__dirname}/builds/readers/jpeg2000.ts`,
    `${__dirname}/builds/readers/json.ts`,
    `${__dirname}/builds/readers/lanczos.ts`,
    `${__dirname}/builds/readers/nadgrid.ts`,
    `${__dirname}/builds/readers/netcdf.ts`,
    `${__dirname}/builds/readers/osm.ts`,
    `${__dirname}/builds/readers/pmtilesReader.ts`,
    `${__dirname}/builds/readers/protobuf.ts`,
    `${__dirname}/builds/readers/shapefile.ts`,
    `${__dirname}/builds/readers/tileReader.ts`,
    `${__dirname}/builds/readers/wkt.ts`,
    `${__dirname}/builds/readers/xml.ts`,
    // SPACE
    `${__dirname}/builds/space/sat.ts`,
    // TOOLS
    `${__dirname}/builds/tools/delaunator.ts`,
    `${__dirname}/builds/tools/interpolators.ts`,
    `${__dirname}/builds/tools/orthodrome.ts`,
    `${__dirname}/builds/tools/polylabel.ts`,
    // UTIL
    `${__dirname}/builds/util/gzip.ts`,
    `${__dirname}/builds/util/lzw.ts`,
    `${__dirname}/builds/util/polyfills.ts`,
    // WRITERS
    `${__dirname}/builds/writers/pmtilesWriter.ts`,
    `${__dirname}/builds/writers/tileWriter.ts`,
  ],
  // splitting: true,
  outdir: `${__dirname}/../build_bundle_tmp`,
  minify: true, // Minify the output
  legalComments: 'none', // Remove all comments, including licenses
});

// Gzip and Brotli compress all files in `build_bundle_tmp`
for (const folder of await readdir(`${__dirname}/../build_bundle_tmp`)) {
  for (const file of await readdir(`${__dirname}/../build_bundle_tmp/${folder}`)) {
    const name = file.split('.')[0];
    const filePath = `${__dirname}/../build_bundle_tmp/${folder}/${file}`;
    const stats = await stat(filePath);

    // Skip directories
    if (stats.isFile()) {
      const data = await Bun.file(filePath).arrayBuffer();
      const fileBuffer = Buffer.from(data);
      // Gzip compression
      const gzip = gzipSync(fileBuffer);
      // Brotli compression
      const brotli = brotliCompressSync(fileBuffer);

      const fileSize = formatBytes(fileBuffer.byteLength);
      const gzipSize = formatBytes(gzip.byteLength);
      const brotliSize = formatBytes(brotli.byteLength);

      console.info(`Compressed ${name}: ${fileSize} → ${gzipSize}, ${brotliSize}`);
      const svgFile = makeBadge({
        label: 'bundle',
        message: fileSize,
        color: 'yellow',
      });
      const svgGzip = makeBadge({
        label: 'gzip',
        message: gzipSize,
        color: 'orange',
      });
      const svgGzipCover = makeBadge({
        label: 'gzip',
        message: gzipSize,
        color: 'blue',
      });
      const svgBrotli = makeBadge({
        label: 'brotli',
        message: brotliSize,
        color: 'red',
      });
      await Bun.write(`${__dirname}/../assets/badges/${name}-file.svg`, svgFile);
      await Bun.write(`${__dirname}/../assets/badges/${name}-gzip.svg`, svgGzip);
      await Bun.write(`${__dirname}/../assets/badges/${name}-gzip-cover.svg`, svgGzipCover);
      await Bun.write(`${__dirname}/../assets/badges/${name}-brotli.svg`, svgBrotli);
    }
  }
}

await rm(`${__dirname}/../build_bundle_tmp`, { recursive: true, force: true });

/**
 * Convert bytes to a human-readable size string.
 * @param bytes - The size in bytes.
 * @returns The size formatted as a string with an appropriate unit (e.g., "12.2 kB").
 */
function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes';

  const units = ['Bytes', 'kB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  const factor = Math.floor(Math.log10(bytes) / 3);
  const size = bytes / Math.pow(1000, factor);

  return `${size} ${units[factor]}`;
}
