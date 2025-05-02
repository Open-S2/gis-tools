// BUILD GUIDE:
// GOTO: https://epsg.org/
// REGISTER
// GOTO: https://epsg.org/download-dataset.html
// DOWNLOAD: something like "EPSG-v12.007-WKT.zip" (WKT Files) (5.40 MB)"
// EXTRACT: into ./tools/
// RUN: `bun build:epsg` to generate ./src/proj/projections/epsg.ts

import { parseWKTProjection } from '../src';
import { appendFileSync, existsSync, readdirSync, unlinkSync } from 'fs';

const folder = 'EPSG-v12_007-WKT';
const outTS = `${__dirname}/../src/proj/projections/epsg.ts`;
const outRS = `${__dirname}/../rust/proj/src/generated/epsg.rs`;

// get list of files in ./EPSG-v12_007-WKT
let files = readdirSync(`${__dirname}/${folder}`);

// delete file if it exsts: `${__dirname}/../src/proj/projections/epsg.ts`
if (existsSync(outTS)) unlinkSync(outTS);
// delete file if it exsts: `${__dirname}/../rust/proj/epsg.rs`
if (existsSync(outRS)) unlinkSync(outRS);

// start with automation comment:
appendFileSync(
  outTS,
  `/* AUTO-GENERATED FILE, DO NOT EDIT */\n/* eslint-disable */\n// @ts-nocheck\n`,
);
appendFileSync(
  outRS,
  `// AUTO-GENERATED FILE, DO NOT EDIT
// generated with \`bun build:epsg\`
#![rustfmt::skip]
#![allow(dead_code, unused_imports, clippy::all)]
`,
);

// SORT the files by the code
files = files.sort((a, b) => {
  const aCode = a.split('.')[0].split('-').pop();
  const bCode = b.split('.')[0].split('-').pop();
  return parseInt(aCode ?? '') - parseInt(bCode ?? '');
});

for (const file of files) {
  const code = file.split('.')[0].split('-').pop();
  let text = await Bun.file(`${__dirname}/${folder}/${file}`).text();
  let rustText = text.replaceAll("'", '\\"');
  rustText = rustText.replaceAll('"', '\\"');
  text = text.replaceAll("'", "\\'");
  const wkt = parseWKTProjection(text);
  const { name, type, DATUM } = wkt;
  // @ts-expect-error - dunno why its failing
  const datumName = DATUM?.ELLIPSOID?.name;
  appendFileSync(
    outTS,
    `\n/**
 * # ${name}
 * - **Type**: ${type}
 * - **Datum**: ${datumName ?? 'N/A'}
 */
export const EPSG_${code} =
  '${text}';\n`,
  );

  appendFileSync(
    outRS,
    `
/// # ${name}
/// - **Type**: ${type}
/// - **Datum**: ${datumName ?? 'N/A'}
pub const EPSG_${code}: &str = "${rustText}";\n`,
  );
}
