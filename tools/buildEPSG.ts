// BUILD GUIDE:
// GOTO: https://epsg.org/
// REGISTER
// GOTO: https://epsg.org/download-dataset.html
// DOWNLOAD: something like "EPSG v11.031 (WKT File) (5.40 MB)"
// EXTRACT: into ./tools/
// RUN: bun build:epsg

import { parseWKTProjection } from '../src';
import { appendFileSync, existsSync, readdirSync, unlinkSync } from 'fs';

const folder = 'EPSG-v11_031-WKT';
const out = `${__dirname}/../src/proj/projections/epsg.ts`;

// get list of files in ./EPSG-v11_031-WKT
const files = readdirSync(`${__dirname}/${folder}`);

// delete file if it exsts: `${__dirname}/../src/proj/projections/epsg.ts`
if (existsSync(out)) unlinkSync(out);

// start with automation comment:
appendFileSync(out, `/* AUTO-GENERATED FILE, DO NOT EDIT */\n`);

for (const file of files) {
  const code = file.split('.')[0].split('-').pop();
  let text = await Bun.file(`${__dirname}/${folder}/${file}`).text();
  text = text.replaceAll("'", "\\'");
  const wkt = parseWKTProjection(text);
  const { name, type, DATUM } = wkt;
  // @ts-expect-error - dunno why its failing
  const datumName = DATUM?.ELLIPSOID?.name;
  appendFileSync(
    out,
    `\n/**
 * # ${name}
 * - **Type**: ${type}
 * - **Datum**: ${datumName ?? 'N/A'}
 */
export const EPSG_${code} =
  '${text}';\n`,
  );
}
