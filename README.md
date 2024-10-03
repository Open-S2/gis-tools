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
