// /**
//  * Build vector tiles give a guide on what sources to parse data from and how to store it
//  * @param buildGuide - the user defined guide on building the vector tiles
//  */
// export async function toVectorTiles(buildGuide: BuildGuide): Promise<void> {
//   const { tileWriter, threads } = buildGuide;
//   const totalThreads = Math.max(threads ?? 1, navigator.hardwareConcurrency ?? 1);

//   // STEP 1: Convert all features to tile slices of said features.
//   await toVectorTilesSliceFeatures(buildGuide, totalThreads);

//   // STEP 2: Sort the stores

//   // STEP 3: Create workers that collect all existing multimap feature stores and build tiles

//   // STEP 4: build metadata based on the guide
//   const metaBuilder = new MetadataBuilder();
//   updateBuilder(metaBuilder, buildGuide);
//   const metadata = metaBuilder.commit();

//   // STEP 5: Commit the metadata
//   await tileWriter.commit(metadata);
// }

// /**
//  * STEP 1: Convert all features to tile slices of said features.
//  * @param buildGuide - the user defined guide on building the vector tiles
//  * @param totalThreads - the number of threads to use
//  */
// async function toVectorTilesSliceFeatures(
//   buildGuide: BuildGuide,
//   totalThreads: number,
// ): Promise<void> {
//   const { vectorSources, rasterSources, layerGuides } = buildGuide;
//   const featuresIterator = getFeature(vectorSources, rasterSources);

//   await new Promise<void>((resolve) => {
//     let threadsComplete = 0;
//     for (let i = 0; i < totalThreads; i++) {
//       const worker = new Worker(new URL('./vectorWorker', import.meta.url).href, {
//         type: 'module',
//       });
//       /** A ready state has been submitted for work */
//       worker.onmessage = async (): Promise<void> => {
//         // iterate features and have workers split/store them
//         const nextFeature = await featuresIterator.next();
//         if (nextFeature.done === true) {
//           threadsComplete++;
//           if (threadsComplete === totalThreads) resolve();
//           worker.terminate();
//         } else {
//           const { sourceName, feature } = nextFeature.value;
//           const featureMessage: FeatureMessage = { type: 'feature', sourceName, feature };
//           worker.postMessage(featureMessage);
//         }
//       };
//       // Prepare workers with init messages
//       const stringifiedLayerGuides = prepareLayerGuides(layerGuides);
//       const initMessage: InitMessage = { type: 'init', id: i, layerGuides: stringifiedLayerGuides };
//       worker.postMessage(initMessage);
//     }
//   });
// }
