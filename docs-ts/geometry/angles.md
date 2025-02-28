<h1 style="text-align: center;">
  <div align="center">Geometry - Angles</div>
</h1>

<p align="center">
  <img src="../../assets/badges/angles-file.svg" alt="angles-file-ts">
  <img src="../../assets/badges/angles-gzip.svg" alt="angles-gzip-ts">
  <img src="../../assets/badges/angles-brotli.svg" alt="angles-brotli-ts">
</p>

# S1Angle

## S1Angle Description

The internal representation is a double-precision value in radians. This class represents a one-dimensional angle (as opposed to a two-dimensional solid angle). It has methods for converting angles to or from radians, degrees, and the E5/E6/E7 representations (i.e. degrees multiplied by 1e5/1e6/1e7 and rounded to the nearest integer).

## S1Angle Usage

### Create an S1Angle from degrees

```ts
import { angleFromDegrees } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = angleFromDegrees(45);
```

### Convert an S1Angle to degrees

```ts
import { angleToDegrees } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = 0;
const degrees: number = angleToDegrees(angle);
```

### Build an angle in E5 format

```ts
import { angleToE5 } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = angleToE5(45);
```

### Build an angle in E6 format

```ts
import { angleToE6 } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = angleToE6(45);
```

### Build an angle in E7 format

```ts
import { angleToE7 } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = angleToE7(45);
```

### Convert two S2Points to an S1Angle

Return the angle between two points, which is also equal to the distance between these points on the unit sphere.  The points do not need to be normalized.  This function has a maximum error of 3.25 *DBL_EPSILON (or 2.5* DBL_EPSILON for angles up to 1 radian). If either point is zero-length (e.g. an uninitialized S2Point), or almost zero-length, the resulting angle will be zero.

```ts
import { angleFromS2Points } from 'gis-tools-ts';
import type { S1Angle, VectorPoint } from 'gis-tools-ts';

const p1: VectorPoint = { x: 0, y: -1, z: 0 };
const p2: VectorPoint = { x: 1, y: 0, z: 0 };
const angle: S1Angle = angleFromS2Points(p1, p2);
```

### Convert two Lon-Lat to an S1Angle

Like the constructor above, but return the angle (i.e., distance) between two S2LatLng points.  This function has about 15 digits of accuracy for small distances but only about 8 digits of accuracy as the distance approaches 180 degrees (i.e., nearly-antipodal points).

```ts
import { angleFromS2Points } from 'gis-tools-ts';
import type { S1Angle, LonLat } from 'gis-tools-ts';

const p1: LonLat = { x: 0, y: -1 };
const p2: LonLat = { x: 1, y: 0 };
const angle: S1Angle = angleFromS2Points(p1, p2);
```

### Convert an angle in radians to an distance in meters

```ts
import { angleToMeters } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = 0;
const meters: number = angleToMeters(angle);
```

### Convert an distance in meters to an angle in radians

```ts
import { angleFromMeters } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const meters: number = 10;
const radians: S1Angle = angleFromMeters(meters);
```

### Convert an angle in radians to an distance in kilometers

```ts
import { angleToKM } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = 0;
const km: number = angleToKM(angle);
```

### Convert an distance in kilometers to an angle in radians

```ts
import { angleFromKM } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const km: number = 10;
const radians: S1Angle = angleFromKM(km);
```

### Build an angle into an E5 format

```ts
import { angleE5 } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = 0.0;
const e5: number = angleE5(angle);
```

### Build an angle into an E6 format

```ts
import { angleE6 } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = 0.0;
const e6: number = angleE6(angle);
```

### Build an angle into an E7 format

```ts
import { angleE7 } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = 0.0;
const e7: number = angleE7(angle);
```

### Normalize this angle to the range `(-180, 180]` degrees

```ts
import { angleNormalize } from 'gis-tools-ts';
import type { S1Angle } from 'gis-tools-ts';

const angle: S1Angle = 0.0;
const normalizedAngle: S1Angle = angleNormalize(angle);
```

# S1ChordAngle

## S1ChordAngle Description

S1ChordAngle represents the angle subtended by a chord (i.e., the straight line segment connecting two points on the sphere).  Its representation makes it very efficient for computing and comparing distances, but unlike S1Angle it is only capable of representing angles between 0 and Pi radians. S1ChordAngle is intended for applications where many angles need to be computed and compared, otherwise it is simpler to use S1Angle. [See the full description in the ts-docs.](https://open-s2.github.io/gis-tools/types/index.S1ChordAngle.html).

## S1ChordAngle Usage

### Convert an S1Angle to an S1ChordAngle

Conversion from an S1Angle.  Angles outside the range `[0, Pi]` are handled as follows: Infinity() is mapped to Infinity(), negative angles are mapped to Negative(), and finite angles larger than Pi are mapped to Straight().

```ts
import { chordAngFromAngle } from 'gis-tools-ts';
import type { S1Angle, S1ChordAngle } from 'gis-tools-ts';

const angle: S1Angle = 0.0;
const chordAngle: S1ChordAngle = chordAngFromAngle(angle);
```

### Construct an S1ChordAngle from the squared chord length

Note that the argument is automatically clamped to a maximum of 4.0 to handle possible roundoff errors.  The argument must be non-negative.

```ts
import { chordAngFromLength2 } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const length2: number = 1.0;
const chordAngle: S1ChordAngle = chordAngFromLength2(length2);
```

### Construct the S1ChordAngle corresponding to the distance between the two given points

The points must be unit length.

```ts
import { chordAngFromS2Points } from 'gis-tools-ts';
import type { S1ChordAngle, VectorPoint } from 'gis-tools-ts';

const p1: VectorPoint = { x: 0, y: -1, z: 0 };
const p2: VectorPoint = { x: 1, y: 0, z: 0 };
const chordAngle: S1ChordAngle = chordAngFromS2Points(p1, p2);
```

### Return a chord angle of 90 degrees (a "right angle")

```ts
import { chordAngRightAngle } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = chordAngRightAngle();
```

### Return a chord angle of 180 degrees (a "straight angle")

This is the maximum finite chord angle.

```ts
import { chordAngStraightAngle } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = chordAngStraightAngle();
```

### Return a chord angle smaller than Zero()

The only valid operations on Negative() are comparisons, S1Angle conversions, and successor() / predecessor().

```ts
import { chordAngNegativeAngle } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = chordAngNegativeAngle();
```

### Construct an S1ChordAngle that is an upper bound on the given S1Angle

i.i. such that FastUpperBoundFrom(x).toAngle() >= x. Unlike the S1Angle constructor above, this method is very fast, and the bound is accurate to within 1% for distances up to about 3100km on the Earth's surface.

```ts
import { chordAngFastUpperBoundFrom } from 'gis-tools-ts';
import type { S1Angle, S1ChordAngle } from 'gis-tools-ts';

const angle: S1Angle = 0.0;
const chordAngle: S1ChordAngle = chordAngFastUpperBoundFrom(angle);
```

### Convenience function to test if a ChordAngle is special

```ts
import { chordAngIsSpecial } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = 0.0;
const isSpecial: boolean = chordAngIsSpecial(chordAngle);
```

### Convert an S1ChordAngle to an S1Angle

Infinity() is converted to S1Angle.Infinity(), and Negative() is converted to an unspecified negative S1Angle.

```ts
import { chordAngToAngle } from 'gis-tools-ts';
import type { S1ChordAngle, S1Angle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = 0.0;
const angle: S1Angle = chordAngToAngle(chordAngle);
```

### Convert an S1ChordAngle to meters

```ts
import { chordAngToMeters } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = 0.0;
const meters: number = chordAngToMeters(chordAngle);
```

### Convert an S1ChordAngle from meters

```ts
import { chordAngFromMeters } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const meters: number = 0.0;
const chordAngle: S1ChordAngle = chordAngFromMeters(meters);
```

### Convert an S1ChordAngle to kilometers

```ts
import { chordAngToKM } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = 0.0;
const kilometers: number = chordAngToKM(chordAngle);
```

### Convert an S1ChordAngle from kilometers

```ts
import { chordAngFromKM } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const kilometers: number = 0.0;
const chordAngle: S1ChordAngle = chordAngFromKM(kilometers);
```

### Apply a sine function on a S1ChordAngle

```ts
import { chordAngSin } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = 0.0;
const sin: number = chordAngSin(chordAngle);
```

### Apply a cosine function on a S1ChordAngle

```ts
import { chordAngCos } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = 0.0;
const cos: number = chordAngCos(chordAngle);
```

### Apply a tangent function on a S1ChordAngle

```ts
import { chordAngTan } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = 0.0;
const tan: number = chordAngTan(chordAngle);
```

### Returns sin(a)^2 on a S1ChordAngle, but computed more efficiently

```ts
import { chordAngSin2 } from 'gis-tools-ts';
import type { S1ChordAngle } from 'gis-tools-ts';

const chordAngle: S1ChordAngle = 0.0;
const sin2: number = chordAngSin2(chordAngle);
```
