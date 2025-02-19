import type { ProjectionTransform } from '.';
/**
 *
 */
export interface PJ_REGION_S {
  /* lower left corner coordinates (radians) */
  ll_long: number; // F64
  ll_lat: number; // F64
  /* upper right corner coordinates (radians) */
  ur_long: number; // F64
  ur_lat: number; // F64
}

/**
 *
 */
export class PJ_AREA {
  /**
   * @param bbox_set - if true, the area is defined by a bounding box
   * @param west_lon_degree - west longitude
   * @param south_lat_degree - south latitude
   * @param east_lon_degree - east longitude
   * @param north_lat_degree - north latitude
   * @param name - area name
   */
  constructor(
    public bbox_set = false,
    public west_lon_degree = 0, // F64
    public south_lat_degree = 0, // F64
    public east_lon_degree = 0, // F64
    public north_lat_degree = 0, // F64
    public name = '',
  ) {}
}

/**
 *
 */
export class CoordOperation {
  // idxInOriginalList: number; // I32

  // [min|max][x|y]Src define the bounding box of the area of validity of
  // the transformation, expressed in the source CRS. Except if the source
  // CRS is a geocentric one, in which case the bounding box is defined in
  // a geographic lon,lat CRS (pjSrcGeocentricToLonLat will have to be used
  // on input coordinates in the forward direction)
  // minxSrc = 0; // F64
  // minySrc = 0; // F64
  // maxxSrc = 0; // F64
  // maxySrc = 0; // F64

  // [min|max][x|y]Dst define the bounding box of the area of validity of
  // the transformation, expressed in the target CRS. Except if the target
  // CRS is a geocentric one, in which case the bounding box is defined in
  // a geographic lon,lat CRS (pjDstGeocentricToLonLat will have to be used
  // on input coordinates in the inverse direction)
  // minxDst = 0; // F64
  // minyDst = 0; // F64
  // maxxDst = 0; // F64
  // maxyDst = 0; // F64

  isOffshore = false;
  isUnknownAreaName = false;
  isPriorityOp = false;
  srcIsLonLatDegree = false;
  srcIsLatLonDegree = false;
  dstIsLonLatDegree = false;
  dstIsLatLonDegree = false;

  // pjSrcGeocentricToLonLat is defined if the source CRS of pj is geocentric
  // and in that case it transforms from those geocentric coordinates to
  // geographic ones in lon, lat order
  // pjSrcGeocentricToLonLat?: ProjectionTransform;

  // pjDstGeocentricToLonLat is defined if the target CRS of pj is geocentric
  // and in that case it transforms from those geocentric coordinates to
  // geographic ones in lon, lat order
  // pjDstGeocentricToLonLat?: ProjectionTransform;

  // TODO:
  /**
   * @param idxInOriginalList
   * @param minxSrc
   * @param minySrc
   * @param maxxSrc
   * @param maxySrc
   * @param minxDst
   * @param minyDst
   * @param maxxDst
   * @param maxyDst
   * @param pj
   * @param name
   * @param accuracy
   * @param pseudoArea
   * @param areaName
   * @param pjSrcGeocentricToLonLat
   * @param pjDstGeocentricToLonLat
   */
  constructor(
    public idxInOriginalList: number, // I32
    public minxSrc = 0, // F64
    public minySrc = 0, // F64
    public maxxSrc = 0, // F64
    public maxySrc = 0, // F64
    public minxDst = 0, // F64
    public minyDst = 0, // F64
    public maxxDst = 0, // F64
    public maxyDst = 0, // F64
    public pj?: ProjectionTransform,
    public name = '',
    public accuracy = -1, // F64
    public pseudoArea = 0, // F64
    public areaName = '',
    public pjSrcGeocentricToLonLat?: ProjectionTransform,
    public pjDstGeocentricToLonLat?: ProjectionTransform,
  ) {
    // TODO:
    // : idxInOriginalList(idxInOriginalListIn), minxSrc(minxSrcIn),
    //   minySrc(minySrcIn), maxxSrc(maxxSrcIn), maxySrc(maxySrcIn),
    //   minxDst(minxDstIn), minyDst(minyDstIn), maxxDst(maxxDstIn),
    //   maxyDst(maxyDstIn), pj(pjIn), name(nameIn), accuracy(accuracyIn),
    //   pseudoArea(pseudoAreaIn), areaName(areaNameIn ? areaNameIn : ""),
    //   isOffshore(areaName.find("- offshore") != std::string::npos),
    //   isUnknownAreaName(areaName.empty() || areaName == "unknown"),
    //   isPriorityOp(isSpecialCaseForNAD83_to_NAD83HARN(name) ||
    //                isSpecialCaseForGDA94_to_WGS84(name) ||
    //                isSpecialCaseForWGS84_to_GDA2020(name)),
    //   pjSrcGeocentricToLonLat(pjSrcGeocentricToLonLatIn
    //                               ? proj_clone(pjSrcGeocentricToLonLatIn->ctx,
    //                                            pjSrcGeocentricToLonLatIn)
    //                               : nullptr),
    //   pjDstGeocentricToLonLat(pjDstGeocentricToLonLatIn
    //                               ? proj_clone(pjDstGeocentricToLonLatIn->ctx,
    //                                            pjDstGeocentricToLonLatIn)
    //                               : nullptr) {
    // const auto IsLonLatOrLatLon = [](const PJ *crs, bool &isLonLatDegreeOut,
    //                                  bool &isLatLonDegreeOut)
    //     const auto eType = proj_get_type(crs);
    //     if (eType == PJ_TYPE_GEOGRAPHIC_2D_CRS ||
    //         eType == PJ_TYPE_GEOGRAPHIC_3D_CRS) {
    //         const auto cs = proj_crs_get_coordinate_system(crs->ctx, crs);
    //         const char *direction = "";
    //         double conv_factor = 0;
    //         constexpr double EPS = 1e-14;
    //         if (proj_cs_get_axis_info(crs->ctx, cs, 0, nullptr, nullptr,
    //                                   &direction, &conv_factor, nullptr,
    //                                   nullptr, nullptr) &&
    //             ci_equal(direction, "East")) {
    //             isLonLatDegreeOut = fabs(conv_factor - M_PI / 180) < EPS;
    //         } else if (proj_cs_get_axis_info(crs->ctx, cs, 1, nullptr, nullptr,
    //                                          &direction, &conv_factor, nullptr,
    //                                          nullptr, nullptr) &&
    //                    ci_equal(direction, "East")) {
    //             isLatLonDegreeOut = fabs(conv_factor - M_PI / 180) < EPS;
    //         }
    //         proj_destroy(cs);
    //     }
    // };
    // const auto source = proj_get_source_crs(pj->ctx, pj);
    // if (source) {
    //     IsLonLatOrLatLon(source, srcIsLonLatDegree, srcIsLatLonDegree);
    //     proj_destroy(source);
    // }
    // const auto target = proj_get_target_crs(pj->ctx, pj);
    // if (target) {
    //     IsLonLatOrLatLon(target, dstIsLonLatDegree, dstIsLatLonDegree);
    //     proj_destroy(target);
    // }
  }

  // PJCoordOperation(PJ_CONTEXT *ctx, const PJCoordOperation &other)
  //     : idxInOriginalList(other.idxInOriginalList), minxSrc(other.minxSrc),
  //       minySrc(other.minySrc), maxxSrc(other.maxxSrc),
  //       maxySrc(other.maxySrc), minxDst(other.minxDst),
  //       minyDst(other.minyDst), maxxDst(other.maxxDst),
  //       maxyDst(other.maxyDst), pj(proj_clone(ctx, other.pj)),
  //       name(std::move(other.name)), accuracy(other.accuracy),
  //       pseudoArea(other.pseudoArea), areaName(other.areaName),
  //       isOffshore(other.isOffshore),
  //       isUnknownAreaName(other.isUnknownAreaName),
  //       isPriorityOp(other.isPriorityOp),
  //       srcIsLonLatDegree(other.srcIsLonLatDegree),
  //       srcIsLatLonDegree(other.srcIsLatLonDegree),
  //       dstIsLonLatDegree(other.dstIsLonLatDegree),
  //       dstIsLatLonDegree(other.dstIsLatLonDegree),
  //       pjSrcGeocentricToLonLat(
  //           other.pjSrcGeocentricToLonLat
  //               ? proj_clone(ctx, other.pjSrcGeocentricToLonLat)
  //               : nullptr),
  //       pjDstGeocentricToLonLat(
  //           other.pjDstGeocentricToLonLat
  //               ? proj_clone(ctx, other.pjDstGeocentricToLonLat)
  //               : nullptr) {}

  // PJCoordOperation(PJCoordOperation &&other)
  //     : idxInOriginalList(other.idxInOriginalList), minxSrc(other.minxSrc),
  //       minySrc(other.minySrc), maxxSrc(other.maxxSrc),
  //       maxySrc(other.maxySrc), minxDst(other.minxDst),
  //       minyDst(other.minyDst), maxxDst(other.maxxDst),
  //       maxyDst(other.maxyDst), name(std::move(other.name)),
  //       accuracy(other.accuracy), pseudoArea(other.pseudoArea),
  //       areaName(std::move(other.areaName)), isOffshore(other.isOffshore),
  //       isUnknownAreaName(other.isUnknownAreaName),
  //       isPriorityOp(other.isPriorityOp),
  //       srcIsLonLatDegree(other.srcIsLonLatDegree),
  //       srcIsLatLonDegree(other.srcIsLatLonDegree),
  //       dstIsLonLatDegree(other.dstIsLonLatDegree),
  //       dstIsLatLonDegree(other.dstIsLatLonDegree) {
  //     pj = other.pj;
  //     other.pj = nullptr;
  //     pjSrcGeocentricToLonLat = other.pjSrcGeocentricToLonLat;
  //     other.pjSrcGeocentricToLonLat = nullptr;
  //     pjDstGeocentricToLonLat = other.pjDstGeocentricToLonLat;
  //     other.pjDstGeocentricToLonLat = nullptr;
  // }

  // PJCoordOperation &operator=(const PJCoordOperation &) = delete;

  // bool operator==(const PJCoordOperation &other) const {
  //     return idxInOriginalList == other.idxInOriginalList &&
  //            minxSrc == other.minxSrc && minySrc == other.minySrc &&
  //            maxxSrc == other.maxxSrc && maxySrc == other.maxySrc &&
  //            minxDst == other.minxDst && minyDst == other.minyDst &&
  //            maxxDst == other.maxxDst && maxyDst == other.maxyDst &&
  //            name == other.name &&
  //            proj_is_equivalent_to(pj, other.pj, PJ_COMP_STRICT) &&
  //            accuracy == other.accuracy && areaName == other.areaName;
  // }

  //   bool operator!=(const PJCoordOperation &other) const {
  //     return !(operator==(other));
  // }
}
