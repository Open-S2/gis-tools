<h1 style="text-align: center;">
  <div align="center">PROJ Projections</div>
</h1>

<p align="center">
  <img src="../../assets/badges/projections-file.svg" alt="projections-file-ts">
  <img src="../../assets/badges/projections-gzip.svg" alt="projections-gzip-ts">
  <img src="../../assets/badges/projections-brotli.svg" alt="projections-brotli-ts">
</p>

## Description

Projections are added to a transformer using the `insertDefinition` method or in the [Transformer](./transformer.md) constructor as the third parameter (you can add multiple in an array).

## Usage

```ts
import { Transformer, HotineObliqueMercator, EPSG_8803 } from 'gis-tools-ts';

const transform = new Transformer(undefined, 'EPSG_8803', [HotineObliqueMercator], { EPSG_8803 });
// or add to an existing transformer
transform.insertDefinition(HotineObliqueMercator);
// example moving from WGS84 to EPSG:8803
const forward = transform.forward({ x: 60.8, y: -132.2 });
```

Often missing projection errors will provide the short name of the projection. So instead of `HotineObliqueMercator` it will probably report `omerc` is missing. Example:

```sh
Error: "omerc" invalid, unsupported, or not loaded
```

Here is a useful lookup map. Click the import name for the TS documentation:

| Import Name | Short Name |
| --- | --- |
| [AlbersConicEqualArea] | `aea`, `Albers`, `Albers_Conic_Equal_Area` |
| [AzimuthalEquidistant] | `aeqd`, `Azimuthal_Equidistant` |
| [BonneWerner] | `bonne`, `bonne_werner` |
| [CassiniSoldner] | `cass`, `Cassini`, `Cassini_Soldner` |
| [CylindricalEqualArea] | `cea`, `Equal_Area_Cylindrical` |
| [EquidistantCylindrical] | `eqc`, `Equidistant_Cylindrical`, `Equirectangular`, `Equidistant Cylindrical (Plate Carre)` |
| [EquidistantConic] | `eqdc`, `Equidistant_Conic` |
| [EqualEarth] | `eqearth`, `Equal_Earth`, `EqualEarth`, `Equal_Area_Cylindrical`, `Equal Earth` |
| [EquiRectangular] | `equi`, `EquiRectangular` |
| [ExtendedTransverseMercator] | `etmerc`, `ExtendedTransverseMercator`, `Extended_Transverse_Mercator`, `Extended Transverse Mercator` |
| [GaussKruger] | `gauss`, `Gauss_Kruger`, `GaussKruger`, `Gauss Kruger` |
| [Geocentric] | `geocent`, `Geocentric` |
| [GeostationarySatelliteView] | `geos`, `Geostationary_Satellite`, `Geostationary Satellite View`, `GeostationarySatelliteView` |
| [Gnomonic] | `gnom`, `Gnomonic`, `GnomonicProjection`, `Gnomonic Projection` |
| [GaussSchreiberTransverseMercator] | `gstmerc`, `gstmerg`, `GaussSchreiberTransverseMercator`, `Gauss-Schreiber Transverse Mercator`, `Gauss_Schreiber_Transverse_Mercator` |
| [Krovak] | `krovak` |
| [LambertAzimuthalEqualArea] | `laea`, `Lambert_Azimuthal_Equal_Area`, `Lambert Azimuthal Equal Area`, `LambertAzimuthalEqualArea` |
| [LambertConformalConic] | `lcc`, `Lambert_Conformal_Conic`, `Lambert Conformal Conic`, `LambertConformalConic`, `Lambert_Conformal_Conic_1SP`, `Lambert_Conformal_Conic_2SP` |
| [Mercator] | `merc`, `Mercator`, `Mercator_1SP`, `Mercator_Auxiliary_Sphere` |
| [MillerCylindrical] | `mill`, `Miller_Cylindrical`, `Miller Cylindrical`, `MillerCylindrical` |
| [Mollweide] | `moll`, `Mollweide` |
| [NewZealandMapGrid] | `nzmg`, `New_Zealand_Map_Grid`, `NewZealandMapGrid` |
| [HotineObliqueMercator] | `omerc`, `Hotine_Oblique_Mercator`, `HotineObliqueMercator`, `Hotine Oblique Mercator`, `Oblique_Mercator` |
| [Orthographic] | `ortho`, `Orthographic` |
| [Polyconic] | `poly`, `Polyconic` |
| [QuadrilateralizedSphericalCube] | `qsc`, `Quadrilateralized_Spherical_Cube`, `QuadrilateralizedSphericalCube`, `Quadrilateralized_Spherical_Cube` |
| [Robinson] | `robin`, `Robinson` |
| [Sinusoidal] | `sinu`, `Sinusoidal` |
| [SwissObliqueMercator] | `somerc`, `Swiss_Oblique_Mercator`, `SwissObliqueMercator`, `Swiss Oblique Mercator` |
| [StereographicSouthPole] | `stere`, `Stereographic_South_Pole`, `StereographicSouthPole`, `Stereographic South Pole`, `Polar_Stereographic` |
| [StereographicNorthPole] | `sterea`, `Oblique_Stereographic`, `StereographicNorthPole`, `Stereographic North Pole`, `Stereographic_North_Pole` |
| [TransverseMercator] | `tmerc`, `Transverse_Mercator`, `TransverseMercator`, `Transverse Mercator` |
| [TiltedPerspective] | `tpers`, `TiltedPerspective`, `Tilted_Perspective` |
| [UniversalTransverseMercator] | `utm`, `Universal_Transverse_Mercator`, `UniversalTransverseMercator`, `Universal Transverse Mercator` |
| [VanDerGrinten] | `vandg`, `VanDerGrinten`, `Van_der_Grinten_I` |

[AlbersConicEqualArea]: https://open-s2.github.io/gis-tools/classes/index.AlbersConicEqualArea.html
[AzimuthalEquidistant]: https://open-s2.github.io/gis-tools/classes/index.AzimuthalEquidistant.html
[BonneWerner]: https://open-s2.github.io/gis-tools/classes/index.BonneWerner.html
[CassiniSoldner]: https://open-s2.github.io/gis-tools/classes/index.CassiniSoldner.html
[CylindricalEqualArea]: https://open-s2.github.io/gis-tools/classes/index.CylindricalEqualArea.html
[EquidistantCylindrical]: https://open-s2.github.io/gis-tools/classes/index.EquidistantCylindrical.html
[EquidistantConic]: https://open-s2.github.io/gis-tools/classes/index.EquidistantConic.html
[EqualEarth]: https://open-s2.github.io/gis-tools/classes/index.EqualEarth.html
[EquiRectangular]: https://open-s2.github.io/gis-tools/classes/index.EquiRectangular.html
[ExtendedTransverseMercator]: https://open-s2.github.io/gis-tools/classes/index.ExtendedTransverseMercator.html
[GaussKruger]: https://open-s2.github.io/gis-tools/classes/index.GaussKruger.html
[Geocentric]: https://open-s2.github.io/gis-tools/classes/index.Geocentric.html
[GeostationarySatelliteView]: https://open-s2.github.io/gis-tools/classes/index.GeostationarySatelliteView.html
[Gnomonic]: https://open-s2.github.io/gis-tools/classes/index.Gnomonic.html
[GaussSchreiberTransverseMercator]: https://open-s2.github.io/gis-tools/classes/index.GaussSchreiberTransverseMercator.html
[Krovak]: https://open-s2.github.io/gis-tools/classes/index.Krovak.html
[LambertAzimuthalEqualArea]: https://open-s2.github.io/gis-tools/classes/index.LambertAzimuthalEqualArea.html
[LambertConformalConic]: https://open-s2.github.io/gis-tools/classes/index.LambertConformalConic.html
[Mercator]: https://open-s2.github.io/gis-tools/classes/index.Mercator.html
[MillerCylindrical]: https://open-s2.github.io/gis-tools/classes/index.MillerCylindrical.html
[Mollweide]: https://open-s2.github.io/gis-tools/classes/index.Mollweide.html
[NewZealandMapGrid]: https://open-s2.github.io/gis-tools/classes/index.NewZealandMapGrid.html
[HotineObliqueMercator]: https://open-s2.github.io/gis-tools/classes/index.HotineObliqueMercator.html
[Orthographic]: https://open-s2.github.io/gis-tools/classes/index.Orthographic.html
[Polyconic]: https://open-s2.github.io/gis-tools/classes/index.Polyconic.html
[QuadrilateralizedSphericalCube]: https://open-s2.github.io/gis-tools/classes/index.QuadrilateralizedSphericalCube.html
[Robinson]: https://open-s2.github.io/gis-tools/classes/index.Robinson.html
[Sinusoidal]: https://open-s2.github.io/gis-tools/classes/index.Sinusoidal.html
[SwissObliqueMercator]: https://open-s2.github.io/gis-tools/classes/index.SwissObliqueMercator.html
[StereographicSouthPole]: https://open-s2.github.io/gis-tools/classes/index.StereographicSouthPole.html
[StereographicNorthPole]: https://open-s2.github.io/gis-tools/classes/index.StereographicNorthPole.html
[TransverseMercator]: https://open-s2.github.io/gis-tools/classes/index.TransverseMercator.html
[TiltedPerspective]: https://open-s2.github.io/gis-tools/classes/index.TiltedPerspective.html
[UniversalTransverseMercator]: https://open-s2.github.io/gis-tools/classes/index.UniversalTransverseMercator.html
[VanDerGrinten]: https://open-s2.github.io/gis-tools/classes/index.VanDerGrinten.html
