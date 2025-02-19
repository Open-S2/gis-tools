import type { ProjectionTransform } from '../projections';

// Geodetic transformations are typically organized in a number of
//     steps. For example, a datum shift could be carried out through
//     these steps:
//
//     1. Convert (latitude, longitude, ellipsoidal height) to
//        3D geocentric cartesian coordinates (X, Y, Z)
//     2. Transform the (X, Y, Z) coordinates to the new datum, using a
//        7 parameter Helmert transformation.
//     3. Convert (X, Y, Z) back to (latitude, longitude, ellipsoidal height)
//
//     If the height system used is orthometric, rather than ellipsoidal,
//     another step is needed at each end of the process:
//
//     1. Add the local geoid undulation (N) to the orthometric height
//        to obtain the ellipsoidal (i.e. geometric) height.
//     2. Convert (latitude, longitude, ellipsoidal height) to
//        3D geocentric cartesian coordinates (X, Y, Z)
//     3. Transform the (X, Y, Z) coordinates to the new datum, using a
//        7 parameter Helmert transformation.
//     4. Convert (X, Y, Z) back to (latitude, longitude, ellipsoidal height)
//     5. Subtract the local geoid undulation (N) from the ellipsoidal height
//        to obtain the orthometric height.
//
//     Additional steps can be added for e.g. change of vertical datum, so the
//     list can grow fairly long. None of the steps are, however, particularly
//     complex, and data flow is strictly from top to bottom.
//
//     Hence, in principle, the first example above could be implemented using
//     Unix pipelines:
//
//     cat my_coordinates | geographic_to_xyz | helmert | xyz_to_geographic >
// my_transformed_coordinates
//
//     in the grand tradition of Software Tools [1].
//
//     The proj pipeline driver implements a similar concept: Stringing together
//     a number of steps, feeding the output of one step to the input of the next.
//
//     It is a very powerful concept, that increases the range of relevance of the
//     proj.4 system substantially. It is, however, not a particularly intrusive
//     addition to the PROJ.4 code base: The implementation is by and large
// completed by adding an extra projection called "pipeline" (i.e. this file),
// which handles all business, and a small amount of added functionality in the
//     pj_init code, implementing support for multilevel, embedded pipelines.
//
//     Syntactically, the pipeline system introduces the "+step" keyword (which
//     indicates the start of each transformation step), and reintroduces the +inv
//     keyword (indicating that a given transformation step should run in reverse,
// i.e. forward, when the pipeline is executed in inverse direction, and vice
// versa).
//
//     Hence, the first transformation example above, can be implemented as:
//
//     +proj=pipeline +step proj=cart +step proj=helmert <ARGS> +step proj=cart
// +inv
//
//     Where <ARGS> indicate the Helmert arguments: 3 translations (+x=..., +y=...,
//     +z=...), 3 rotations (+rx=..., +ry=..., +rz=...) and a scale factor
// (+s=...). Following geodetic conventions, the rotations are given in arcseconds,
//     and the scale factor is given as parts-per-million.
//
//     [1] B. W. Kernighan & P. J. Plauger: Software tools.
//         Reading, Massachusetts, Addison-Wesley, 1976, 338 pp.
//
// Transformation pipeline manager

/**
 *
 */
export interface Step {
  // PJ *pj = nullptr;
  // bool omit_fwd = false;
  // bool omit_inv = false;

  // Step(PJ *pjIn, bool omitFwdIn, bool omitInvIn)
  //     : pj(pjIn), omit_fwd(omitFwdIn), omit_inv(omitInvIn) {}
  // Step(Step &&other)
  //     : pj(std::move(other.pj)), omit_fwd(other.omit_fwd),
  //       omit_inv(other.omit_inv) {
  //     other.pj = nullptr;
  // }
  // Step(const Step &) = delete;
  // Step &operator=(const Step &) = delete;

  // ~Step() { proj_destroy(pj); }
  pj?: ProjectionTransform;
  omit_fwd: boolean; // default to false
  omit_inv: boolean; // default to false
}
