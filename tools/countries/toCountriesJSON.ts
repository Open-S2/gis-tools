// import { FileReader } from '../../src/file';
// import { OSMReader, TagFilter } from '../../src';
import { OSMFileReader } from '../../src/file';
import { TagFilter } from '../../src';

import type { Country } from '../../src/constants/countries';
import type { VectorPoint } from '../../src';

const tagFilter = new TagFilter();
tagFilter.addFilter('Relation', 'admin_level', '2');

// const fileReader = new FileReader(`${__dirname}/countries.osm.pbf`);
const reader = new OSMFileReader(`${__dirname}/countries.osm.pbf`, { tagFilter });

const res: VectorPoint<Country>[] = [];

for await (const feature of reader) {
  const { id: boundaryID, properties, metadata } = feature;
  if (metadata?.type !== 'relation') continue;
  // if (boundaryID === 270056) console.log('nodes', feature.metadata?.nodes);
  const labelNode = feature.metadata?.nodes?.find((n) => n.role === 'label');
  if (labelNode === undefined) continue;
  const node = await reader.getNode(labelNode.node);
  if (node === undefined) continue;
  const { x, y } = feature.geometry.coordinates as { x: number; y: number };
  const {
    name,
    'ISO3166-1:alpha2': alpha2,
    'ISO3166-1:alpha3': alpha3,
    'ISO3166-1:numeric': numeric,
    wikidata,
    wikipedia,
    'wikipedia:en': wikipedia_en,
    'name:ar': name_ar,
    'name:en': name_en,
    'name:es': name_es,
    'name:fr': name_fr,
    'name:de': name_de,
    'name:it': name_it,
    'name:pt': name_pt,
    'name:ru': name_ru,
    'name:zh-Hans': name_zh_Hans,
    'name:zh-Hant': name_zh_Hant,
    'name:ja': name_ja,
    'name:ko': name_ko,
    'name:vi': name_vi,
    'name:hi': name_hi,
    'name:bn': name_bn,
    'name:ur': name_ur,
    'name:fa': name_fa,
    'name:tr': name_tr,
    'name:sw': name_sw,
    'name:nl': name_nl,
    'name:pl': name_pl,
    'name:el': name_el,
    'name:he': name_he,
    'name:th': name_th,
    'name:ms': name_ms,
  } = properties;
  // if (name === undefined || alpha2 === undefined || alpha3 === undefined) continue;
  const m: Country = {
    boundaryID,
    name,
    alpha2,
    alpha3,
    numeric: numeric !== undefined ? parseInt(numeric, 10) : undefined,
    wikidata,
    wikipedia,
    wikipedia_en,
    name_ar,
    name_en,
    name_es,
    name_fr,
    name_de,
    name_it,
    name_pt,
    name_ru,
    name_zh_Hans,
    name_zh_Hant,
    name_ja,
    name_ko,
    name_vi,
    name_hi,
    name_bn,
    name_ur,
    name_fa,
    name_tr,
    name_sw,
    name_nl,
    name_pl,
    name_el,
    name_he,
    name_th,
    name_ms,
  };
  for (const key in m) if (m[key] === undefined) delete m[key];
  res.push({ x, y, m });
}

// Sort res alphabetically by 'name_en', fallback to 'name'
res.sort((a, b) => {
  return (a.m?.name_en ?? a.m?.name ?? '').localeCompare(b.m?.name_en ?? b.m?.name ?? '');
});

// console.log(res, res.length);

reader.close();
