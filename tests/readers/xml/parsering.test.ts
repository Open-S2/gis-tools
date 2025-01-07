import {
  xmlCountSubstring,
  xmlFindTagByName,
  xmlFindTagByPath,
  xmlFindTagsByName,
  xmlFindTagsByPath,
  xmlGetAttribute,
  xmlIndexOfMatch,
  xmlIndexOfMatchEnd,
  xmlRemoveComments,
  xmlRemoveTagsByName,
} from '../../../src/readers/xml/parsing';

import { beforeAll, expect, test } from 'bun:test';

import type { XMLTag } from '../../../src/readers/xml/parsing';

let iso: string;
let mrf: string;
let tiffAux: string;
let tmx: string;
let svg: string;

beforeAll(async () => {
  iso = await Bun.file(`${__dirname}/fixtures/iso.xml`).text();
  mrf = await Bun.file(`${__dirname}/fixtures/m_3008501_ne_16_1_20171018.mrf`).text();
  tiffAux = await Bun.file(`${__dirname}/fixtures/rgb_raster.tif.aux.xml`).text();
  // tmx example from https://en.wikipedia.org/wiki/Translation_Memory_eXchange
  tmx = await Bun.file(`${__dirname}/fixtures/tmx.xml`).text();
  // svg example from https://en.wikipedia.org/wiki/SVG
  svg = await Bun.file(`${__dirname}/fixtures/example.svg`).text();
});

const nested = '<Thing><Thing attr=1></Thing><Thing attr=2></Thing></Thing>';

const commented = `<Thing>
<!--
  <Thing attr=1></Thing>
-->
<Thing attr=2></Thing>
</Thing>`;

const multiline = `<div
id="container"
>
  <div
    id="inside"
    data-foo="bar"
  </div>
</div>
`;

test('tmx', () => {
  expect(xmlGetAttribute(tmx, 'version')).toEqual('1.4');
  const header: XMLTag = xmlFindTagByName(tmx, 'header') ?? {
    inner: '',
    outer: '',
    start: 0,
    end: 0,
  };
  expect(xmlGetAttribute(header, 'srclang')).toEqual('en');
  expect(xmlGetAttribute(header, 'o-tmf')).toEqual('ABCTransMem');
  const tu = xmlFindTagByName(tmx, 'tu', { debug: false });
  expect((tu?.inner ?? '').trim()).toEqual(
    '<tuv xml:lang="en">\n        <seg>Hello world!</seg>\n      </tuv>\n      <tuv xml:lang="fr">\n        <seg>Bonjour tout le monde!</seg>\n      </tuv>',
  );
  const tuvs = xmlFindTagsByName(tmx, 'tuv');
  expect(tuvs.length).toEqual(2);
});

test('svg', () => {
  const tag: XMLTag = xmlFindTagByName(svg, 'svg') ?? { inner: '', outer: '', start: 0, end: 0 };
  expect(xmlGetAttribute(tag, 'height')).toEqual('391');
  expect(xmlGetAttribute(tag, 'width')).toEqual('391');
  expect(xmlGetAttribute(tag, 'viewBox')).toEqual('-70.5 -70.5 391 391');
  expect(xmlGetAttribute(tag, 'xmlns:xlink')).toEqual('http://www.w3.org/1999/xlink');
  expect(xmlGetAttribute(xmlFindTagByPath(svg, ['g']) ?? '', 'opacity')).toEqual('0.8');
  expect(xmlGetAttribute(xmlFindTagByName(svg, 'rect') ?? '', 'fill')).toEqual('#fff');
  const rect = xmlFindTagByPath(svg, ['g', 'rect'])?.outer;
  expect(xmlGetAttribute(rect ?? '', 'x')).toEqual('25');
  expect(xmlGetAttribute(rect ?? '', 'stroke-width')).toEqual('4');
});

test('support multi-line tags', () => {
  const container = xmlFindTagByName(multiline, 'div');
  expect(container?.outer).toEqual(
    `<div\nid="container"\n>\n  <div\n    id="inside"\n    data-foo="bar"\n  </div>\n</div>`,
  );
  expect(container?.inner).toEqual(`\n  <div\n    id="inside"\n    data-foo="bar"\n  </div>\n`);
  expect(xmlGetAttribute(container?.outer ?? '', 'id')).toEqual('container');
  expect(xmlGetAttribute(container?.outer ?? '', 'data-foo')).toBeUndefined();
  expect(xmlGetAttribute((container?.inner ?? '').trim(), 'data-foo', { debug: false })).toEqual(
    'bar',
  );
});

test('should get gmd:code and avoid gmd:codeSpace', () => {
  const index = xmlIndexOfMatch(iso, `<gmd:code[ >]`, 0);
  expect(iso.slice(index).startsWith('<gmd:code')).toEqual(true);

  const tag = xmlFindTagByName(iso, 'gmd:code', { startIndex: index + 1 });
  expect(tag).toBeUndefined();
});

test('xmlIndexOfMatchEnd', () => {
  const xml = `<items><item><item></items>`;
  const index = xmlIndexOfMatchEnd(xml, '[ /]items>', 0);
  expect(index).toEqual(xml.length - 1);
});

test('removing comments', () => {
  expect(xmlRemoveComments(commented)).toEqual('<Thing>\n\n<Thing attr=2></Thing>\n</Thing>');
  expect(xmlRemoveComments('<A><!--<B/>--><!--<C/>--></A>')).toEqual('<A></A>');
});

test('count substring', () => {
  expect(xmlCountSubstring(nested, '<namespace:name')).toEqual(0);
  expect(xmlCountSubstring(nested, '<Test')).toEqual(0);
  expect(xmlCountSubstring(nested, '<Thing')).toEqual(3);
  expect(xmlCountSubstring(nested, '/Thing>')).toEqual(3);
});

test('should find all the urls in iso.xml', () => {
  const urls = xmlFindTagsByName(iso, 'gmd:URL');
  expect(urls[0].inner).toEqual('http://geomap.arpa.veneto.it/layers/geonode%3Aatlanteil');
  expect(urls.length).toEqual(29);
});

test('should get only tags with full string match on tag name', () => {
  const urls = xmlFindTagsByName(iso, 'gmd:code');
  expect(urls.length).toEqual(1);
});

test('should get info from iso.xml file', () => {
  const tag = xmlFindTagByPath(iso, ['gmd:RS_Identifier', 'gmd:code', 'gco:CharacterString']);
  const projection = parseInt(tag!.inner!);
  expect(projection).toEqual(4326);

  const longitude = Number(xmlFindTagByPath(iso, ['gmd:westBoundLongitude', 'gco:Decimal'])!.inner);
  expect(longitude).toEqual(10.2822923743907);
});

test('should get raster size from a .mrf file', () => {
  const rasterSize = xmlFindTagByPath(mrf, ['MRF_META', 'Raster', 'Size'], {
    debug: false,
  })!;
  expect(rasterSize?.outer).toEqual('<Size x="6638" y="7587" c="4" />');
  expect(rasterSize?.inner).toEqual(null);
});

test('should get all character strings', () => {
  const tags = xmlFindTagsByPath(iso, ['gmd:RS_Identifier', 'gmd:code']);
  expect(tags.length).toEqual(1);
  expect(tags[0]?.inner === '').toEqual(false);
});

test('should get all metadata for bands from .tif.aux.xml', () => {
  const debug = false;
  const mdis = xmlFindTagsByPath(tiffAux, ['Metadata', 'MDI'], { debug });
  expect(mdis.length).toEqual(15);
});

test('should get attributes from metadata', () => {
  const mdi = xmlFindTagByPath(tiffAux, ['Metadata', 'MDI'], { debug: false });
  const key = xmlGetAttribute(mdi!, 'key', { debug: false });
  expect(key).toEqual('SourceBandIndex');
});

test('should get raster width from a .mrf file', () => {
  const rasterSize = '<Size x="6638" y="7587" c="4" />';
  expect(xmlGetAttribute(rasterSize, 'x')).toEqual('6638');
  expect(xmlGetAttribute(rasterSize, 'y')).toEqual('7587');
  expect(xmlGetAttribute(rasterSize, 'c')).toEqual('4');
});

test('should get first tag', () => {
  const xml = `<fields> <field datatype="text" name="L101"/> <field datatype="text" name="L101_1"/> <field datatype="text" name="P102"/> <field datatype="text" name="P102_1"> <source></source> <param></param> </field> <field datatype="text" name="P103"></field> </fields>`;
  const tag = xmlFindTagByName(xml, 'field', { debug: false })!;
  expect(tag.outer).toEqual(`<field datatype="text" name="L101"/>`);

  const tag2 = xmlFindTagByName(xml, 'field', { debug: false, nested: false })!;
  expect(tag2.outer).toEqual(`<field datatype="text" name="L101"/>`);
});

test('should get all tags (self-closing and not)', () => {
  const xml = `<fields> <field datatype="text" name="L101"/> <field datatype="text" name="L101_1"/> <field datatype="text" name="P102"/> <field datatype="text" name="P102_1"> <source></source> <param></param> </field> <field datatype="text" name="P103"></field> </fields>`;
  const tags = xmlFindTagsByName(xml, 'field', { debug: false });
  expect(tags.length).toEqual(5);
});

test('should get self-closing with immediate close and without interior space', () => {
  const xml = `<House><Kitchen/></House>`;
  const tag = xmlFindTagByName(xml, 'Kitchen')!;
  expect(tag.outer).toEqual('<Kitchen/>');
  expect(tag.inner).toEqual(null);
});

test('should handle nested tags', () => {
  const xml = `<Thing><Thing sub1>A</Thing><Thing sub2>B</Thing></Thing>`;

  expect(xmlFindTagByName(xml, 'Thing')!.outer).toEqual(xml);
  expect(xmlFindTagByName(xml, 'Thing')!.outer).toEqual(xml);

  expect(xmlFindTagsByName(xml, 'Thing').length).toEqual(3);
  expect(xmlFindTagsByName(xml, 'Thing')[0].outer).toEqual(xml);
  expect(xmlFindTagsByName(xml, 'Thing', { nested: true }).length).toEqual(3);
  expect(xmlFindTagsByName(xml, 'Thing', { nested: true })[0].outer).toEqual(xml);
  expect(xmlFindTagsByName(xml, 'Thing', { nested: false }).length).toEqual(1);
  expect(xmlFindTagsByName(xml, 'Thing', { nested: false })[0].outer).toEqual(xml);

  expect(xmlFindTagsByPath(xml, ['Thing']).length).toEqual(1);
  expect(xmlFindTagsByPath(xml, ['Thing'])[0].outer).toEqual(xml);
  expect(xmlFindTagsByPath(xml, ['Thing', 'Thing'])).toEqual([
    { inner: 'A', outer: '<Thing sub1>A</Thing>', start: 7, end: 28 },
    { inner: 'B', outer: '<Thing sub2>B</Thing>', start: 28, end: 49 },
  ]);
  expect(xmlFindTagByPath(xml, ['Thing'])!.outer).toEqual(xml);
});

test('xmlRemoveTagsByName', () => {
  expect(xmlRemoveTagsByName('<ul><li>A</li><li>B</li></ul>', 'li')).toEqual('<ul></ul>');
});

test('check immutability of xmlFindTagsByPath', () => {
  const path = ['gmd:RS_Identifier', 'gmd:code'] as const;
  const tags = xmlFindTagsByPath(iso, path);
  expect(tags.length).toEqual(1);
  expect(tags[0].inner === '').toEqual(false);
  expect(path.length).toEqual(2);
});

test('simple check xmlFindTagsByPath with index', () => {
  const xml = '<tag>A</tag><tag>B</tag><tag>C</tag><tag>D</tag>';
  const tags = xmlFindTagsByPath(xml, [{ name: 'tag', index: 2 }]);
  expect(tags).toEqual([{ outer: '<tag>C</tag>', inner: 'C', start: 24, end: 36 }]);
});

test('xmlFindTagsByPath with larger source', () => {
  const tags = xmlFindTagsByPath(iso, [
    'gmd:MD_DigitalTransferOptions',
    { name: 'gmd:onLine', index: 10 },
    'gmd:CI_OnlineResource',
    'gmd:description',
    'gco:CharacterString',
  ]);
  expect(tags).toEqual([
    {
      outer:
        '<gco:CharacterString>Veneto Atlas of artificial night sky brightness - GRID (ArcGrid Format)</gco:CharacterString>',
      inner: 'Veneto Atlas of artificial night sky brightness - GRID (ArcGrid Format)',
      start: 20934,
      end: 21048,
    },
  ]);
});

test('check xmlFindTagsByPath with index (multi-level)', () => {
  const xml = `
  <outer>
    <pair>
      <tag>A</tag>
      <tag>B</tag>
    </pair>
    <pair>
      <tag>C</tag>
      <tag>D</tag>
    </pair>
  </outer>`;

  expect(xmlFindTagsByPath(xml, [{ name: 'tag', index: 2 }])).toEqual([
    { inner: 'C', outer: '<tag>C</tag>', start: 89, end: 101 },
  ]);

  expect(
    xmlFindTagsByPath(xml, ['outer', { name: 'pair', index: 1 }, { name: 'tag', index: 0 }]),
  ).toEqual([{ inner: 'C', outer: '<tag>C</tag>', start: 89, end: 101 }]);

  expect(
    xmlFindTagsByPath(xml, ['outer', { name: 'pair', index: 1 }, { name: 'tag', index: 1 }]),
  ).toEqual([{ inner: 'D', outer: '<tag>D</tag>', start: 108, end: 120 }]);
});
