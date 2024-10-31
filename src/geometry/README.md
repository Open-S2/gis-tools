# Geometry

The goal here is only to build code for the `s2json-spec` package while also providing additional geometry utilities necessary for other modules.

## TODO

### Generic

- [ ] **clean**: remove redundant points, (multi)linestring and/or (multi)polygon, fixes kinks, etc.

### Points

- [ ] **center**: find centerpoint of multi-point

### Lines

- [ ] **along**: (given linestring find the point at distance provided)
- [ ] **length**: length of a linestring
- [ ] **pointToLineDistance**: distance from a point to a line
- [ ] **bezierSpline**: create a bezier spline from a line

### Polygons

- [ ] **area**: area of a polygon
- [ ] **length**: length of each polygon ring
- [ ] **pointInPolygon**: test if a point is in a polygon
- [ ] **dekink**: remove kinks from a polygon
- [ ] **boolean**: boolean operations on polygons
