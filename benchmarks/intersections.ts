import { union } from '@turf/turf';
import { polygonsIntersections, toVector } from '../src/index.js';

import type { FeatureCollection, VectorMultiPolygon } from 's2json-spec';
import type {
  FeatureCollection as TurfFeatureCollection,
  MultiPolygon as TurfMultiPolygon,
  Polygon as TurfPolygon,
} from 'geojson';

console.info('intersections: ');

const featureCollection: FeatureCollection = await Bun.file(
  `${__dirname}/../tests/geometry/tools/fixtures/chunks-water.json`,
).json();
// convert to vector format
const vectorPolygons: VectorMultiPolygon = [];
for (const feature of featureCollection.features) {
  const vectorFeature = toVector(feature);
  const { geometry } = vectorFeature;
  if (geometry.type === 'MultiPolygon') {
    vectorPolygons.push(...geometry.coordinates);
  } else if (geometry.type === 'Polygon') {
    vectorPolygons.push(geometry.coordinates);
  }
}

// GIS-TOOLS

const startIntersection = Bun.nanoseconds();

const _intersections = polygonsIntersections(vectorPolygons);

const endIntersection = Bun.nanoseconds();
const secondsIntersections = (endIntersection - startIntersection) / 1_000_000_000;
console.info('GIS-TOOLS Intersections time: ', secondsIntersections);

// TURF

const startTurf = Bun.nanoseconds();

const _intersectionsTurf = union(
  featureCollection as TurfFeatureCollection<TurfPolygon | TurfMultiPolygon>,
);

const endTurf = Bun.nanoseconds();
const secondsTurf = (endTurf - startTurf) / 1_000_000_000;
console.info('TURF Intersections time: ', secondsTurf);

// console.info('intersectionsTurf', intersectionsTurf);

//  bun run ./benchmarks/intersections.ts
// dekink:
// GIS-TOOLS Dekink time:  4.616010417
// TURF Dekink time:  15.452188166

// NOTE: All packages in turf:
//  npm ls --all --parseable | sort | uniq
//
//
// /node_modules/@turf/along
// /node_modules/@turf/angle
// /node_modules/@turf/area
// /node_modules/@turf/bbox
// /node_modules/@turf/bbox-clip
// /node_modules/@turf/bbox-polygon
// /node_modules/@turf/bearing
// /node_modules/@turf/bezier-spline
// /node_modules/@turf/boolean-clockwise
// /node_modules/@turf/boolean-concave
// /node_modules/@turf/boolean-contains
// /node_modules/@turf/boolean-crosses
// /node_modules/@turf/boolean-disjoint
// /node_modules/@turf/boolean-equal
// /node_modules/@turf/boolean-intersects
// /node_modules/@turf/boolean-overlap
// /node_modules/@turf/boolean-parallel
// /node_modules/@turf/boolean-point-in-polygon
// /node_modules/@turf/boolean-point-on-line
// /node_modules/@turf/boolean-touches
// /node_modules/@turf/boolean-valid
// /node_modules/@turf/boolean-within
// /node_modules/@turf/buffer
// /node_modules/@turf/center
// /node_modules/@turf/center-mean
// /node_modules/@turf/center-median
// /node_modules/@turf/center-of-mass
// /node_modules/@turf/centroid
// /node_modules/@turf/circle
// /node_modules/@turf/clean-coords
// /node_modules/@turf/clone
// /node_modules/@turf/clusters
// /node_modules/@turf/clusters-dbscan
// /node_modules/@turf/clusters-kmeans
// /node_modules/@turf/collect
// /node_modules/@turf/combine
// /node_modules/@turf/concave
// /node_modules/@turf/convex
// /node_modules/@turf/destination
// /node_modules/@turf/difference
// /node_modules/@turf/dissolve
// /node_modules/@turf/distance
// /node_modules/@turf/distance-weight
// /node_modules/@turf/ellipse
// /node_modules/@turf/envelope
// /node_modules/@turf/explode
// /node_modules/@turf/flatten
// /node_modules/@turf/flip
// /node_modules/@turf/geojson-rbush
// /node_modules/@turf/great-circle
// /node_modules/@turf/helpers
// /node_modules/@turf/hex-grid
// /node_modules/@turf/interpolate
// /node_modules/@turf/intersect
// /node_modules/@turf/invariant
// /node_modules/@turf/isobands
// /node_modules/@turf/isolines
// /node_modules/@turf/jsts
// /node_modules/@turf/kinks
// /node_modules/@turf/length
// /node_modules/@turf/line-arc
// /node_modules/@turf/line-chunk
// /node_modules/@turf/line-intersect
// /node_modules/@turf/line-offset
// /node_modules/@turf/line-overlap
// /node_modules/@turf/line-segment
// /node_modules/@turf/line-slice
// /node_modules/@turf/line-slice-along
// /node_modules/@turf/line-split
// /node_modules/@turf/line-to-polygon
// /node_modules/@turf/mask
// /node_modules/@turf/meta
// /node_modules/@turf/midpoint
// /node_modules/@turf/moran-index
// /node_modules/@turf/nearest-neighbor-analysis
// /node_modules/@turf/nearest-point
// /node_modules/@turf/nearest-point-on-line
// /node_modules/@turf/nearest-point-to-line
// /node_modules/@turf/planepoint
// /node_modules/@turf/point-grid
// /node_modules/@turf/point-on-feature
// /node_modules/@turf/point-to-line-distance
// /node_modules/@turf/point-to-polygon-distance
// /node_modules/@turf/points-within-polygon
// /node_modules/@turf/polygon-smooth
// /node_modules/@turf/polygon-tangents
// /node_modules/@turf/polygon-to-line
// /node_modules/@turf/polygonize
// /node_modules/@turf/projection
// /node_modules/@turf/quadrat-analysis
// /node_modules/@turf/random
// /node_modules/@turf/rectangle-grid
// /node_modules/@turf/rewind
// /node_modules/@turf/rhumb-bearing
// /node_modules/@turf/rhumb-destination
// /node_modules/@turf/rhumb-distance
// /node_modules/@turf/sample
// /node_modules/@turf/sector
// /node_modules/@turf/shortest-path
// /node_modules/@turf/simplify
// /node_modules/@turf/square
// /node_modules/@turf/square-grid
// /node_modules/@turf/standard-deviational-ellipse
// /node_modules/@turf/tag
// /node_modules/@turf/tesselate
// /node_modules/@turf/tin
// /node_modules/@turf/transform-rotate
// /node_modules/@turf/transform-scale
// /node_modules/@turf/transform-translate
// /node_modules/@turf/triangle-grid
// /node_modules/@turf/truncate
// /node_modules/@turf/turf
// /node_modules/@turf/union
// /node_modules/@turf/unkink-polygon
// /node_modules/@turf/voronoi
// /node_modules/@types/d3-voronoi
// /node_modules/@types/geojson
//
//
// /node_modules/bignumber.js
// /node_modules/commander
// /node_modules/concaveman
// /node_modules/d3-array
// /node_modules/d3-geo
// /node_modules/d3-voronoi
// /node_modules/earcut
// /node_modules/fast-deep-equal
// /node_modules/geojson-equality-ts
// /node_modules/geojson-polygon-self-intersections
// /node_modules/geojson-polygon-self-intersections/node_modules/quickselect
// /node_modules/geojson-polygon-self-intersections/node_modules/rbush
// /node_modules/jsts
// /node_modules/marchingsquares
// /node_modules/point-in-polygon
// /node_modules/point-in-polygon-hao
// /node_modules/point-in-polygon-hao/node_modules/robust-predicates
// /node_modules/polyclip-ts
// /node_modules/quickselect
// /node_modules/rbush
// /node_modules/robust-predicates
// /node_modules/skmeans
// /node_modules/splaytree-ts
// /node_modules/sweepline-intersections
// /node_modules/tinyqueue
// /node_modules/topojson-client
// /node_modules/topojson-server
// /node_modules/tslib
