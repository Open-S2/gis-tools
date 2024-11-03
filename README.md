<h1 style="text-align: center;">
  <div align="center">s2-tools</div>
</h1>

<p align="center">
  <a href="https://img.shields.io/github/actions/workflow/status/Open-S2/s2-tools/test.yml?logo=github">
    <img src="https://img.shields.io/github/actions/workflow/status/Open-S2/s2-tools/test.yml?logo=github" alt="GitHub Actions Workflow Status">
  </a>
  <a href="https://npmjs.org/package/s2-tools">
    <img src="https://img.shields.io/npm/v/s2-tools.svg?logo=npm&logoColor=white" alt="npm">
  </a>
  <a href="https://crates.io/crates/s2-tools">
    <img src="https://img.shields.io/crates/v/s2-tools.svg?logo=rust&logoColor=white" alt="crate">
  </a>
  <a href="https://www.npmjs.com/package/s2-tools">
    <img src="https://img.shields.io/npm/dm/s2-tools.svg" alt="downloads">
  </a>
  <a href="https://bundlejs.com/?q=s2-tools">
    <img src="https://img.shields.io/bundlejs/size/s2-tools" alt="bundle">
  </a>
  <a href="https://open-s2.github.io/s2-tools/">
    <img src="https://img.shields.io/badge/docs-typescript-yellow.svg" alt="docs-ts">
  </a>
  <a href="https://docs.rs/s2-tools">
    <img src="https://img.shields.io/badge/docs-rust-yellow.svg" alt="docs-rust">
  </a>
  <a href="https://coveralls.io/github/Open-S2/s2-tools?branch=master">
    <img src="https://coveralls.io/repos/github/Open-S2/s2-tools/badge.svg?branch=master" alt="code-coverage">
  </a>
  <a href="https://discord.opens2.com">
    <img src="https://img.shields.io/discord/953563031701426206?logo=discord&logoColor=white" alt="Discord">
  </a>
</p>

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
[esBadge]: https://deno.bundlejs.com/badge?q=s2-tools&treeshake=[{+externalSort+}]
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
# faster
cargo tarpaulin --color always --skip-clean
# bacon
bacon coverage # or type `l` inside the tool
```
