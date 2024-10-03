import computeShader from './sgp4.wgsl';
import { earthRadius, j2, j3oj2, pi, twoPi, vkmpersec, x2o3, xke } from '../util';

import type { Satellite } from '../sat';

/**
 * Note: After creating the class you must call `await bfGPU.init()`
 */
export class SGP4GPU {
  #device: GPUDevice;
  #sgp4Pipeline!: GPUComputePipeline;
  #layout!: GPUBindGroupLayout;
  #bindGroup0!: GPUBindGroup;
  #bindGroup1!: GPUBindGroup;
  #size = 0;
  /**
   * @param device
   */
  constructor(device: GPUDevice) {
    this.#device = device;
  }

  /**
   *
   */
  async init(): Promise<void> {
    const layout = (this.#layout = this.#device.createBindGroupLayout({
      entries: [
        {
          // constants
          binding: 0,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: 'uniform',
          },
        },
        {
          // tsince
          binding: 1,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: 'uniform',
          },
        },
        {
          // size
          binding: 2,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: 'uniform',
          },
        },
        {
          // array<Satellite>
          binding: 3,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: 'read-only-storage',
          },
        },
        {
          // array<SatOutput>
          binding: 4,
          visibility: GPUShaderStage.COMPUTE,
          buffer: {
            type: 'storage',
          },
        },
      ],
    }));

    // ** BUILD PIPELINES **

    // ? SGP4
    // Compute shader code
    const computeShaderModule = this.#device.createShaderModule({ code: computeShader });
    // Pipeline setup
    this.#sgp4Pipeline = await this.#device.createComputePipelineAsync({
      label: 'SGP4',
      layout: this.#device.createPipelineLayout({
        bindGroupLayouts: [layout],
      }),
      compute: {
        module: computeShaderModule,
        entryPoint: 'sgp4',
      },
    });
  }

  // returns the distance GPU Buffer incase you want to use it
  /**
   * @param sats
   */
  prepareData(sats: Satellite[]): GPUBuffer {
    const size = (this.#size = sats.length);
    // ? PREP CONSTANTS
    const constants = new Float32Array([
      pi, // pi: f32
      twoPi, // twoPi: f32
      earthRadius, // earthRadius: f32
      xke, // xke: f32
      vkmpersec, // vkmpersec: f32
      j2, // j2: f32
      j3oj2, // j3oj2: f32
      x2o3, // x2o3: f32
    ]);
    const gpuBufferConstants = this.#device.createBuffer({
      mappedAtCreation: true,
      size: constants.byteLength,
      usage: GPUBufferUsage.UNIFORM,
    });
    const arrayBufferConstants = gpuBufferConstants.getMappedRange();
    new Float32Array(arrayBufferConstants).set(constants);
    gpuBufferConstants.unmap();

    // ? PREP SIZE
    const sizeArr = new Float32Array([size]);
    const gpuBufferSize = this.#device.createBuffer({
      mappedAtCreation: true,
      size: sizeArr.byteLength,
      usage: GPUBufferUsage.UNIFORM,
    });
    const arrayBufferSize = gpuBufferSize.getMappedRange();
    new Float32Array(arrayBufferSize).set(sizeArr);
    gpuBufferSize.unmap();

    // ? Satellites
    const satsArray = new Float32Array(sats.flatMap((s) => s.gpu()));
    const gpuBufferSats = this.#device.createBuffer({
      mappedAtCreation: true,
      size: satsArray.byteLength,
      usage: GPUBufferUsage.STORAGE,
    });
    const arrayBufferSats = gpuBufferSats.getMappedRange();
    new Float32Array(arrayBufferSats).set(satsArray);
    gpuBufferSats.unmap();

    // ? OUT (build result distance array)
    const out = Array(size * 7).fill(0);
    const outArray = new Uint32Array(out);
    const gpuBufferOut = this.#device.createBuffer({
      mappedAtCreation: true,
      size: size * 7 * 4,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
    });
    const arrayBufferDist = gpuBufferOut.getMappedRange();
    new Uint32Array(arrayBufferDist).set(outArray);
    gpuBufferOut.unmap();

    // ? prep bind group
    const entries = [
      {
        binding: 0,
        resource: {
          buffer: gpuBufferConstants,
        },
      },
      {
        binding: 1,
        resource: {
          buffer: gpuBufferSize,
        },
      },
      {
        binding: 2,
        resource: {
          buffer: gpuBufferSats,
        },
      },
      {
        binding: 3,
        resource: {
          buffer: gpuBufferOut,
        },
      },
    ];

    // Bind group layout and bind group
    this.#bindGroup0 = this.#device.createBindGroup({ layout: this.#layout, entries });

    return gpuBufferOut;
  }

  // returns the distance GPU Buffer incase you want to use it
  /**
   * @param tsince
   */
  setTime(tsince: number): void {
    // ? Prep buffer
    const tsinceArr = new Float32Array([tsince]);
    const gpuBufferTsince = this.#device.createBuffer({
      mappedAtCreation: true,
      size: tsinceArr.byteLength,
      usage: GPUBufferUsage.UNIFORM,
    });
    const arrayBufferSize = gpuBufferTsince.getMappedRange();
    new Float32Array(arrayBufferSize).set(tsinceArr);
    gpuBufferTsince.unmap();

    // ? prep bind group
    const entries = [
      {
        binding: 0,
        resource: {
          buffer: gpuBufferTsince,
        },
      },
    ];

    // Bind group layout and bind group
    this.#bindGroup1 = this.#device.createBindGroup({ layout: this.#layout, entries });
  }

  // const commandEncoder = device.createCommandEncoder()
  // const passEncoder = commandEncoder.beginComputePass()
  // do stuff...
  // this.run(passEncoder)
  // do stuff...
  // passEncoder.end()
  /**
   * @param passEncoder
   * @param blockSize
   */
  run(passEncoder: GPUComputePassEncoder, blockSize = 256): void {
    const numBlocks = Math.ceil((this.#size + blockSize - 1) / blockSize);
    // Commands submission
    passEncoder.setBindGroup(0, this.#bindGroup0);
    passEncoder.setBindGroup(1, this.#bindGroup1);
    passEncoder.setPipeline(this.#sgp4Pipeline);
    passEncoder.dispatchWorkgroups(numBlocks);
  }

  // run this command after sending all instructions to passEncoder but BEFORE submitting to the queue
  // const gpuBufferOut = this.prepareData({ ... })
  // ...
  // this.run(passEncoder)
  // passEncoder.end()
  // this.createReadDistBuffer(gpuBufferOut)
  // device.queue.submit([commandEncoder.finish()])
  // await gpuReadBufferDist.mapAsync(GPUMapMode.READ)
  // const resultDist = gpuReadBufferDist.getMappedRange()
  // const resultAB = new Float32Array(resultDist)
  /**
   * @param commandEncoder
   * @param gpuBufferOut
   */
  createReadDistBuffer(commandEncoder: GPUCommandEncoder, gpuBufferOut: GPUBuffer): GPUBuffer {
    const size = this.#size * 7 * 4;
    const gpuReadBufferDist = this.#device.createBuffer({
      size,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
    });

    // Encode commands for copying buffer to buffer.
    commandEncoder.copyBufferToBuffer(
      gpuBufferOut /* source buffer */,
      0 /* source offset */,
      gpuReadBufferDist /* destination buffer */,
      0 /* destination offset */,
      size /* size */,
    );

    return gpuReadBufferDist;
  }
}
