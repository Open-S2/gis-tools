import { availableParallelism } from 'os';

// Augmenting global types
declare global {
  /**
   * Declare the navigator interface for those outside the browser
   */
  interface navigator {
    userAgent: string;
    platform: string;
    gpu?: GPU;
    hardwareConcurrency: number;
  }
}

globalThis.navigator ??= {
  userAgent: 'local',
  platform: 'MacIntel',
  // @ts-expect-error - GPU is not defined for our use case
  gpu: undefined,
  hardwareConcurrency: availableParallelism(),
};
