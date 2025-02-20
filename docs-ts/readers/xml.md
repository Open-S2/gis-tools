<h1 style="text-align: center;">
  <div align="center">XML Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/xml-file.svg" alt="xml-file-ts">
  <img src="../../assets/badges/xml-gzip.svg" alt="xml-gzip-ts">
  <img src="../../assets/badges/xml-brotli.svg" alt="xml-brotli-ts">
</p>

## Description

A port of [A light XML library](https://github.com/DanielJDufour/xml-utils).

## Usage

XML tags are represented by a simple object with an outer and inner property.
The "outer" property is the text string that completely encompasses the tag, equivalent to an HTML element's "outerHTML".
The "inner" property represents the sub-parts of the tag.  It is similar to an HTML element's "textContent".

## Get attribute

```ts
import { getAttribute } from 'gis-tools-ts';

const xml = `<MDI key="INTERLEAVE">PIXEL</MDI>`;
const key = getAttribute(xml, 'key');
// key is 'INTERLEAVE'
```

## Find one tag by name

```ts
import { findTagByName } from 'gis-tools-ts';

const xml = `<Metadata domain="IMAGE_STRUCTURE"><MDI key="INTERLEAVE">PIXEL</MDI></Metadata>`
const tag = findTagByName(xml, 'MDI');
```

tag is

```json
{
  outer: "<MDI key="INTERLEAVE">PIXEL</MDI>",
  inner: "PIXEL"
}
```

## Find multiple tags with the same name

```ts
import { findTagsByName } from 'gis-tools-ts';

const xml = `
    <Metadata>
      <MDI key="SourceBandIndex">1</MDI>
      <MDI key="STATISTICS_MAXIMUM">255</MDI>
      <MDI key="STATISTICS_MEAN">96.372431147996</MDI>
      <MDI key="STATISTICS_MINIMUM">0</MDI>
      <MDI key="STATISTICS_STDDEV">50.057898474622</MDI>
    </Metadata>
`;
const tags = findTagsByName(xml, 'MDI');
// tags is an array of tags
```

## Find one tag by path

```ts
import { findTagByPath } from 'gis-tools-ts';

const xml = `
       <gmd:referenceSystemIdentifier>
         <gmd:RS_Identifier>
           <gmd:code>
             <gco:CharacterString>4326</gco:CharacterString>
           </gmd:code>
           <gmd:codeSpace>
             <gco:CharacterString>EPSG</gco:CharacterString>
           </gmd:codeSpace>
           <gmd:version>
             <gco:CharacterString>6.11</gco:CharacterString>
           </gmd:version>
         </gmd:RS_Identifier>
       </gmd:referenceSystemIdentifier>
       `;
const tag = findTagByPath(xml, ["gmd:RS_Identifier", "gmd:code", "gco:CharacterString"]);
```

## Find multiple tags by path

To get an array of tags that follow a path:

```ts
import { findTagsByPath } from 'gis-tools-ts';

const tags = findTagsByPath(xml, ['Metadata', 'MDI']);
// tags is an array of tags

// find description for 10th tag in list
const tags = findTagsByPath(iso, [
  { name: "gmd:onLine", index: 9 }, // using zero-based index
  'gmd:description',
  'gco:CharacterString'
]);
```

## Remove comments

```ts
import { removeComments } from 'gis-tools-ts';

const xml = `<list>
  <!--<A/>-->
  <B/>
</list>`;
removeComments(xml);
"<list>\n  \n<B/><list>";
```

## Remove tags by name

```ts
import { removeTagsByName } from 'gis-tools-ts';

const xml = "<ul><li>A</li><li>B</li></ul>";
removeTagsByName(xml, "li")
"<ul></ul>"
```

## Useful links

- <https://github.com/DanielJDufour/xml-utils>
- <https://en.wikipedia.org/wiki/XML>
