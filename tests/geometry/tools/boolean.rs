#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{clean_polygons, convert, polygons_union};
    use s2json::{BBox, JSONCollection, Projection, VectorGeometry, VectorMultiPolygon};
    use std::fs;
    use std::{mem::take, path::PathBuf};

    /// Reads a geojson file into a VectorMultiPolygon
    /// NOTE: Actually writing/testing failed test case has all been done in the Typescript equivalent
    fn get_data(folder: String, name: Option<String>, clean: bool) -> VectorMultiPolygon {
        let name = name.unwrap_or("cleaned".to_string());
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

        if clean {
            if let Some((cleaned, _)) = clean_polygons(&res, true, false, false) {
                return cleaned;
            } else {
                return vec![];
            }
        }

        res
    }

    fn assert_nested_approx(a: &VectorMultiPolygon, b: &VectorMultiPolygon, eps: f64) {
        assert_eq!(a.len(), b.len(), "\nleft: {:?}\n\nright: {:?}", a, b);
        for (pa, pb) in a.iter().zip(b) {
            assert_eq!(pa.len(), pb.len(), "\nleft: {:?}\n\nright: {:?}", a, b);
            for (ra, rb) in pa.iter().zip(pb) {
                assert_eq!(ra.len(), rb.len(), "\nleft: {:?}\n\nright: {:?}", a, b);
                for (p1, p2) in ra.iter().zip(rb) {
                    assert!(
                        (p1.x - p2.x).abs() <= eps,
                        "x diff: {} vs {}\n\nleft: {:?}\n\nright: {:?}",
                        p1.x,
                        p2.x,
                        a,
                        b
                    );
                    assert!(
                        (p1.y - p2.y).abs() <= eps,
                        "y diff: {} vs {}\n\nleft: {:?}\n\nright: {:?}",
                        p1.y,
                        p2.y,
                        a,
                        b
                    );
                }
            }
        }
    }

    macro_rules! polygons_union_test {
        ($name:ident, $folder:expr) => {
            #[test]
            fn $name() {
                let input = get_data($folder.to_string(), None, true);
                let output = get_data($folder.to_string(), Some("union".into()), false);
                let (unioned, _) = polygons_union(&input).unwrap_or((vec![], BBox::default()));
                assert_nested_approx(&unioned, &output, 1e-7);
            }
        };
    }

    polygons_union_test!(
        polygons_union_almost_colinear_segments_but_not,
        "almost-colinear-segments-but-not"
    );
    polygons_union_test!(
        polygons_union_almost_colinear_segments_but_not_2,
        "almost-colinear-segments-but-not-2"
    );
    polygons_union_test!(polygons_union_almost_parrallel_segments, "almost-parrallel-segments");
    polygons_union_test!(polygons_union_almost_parrallel_segments_2, "almost-parrallel-segments-2");
    polygons_union_test!(polygons_union_almost_parrallel_segments_3, "almost-parrallel-segments-3");
    polygons_union_test!(
        polygons_union_clean_multipoly_with_polys_overlapping,
        "clean-multipoly-with-polys-overlapping"
    );
    polygons_union_test!(
        polygons_union_clean_multipoly_with_polys_touching,
        "clean-multipoly-with-polys-touching"
    );
    polygons_union_test!(
        polygons_union_clean_poly_with_backward_ring_winding_order,
        "clean-poly-with-backward-ring-winding-order"
    );
    polygons_union_test!(
        polygons_union_clean_poly_with_repeated_and_extra_points,
        "clean-poly-with-repeated-and-extra-points"
    );
    polygons_union_test!(polygons_union_collapsed_edges_removed, "collapsed-edges-removed");
    polygons_union_test!(polygons_union_disjoint_union, "disjoint-union");
    polygons_union_test!(polygons_union_dont_consume_prev_segment_1, "dont-consume-prev-segment-1");
    polygons_union_test!(polygons_union_dont_consume_prev_segment_2, "dont-consume-prev-segment-2");
    polygons_union_test!(polygons_union_dont_consume_prev_segment_3, "dont-consume-prev-segment-3");
    polygons_union_test!(polygons_union_double_overlap, "double-overlap");
    polygons_union_test!(polygons_union_empty_multipoly, "empty-multipoly");
    polygons_union_test!(polygons_union_high_coincidence, "high-coincidence");
    polygons_union_test!(polygons_union_hole_from_outers_bug, "hole-from-outers-bug");
    polygons_union_test!(polygons_union_hole_interacts_outer, "hole-interacts-outer");
    // polygons_union_test!(polygons_union_infinitely_thin_polygon, "infinitely-thin-polygon");
    polygons_union_test!(polygons_union_intersection_after_remove_1, "intersection-after-remove-1");
    polygons_union_test!(polygons_union_intersection_after_remove_2, "intersection-after-remove-2");
    polygons_union_test!(polygons_union_island_in_hole_4x, "island-in-hole-4x");
    polygons_union_test!(polygons_union_issue_1, "issue-1");
    polygons_union_test!(polygons_union_issue_36, "issue-36");
    polygons_union_test!(polygons_union_issue_37, "issue-37");
    polygons_union_test!(polygons_union_issue_38, "issue-38");
    polygons_union_test!(polygons_union_issue_44, "issue-44");
    polygons_union_test!(polygons_union_issue_60, "issue-60");
    polygons_union_test!(polygons_union_issue_60_2, "issue-60-2");
    polygons_union_test!(polygons_union_issue_60_3, "issue-60-3");
    polygons_union_test!(polygons_union_issue_60_4, "issue-60-4");
    polygons_union_test!(polygons_union_issue_60_5, "issue-60-5");
    // polygons_union_test!(polygons_union_issue_60_6, "issue-60-6");
    polygons_union_test!(polygons_union_issue_60_7, "issue-60-7");
    polygons_union_test!(polygons_union_issue_60_8, "issue-60-8");
    // polygons_union_test!(polygons_union_issue_61, "issue-61");
    polygons_union_test!(polygons_union_issue_61_2, "issue-61-2");
    polygons_union_test!(polygons_union_issue_62, "issue-62");
    polygons_union_test!(polygons_union_issue_62_2, "issue-62-2");
    // polygons_union_test!(polygons_union_issue_66, "issue-66");
    polygons_union_test!(polygons_union_issue_68, "issue-68");
    polygons_union_test!(polygons_union_issue_68_1, "issue-68-1");
    polygons_union_test!(polygons_union_issue_75, "issue-75");
    polygons_union_test!(polygons_union_issue_78, "issue-78");
    polygons_union_test!(polygons_union_issue_79, "issue-79");
    polygons_union_test!(polygons_union_issue_83, "issue-83");
    polygons_union_test!(polygons_union_issue_85, "issue-85");
    polygons_union_test!(polygons_union_issue_86, "issue-86");
    polygons_union_test!(polygons_union_issue_90, "issue-90");
    polygons_union_test!(polygons_union_issue_91, "issue-91");
    polygons_union_test!(polygons_union_issue_93, "issue-93");
    // polygons_union_test!(polygons_union_issue_94, "issue-94");
    polygons_union_test!(polygons_union_issue_105, "issue-105");
    polygons_union_test!(polygons_union_issue_115, "issue-115");
    polygons_union_test!(polygons_union_issue_118, "issue-118");
    polygons_union_test!(polygons_union_issue_118_2, "issue-118-2");
    polygons_union_test!(polygons_union_issue_124, "issue-124");
    polygons_union_test!(polygons_union_issue_139, "issue-139");
    polygons_union_test!(polygons_union_issue_140, "issue-140");
    polygons_union_test!(polygons_union_issue_141, "issue-141");
    polygons_union_test!(polygons_union_issue_142, "issue-142");
    polygons_union_test!(polygons_union_issue_142_outeres, "issue-142-outers");
    polygons_union_test!(polygons_union_issue_142_simple, "issue-142-simple");
    polygons_union_test!(polygons_union_issue_turf_1094, "issue-turf-1094");
    polygons_union_test!(polygons_union_maybe_colinear_sides, "maybe-colinear-sides");
    polygons_union_test!(polygons_union_multipoly_and_square, "multipoly-and-square");
    polygons_union_test!(
        polygons_union_multipoly_with_hole_and_square,
        "multipoly-with-hole-and-square"
    );
    polygons_union_test!(
        polygons_union_multipolys_with_disjoint_polys,
        "multipolys-with-disjoint-polys"
    );
    polygons_union_test!(polygons_union_nearly_vertical_far_right, "nearly-vertical-far-right");
    polygons_union_test!(polygons_union_no_bbox_overlap, "no-bbox-overlap");
    // polygons_union_test!(
    //     polygons_union_no_self_intersecting_rings_output,
    //     "no-self-intersecting-rings-output"
    // );
    polygons_union_test!(polygons_union_non_zero_rule_not_even_odd, "non-zero-rule-not-even-odd");
    polygons_union_test!(polygons_union_overlap_edges, "overlap-edges");
    // polygons_union_test!(polygons_union_overlap_loop, "overlap-loop");
    // polygons_union_test!(polygons_union_overlapping_clippings, "overlapping-clippings");
    polygons_union_test!(polygons_union_poly_and_square, "poly-and-square");
    polygons_union_test!(polygons_union_poly_with_hole_and_square, "poly-with-hole-and-square");
    polygons_union_test!(polygons_union_polygon_and_trapezoid, "polygon-and-trapezoid");
    polygons_union_test!(
        polygons_union_right_sweep_events_change_ordering,
        "right-sweep-events-change-ordering"
    );
    polygons_union_test!(polygons_union_rings_with_no_area, "rings-with-no-area");
    polygons_union_test!(polygons_union_saw_and_cheese, "saw-and-cheese");
    polygons_union_test!(
        polygons_union_self_intersects_but_doesnt_cross_1,
        "self-intersects-but-doesnt-cross-1"
    );
    polygons_union_test!(
        polygons_union_self_intersects_but_doesnt_cross_2,
        "self-intersects-but-doesnt-cross-2"
    );
    polygons_union_test!(polygons_union_simple_kink, "simple-kink");
    polygons_union_test!(polygons_union_simple_kink_2, "simple-kink-2");
    polygons_union_test!(
        polygons_union_split_almost_vertical_segment,
        "split-almost-vertical-segment"
    );
    polygons_union_test!(polygons_union_split_prev_segment, "split-prev-segment");
    polygons_union_test!(polygons_union_three_triangles, "three-triangles");
    polygons_union_test!(polygons_union_touching_boxes, "touching-boxes");
    polygons_union_test!(polygons_union_triple_coincident_segments, "triple-coincident-segments");
    polygons_union_test!(polygons_union_two_disjoint_polygons, "two-disjoint-polygons");
    polygons_union_test!(polygons_union_two_overlapping_triangles, "two-overlapping-triangles");
    polygons_union_test!(
        polygons_union_two_overlapping_triangles_start_inside,
        "two-overlapping-triangles-start-inside"
    );
    polygons_union_test!(
        polygons_union_union_same_shape_multiple_times,
        "union-same-shape-multiple-times"
    );
    polygons_union_test!(
        polygons_union_vertical_intersection_rounding_error,
        "vertical-intersection-rounding-error"
    );
    polygons_union_test!(polygons_union_vertical_segment_upon_split, "vertical-segment-upon-split");
    polygons_union_test!(polygons_union_windmill_3_polys, "windmill-3-polys");
    polygons_union_test!(polygons_union_windmill_3_polys_2, "windmill-3-polys-2");
    polygons_union_test!(polygons_union_windmill_3_polys_3, "windmill-3-polys-3");
    polygons_union_test!(polygons_union_windmill_4_blades, "windmill-4-blades");

    polygons_union_test!(polygons_union_chunks_water_3, "chunks-water-3");
}
