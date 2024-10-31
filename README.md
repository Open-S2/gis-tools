# s2-tools ![GitHub Actions Workflow Status][test-workflow] [![npm][npm-image]][npm-url] [![crate][crate-image]][crate-url] [![downloads][downloads-image]][downloads-url] [![bundle][bundle-image]][bundle-url] [![docs-ts][docs-ts-image]][docs-ts-url] [![docs-rust][docs-rust-image]][docs-rust-url] ![doc-coverage][doc-coverage-image] ![code-coverage][code-coverage-image] [![Discord][discord-image]][discord-url]

[test-workflow]: https://img.shields.io/github/actions/workflow/status/Open-S2/s2-tools/test.yml?logo=github
[npm-image]: https://img.shields.io/npm/v/s2-tools.svg?logo=npm&logoColor=white
[npm-url]: https://npmjs.org/package/s2-tools
[crate-image]: https://img.shields.io/crates/v/s2-tools.svg?logo=rust&logoColor=white
[crate-url]: https://crates.io/crates/s2-tools
[bundle-image]: https://img.shields.io/bundlejs/size/s2-tools
[bundle-url]: https://bundlejs.com/?q=s2-tools&treeshake=%5B%7B+s2-tools+%7D%5D
[downloads-image]: https://img.shields.io/npm/dm/s2-tools.svg
[downloads-url]: https://www.npmjs.com/package/s2-tools
[docs-ts-image]: https://img.shields.io/badge/docs-typescript-yellow.svg
[docs-ts-url]: https://open-s2.github.io/s2-tools/
[docs-rust-image]: https://img.shields.io/badge/docs-rust-yellow.svg
[docs-rust-url]: https://docs.rs/s2-tools
[doc-coverage-image]: https://raw.githubusercontent.com/Open-S2/s2-tools/master/assets/doc-coverage.svg
[code-coverage-image]: https://raw.githubusercontent.com/Open-S2/s2-tools/master/assets/code-coverage.svg
[discord-image]: https://img.shields.io/discord/953563031701426206?logo=discord&logoColor=white
[discord-url]: https://discord.opens2.com

## About

A collection of geospatial tools primarily designed for WGS84, Web Mercator, and S2.

## Install

```bash
# NPM
npm install s2-tools
# PNPM
pnpm add s2-tools
# Yarn
yarn add s2-tools
# Bun
bun add s2-tools
```

## Components

> 💡 **NOTE:** The sizes are estimates and can change based on how you use them. Click the module link for documentation and more precise guides on file cost.

### Converters

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [toJSON]                 | ![To JSON Badge][toJSONBadge] | Convert any Reader to JSON data.                                 |
| [toTiles]                | ![FT Badge][toTilesBadge]     | Convert any Reader to vector and/or raster tiles.                |

[toJSON]: https://github.com/Open-S2/s2-tools/tree/master/docs-ts/converters/toJSON.md
[toJSONBadge]: https://deno.bundlejs.com/badge?q=s2-tools&treeshake=[{+toJSON,toJSONLD,BufferReader+}]
[toTiles]: https://github.com/Open-S2/s2-tools/tree/master/docs-ts/converters/toTiles.md
[toTilesBadge]: https://deno.bundlejs.com/badge?q=s2-tools&treeshake=[{+toTiles,BufferReader+}]

### Data Stores

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [externalSort]           | ![ES Badge][esBadge]          | Sort large files with uint64 keys                                |
| [kv]                     | ![KV Badge][kvBadge]          | Collection of tools using the filesystem to read and write data. |
| [multiMap]               | ![MM Badge][mmBadge]          | Collection of tools using the filesystem to read and write data. |
| [vector]                 | ![Vec Badge][vecBadge]        | Collection of tools using the filesystem to read and write data. |

[externalSort]: https://github.com/Open-S2/s2-tools/tree/master/docs-ts/dataStores/externalSort.md
[esBadge]: https://deno.bundlejs.com/badge?q=s2-tools/file&treeshake=[{externalSort}]
[kv]: https://github.com/Open-S2/s2-tools/tree/master/docs-ts/dataStores/kv.md
[kvBadge]: https://deno.bundlejs.com/badge?q=s2-tools&treeshake=[{KV}]
[multiMap]: https://github.com/Open-S2/s2-tools/tree/master/docs-ts/dataStores/multimap.md
[mmBadge]: https://deno.bundlejs.com/badge?q=s2-tools&treeshake=[{MultiMap}]
[vector]: https://github.com/Open-S2/s2-tools/tree/master/docs-ts/dataStores/vector.md
[vecBadge]: https://deno.bundlejs.com/badge?q=s2-tools&treeshake=[{Vector}]

---

## Development

### Requirements

You need the tool `tarpaulin` to generate the coverage report. Install it using the following command:

```bash
cargo install cargo-tarpaulin
```

The `bacon coverage` tool is used to generate the coverage report. To utilize the [pycobertura](https://pypi.org/project/pycobertura/) package for a prettier coverage report, install it using the following command:

```bash
pip install pycobertura
```

### Running Tests

To run the tests, use the following command:

```bash
# TYPESCRIPT
## basic test
bun run test
## live testing
bun run test:dev

# RUST
## basic test
cargo test
# live testing
bacon test
```

### Generating Coverage Report

To generate the coverage report, use the following command:

```bash
cargo tarpaulin
# bacon
bacon coverage # or type `l` inside the tool
```
