import { parseCSVAsRecord } from '../../';

import type { Feature, LineStringGeometry, MValue, Properties } from 's2json-spec';

/** Internal type for shape data for future use */
interface GTFSShape {
  id: string;
  lat: number;
  lon: number;
  sequence: number;
}

/** The properties for each shape */
export interface GTFSShapeProperties extends Properties {
  id: string;
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Agencies
 */
export function parseGTFSShapes(
  input: string,
): Record<string, Feature<undefined, MValue, GTFSShapeProperties, LineStringGeometry>> {
  const data = parseCSVAsRecord(input);
  // 1) Group data by shape_id
  const groups: Record<string, GTFSShape[]> = {};
  data.forEach((d) => {
    const { shape_id, shape_pt_lat, shape_pt_lon, shape_pt_sequence } = d;
    if (groups[shape_id] === undefined) groups[shape_id] = [];
    groups[shape_id].push({
      id: shape_id,
      lat: parseFloat(shape_pt_lat),
      lon: parseFloat(shape_pt_lon),
      sequence: parseInt(shape_pt_sequence, 10),
    });
  });
  // 2) for each shape_id group, sort data by shape_pt_sequence geometry
  const features: Record<
    string,
    Feature<undefined, MValue, GTFSShapeProperties, LineStringGeometry>
  > = {};
  for (const [shapeId, shapes] of Object.entries(groups)) {
    shapes.sort((a, b) => a.sequence - b.sequence);
    features[shapeId] = {
      type: 'Feature',
      geometry: {
        type: 'LineString',
        coordinates: shapes.map((s) => [s.lon, s.lat]),
      },
      properties: { id: shapeId },
    };
  }

  return features;
}
