#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{clean_polygons, convert};
    use s2json::{BBox, JSONCollection, Projection, VectorGeometry, VectorMultiPolygon};
    use std::fs;
    use std::{mem::take, path::PathBuf};

    /// Reads a geojson file into a VectorMultiPolygon
    /// NOTE: Actually writing/testing failed test case has all been done in the Typescript equivalent
    fn get_data(folder: String, name: Option<String>) -> VectorMultiPolygon {
        let name = name.unwrap_or("args".to_string());
        // pull in the geojson
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("tests/geometry/tools/fixtures/{folder}/{name}.geojson"));
        let geojson_data = fs::read_to_string(path).unwrap();
        let feature_collection: JSONCollection = serde_json::from_str(&geojson_data).unwrap();

        // build the vector data
        let mut vector_features =
            convert(Projection::WG, &feature_collection, Some(true), Some(false));

        // shape to result
        let mut res: VectorMultiPolygon = vec![];
        for feature in vector_features.iter_mut() {
            match &mut feature.geometry {
                VectorGeometry::Polygon(p) => res.push(take(&mut p.coordinates)),
                VectorGeometry::MultiPolygon(p) => res.extend(p.coordinates.drain(..)),
                _ => {}
            }
        }

        res
    }

    // fn assert_nested_approx(a: &VectorMultiPolygon, b: &VectorMultiPolygon, eps: f64) {
    //     assert_eq!(a.len(), b.len());
    //     for (pa, pb) in a.iter().zip(b) {
    //         assert_eq!(pa.len(), pb.len(), "\nleft: {:?}\n\nright: {:?}", a, b);
    //         for (ra, rb) in pa.iter().zip(pb) {
    //             assert_eq!(ra.len(), rb.len());
    //             for (p1, p2) in ra.iter().zip(rb) {
    //                 assert!(
    //                     (p1.x - p2.x).abs() <= eps,
    //                     "x diff: {} vs {}\n\nleft: {:?}\n\nright: {:?}",
    //                     p1.x,
    //                     p2.x,
    //                     a,
    //                     b
    //                 );
    //                 assert!(
    //                     (p1.y - p2.y).abs() <= eps,
    //                     "y diff: {} vs {}\n\nleft: {:?}\n\nright: {:?}",
    //                     p1.y,
    //                     p2.y,
    //                     a,
    //                     b
    //                 );
    //             }
    //         }
    //     }
    // }

    macro_rules! clean_poly_test {
        ($name:ident, $folder:expr) => {
            #[test]
            fn $name() {
                let input = get_data($folder.to_string(), None);
                let output = get_data($folder.to_string(), Some("cleaned".into()));
                let (cleaned, _) = clean_polygons(&input, false, false, false)
                    .unwrap_or((vec![], BBox::default()));
                // assert_nested_approx(&cleaned, &output, 1e-7);
                assert_eq!(cleaned, output);
            }
        };
    }

    clean_poly_test!(
        clean_polygons_almost_colinear_segments_but_not,
        "almost-colinear-segments-but-not"
    );
    clean_poly_test!(
        clean_polygons_almost_colinear_segments_but_not_2,
        "almost-colinear-segments-but-not-2"
    );
    clean_poly_test!(clean_polygons_almost_parrallel_segments, "almost-parrallel-segments");
    clean_poly_test!(clean_polygons_almost_parrallel_segments_2, "almost-parrallel-segments-2");
    clean_poly_test!(clean_polygons_almost_parrallel_segments_3, "almost-parrallel-segments-3");
    clean_poly_test!(
        clean_polygons_clean_multipoly_with_polys_overlapping,
        "clean-multipoly-with-polys-overlapping"
    );
    clean_poly_test!(
        clean_polygons_clean_multipoly_with_polys_touching,
        "clean-multipoly-with-polys-touching"
    );
    clean_poly_test!(
        clean_polygons_clean_poly_with_backward_ring_winding_order,
        "clean-poly-with-backward-ring-winding-order"
    );
    clean_poly_test!(
        clean_polygons_clean_poly_with_repeated_and_extra_points,
        "clean-poly-with-repeated-and-extra-points"
    );
    clean_poly_test!(clean_polygons_collapsed_edges_removed, "collapsed-edges-removed");
    clean_poly_test!(clean_polygons_disjoint_union, "disjoint-union");
    clean_poly_test!(clean_polygons_dont_consume_prev_segment_1, "dont-consume-prev-segment-1");
    clean_poly_test!(clean_polygons_dont_consume_prev_segment_2, "dont-consume-prev-segment-2");
    clean_poly_test!(clean_polygons_dont_consume_prev_segment_3, "dont-consume-prev-segment-3");
    clean_poly_test!(clean_polygons_double_overlap, "double-overlap");
    clean_poly_test!(clean_polygons_empty_multipoly, "empty-multipoly");
    clean_poly_test!(clean_polygons_high_coincidence, "high-coincidence");
    clean_poly_test!(clean_polygons_hole_from_outers_bug, "hole-from-outers-bug");
    clean_poly_test!(clean_polygons_hole_interacts_outer, "hole-interacts-outer");
    clean_poly_test!(clean_polygons_infinitely_thin_polygon, "infinitely-thin-polygon");
    clean_poly_test!(clean_polygons_intersection_after_remove_1, "intersection-after-remove-1");
    clean_poly_test!(clean_polygons_intersection_after_remove_2, "intersection-after-remove-2");
    clean_poly_test!(clean_polygons_island_in_hole_4x, "island-in-hole-4x");
    clean_poly_test!(clean_polygons_issue_1, "issue-1");
    clean_poly_test!(clean_polygons_issue_36, "issue-36");
    clean_poly_test!(clean_polygons_issue_37, "issue-37");
    clean_poly_test!(clean_polygons_issue_38, "issue-38");
    clean_poly_test!(clean_polygons_issue_44, "issue-44");
    clean_poly_test!(clean_polygons_issue_60, "issue-60");
    clean_poly_test!(clean_polygons_issue_60_2, "issue-60-2");
    clean_poly_test!(clean_polygons_issue_60_3, "issue-60-3");
    clean_poly_test!(clean_polygons_issue_60_4, "issue-60-4");
    clean_poly_test!(clean_polygons_issue_60_5, "issue-60-5");
    clean_poly_test!(clean_polygons_issue_60_6, "issue-60-6");
    clean_poly_test!(clean_polygons_issue_60_7, "issue-60-7");
    clean_poly_test!(clean_polygons_issue_60_8, "issue-60-8");
    clean_poly_test!(clean_polygons_issue_61, "issue-61");
    clean_poly_test!(clean_polygons_issue_61_2, "issue-61-2");
    clean_poly_test!(clean_polygons_issue_62, "issue-62");
    clean_poly_test!(clean_polygons_issue_62_2, "issue-62-2");
    clean_poly_test!(clean_polygons_issue_66, "issue-66");
    clean_poly_test!(clean_polygons_issue_68, "issue-68");
    clean_poly_test!(clean_polygons_issue_68_1, "issue-68-1");
    clean_poly_test!(clean_polygons_issue_75, "issue-75");
    clean_poly_test!(clean_polygons_issue_78, "issue-78");
    clean_poly_test!(clean_polygons_issue_79, "issue-79");
    clean_poly_test!(clean_polygons_issue_83, "issue-83");
    clean_poly_test!(clean_polygons_issue_85, "issue-85");
    clean_poly_test!(clean_polygons_issue_86, "issue-86");
    clean_poly_test!(clean_polygons_issue_90, "issue-90");
    clean_poly_test!(clean_polygons_issue_91, "issue-91");
    clean_poly_test!(clean_polygons_issue_93, "issue-93");
    clean_poly_test!(clean_polygons_issue_94, "issue-94");
    clean_poly_test!(clean_polygons_issue_105, "issue-105");
    clean_poly_test!(clean_polygons_issue_115, "issue-115");
    clean_poly_test!(clean_polygons_issue_118, "issue-118");
    clean_poly_test!(clean_polygons_issue_124, "issue-124");
    clean_poly_test!(clean_polygons_issue_139, "issue-139");
    clean_poly_test!(clean_polygons_issue_140, "issue-140");
    clean_poly_test!(clean_polygons_issue_141, "issue-141");
    clean_poly_test!(clean_polygons_issue_142, "issue-142");
    clean_poly_test!(clean_polygons_issue_turf_1094, "issue-turf-1094");
    clean_poly_test!(clean_polygons_maybe_colinear_sides, "maybe-colinear-sides");
    clean_poly_test!(clean_polygons_multipoly_and_square, "multipoly-and-square");
    clean_poly_test!(
        clean_polygons_multipoly_with_hole_and_square,
        "multipoly-with-hole-and-square"
    );
    clean_poly_test!(
        clean_polygons_multipolys_with_disjoint_polys,
        "multipolys-with-disjoint-polys"
    );
    clean_poly_test!(clean_polygons_nearly_vertical_far_right, "nearly-vertical-far-right");
    clean_poly_test!(clean_polygons_no_bbox_overlap, "no-bbox-overlap");
    clean_poly_test!(
        clean_polygons_no_self_intersecting_rings_output,
        "no-self-intersecting-rings-output"
    );
    clean_poly_test!(clean_polygons_non_zero_rule_not_even_odd, "non-zero-rule-not-even-odd");
    clean_poly_test!(clean_polygons_overlap_edges, "overlap-edges");
    clean_poly_test!(clean_polygons_overlap_loop, "overlap-loop");
    clean_poly_test!(clean_polygons_overlapping_clippings, "overlapping-clippings");
    clean_poly_test!(clean_polygons_poly_and_square, "poly-and-square");
    clean_poly_test!(clean_polygons_poly_with_hole_and_square, "poly-with-hole-and-square");
    clean_poly_test!(clean_polygons_polygon_and_trapezoid, "polygon-and-trapezoid");
    clean_poly_test!(
        clean_polygons_right_sweep_events_change_ordering,
        "right-sweep-events-change-ordering"
    );
    clean_poly_test!(clean_polygons_rings_with_no_area, "rings-with-no-area");
    clean_poly_test!(clean_polygons_saw_and_cheese, "saw-and-cheese");
    clean_poly_test!(
        clean_polygons_self_intersects_but_doesnt_cross_1,
        "self-intersects-but-doesnt-cross-1"
    );
    clean_poly_test!(
        clean_polygons_self_intersects_but_doesnt_cross_2,
        "self-intersects-but-doesnt-cross-2"
    );
    clean_poly_test!(clean_polygons_simple_kink, "simple-kink");
    clean_poly_test!(clean_polygons_simple_kink_2, "simple-kink-2");
    clean_poly_test!(clean_polygons_split_almost_vertical_segment, "split-almost-vertical-segment");
    clean_poly_test!(clean_polygons_split_prev_segment, "split-prev-segment");
    clean_poly_test!(clean_polygons_three_triangles, "three-triangles");
    clean_poly_test!(clean_polygons_touching_boxes, "touching-boxes");
    clean_poly_test!(clean_polygons_triple_coincident_segments, "triple-coincident-segments");
    clean_poly_test!(clean_polygons_two_disjoint_polygons, "two-disjoint-polygons");
    clean_poly_test!(clean_polygons_two_overlapping_triangles, "two-overlapping-triangles");
    clean_poly_test!(
        clean_polygons_two_overlapping_triangles_start_inside,
        "two-overlapping-triangles-start-inside"
    );
    clean_poly_test!(
        clean_polygons_union_same_shape_multiple_times,
        "union-same-shape-multiple-times"
    );
    clean_poly_test!(
        clean_polygons_vertical_intersection_rounding_error,
        "vertical-intersection-rounding-error"
    );
    clean_poly_test!(clean_polygons_vertical_segment_upon_split, "vertical-segment-upon-split");
    clean_poly_test!(clean_polygons_windmill_3_polys, "windmill-3-polys");
    clean_poly_test!(clean_polygons_windmill_3_polys_2, "windmill-3-polys-2");
    clean_poly_test!(clean_polygons_windmill_3_polys_3, "windmill-3-polys-3");
    clean_poly_test!(clean_polygons_windmill_4_blades, "windmill-4-blades");
}
