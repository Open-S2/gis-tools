// first convert the wasm file to base64
// const dataS2Cell = await Bun.file('./zig-out/bin/optimized-s2cell.wasm').arrayBuffer();
const dataS2Cell = await Bun.file(
  './target/wasm32-unknown-unknown/release/optimized.wasm',
).arrayBuffer();
const u8S2Cell = Array.from(new Uint8Array(dataS2Cell));
const base64 = btoa(String.fromCharCode.apply(null, u8S2Cell));
const code = `export default '${base64}';\n`;
await Bun.write('./src/wasm/uint64.wasm.ts', code);

try {
  console.info('Starting the build process...');
  await Bun.build({
    entrypoints: ['src/index.ts'],
    outdir: 'dist',
    format: 'esm',
    minify: false,
    sourcemap: 'external',
    // target: 'esnext', // Adjust target based on your project needs
  });
  console.info('Build completed successfully!');
} catch (error) {
  console.error('Build failed:', error);
}

export {};
