use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use s2json::{
    BBox3D, MValue, MValueCompatible, VectorFeature, VectorGeometry, VectorLineString, VectorPoint,
};

/// # Shape Properties
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSShapeProperties {
    /// The ID of the shape
    pub shape_id: String,
}
impl From<String> for GTFSShapeProperties {
    fn from(shape_id: String) -> Self {
        GTFSShapeProperties { shape_id }
    }
}

/// # Shape MValue
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSShapeMValue {
    /// May be missing
    pub shape_dist_traveled: f64,
}
impl From<f64> for GTFSShapeMValue {
    fn from(shape_dist_traveled: f64) -> Self {
        GTFSShapeMValue { shape_dist_traveled }
    }
}

/// # Shape Feature
impl From<GTFSShapes> for VectorFeature {
    fn from(shapes: GTFSShapes) -> Self {
        let bbox = BBox3D::from_linestring(&shapes.shapes);
        let props: GTFSShapeProperties = shapes.shape_id.into();
        VectorFeature {
            properties: props.into(),
            geometry: VectorGeometry::new_linestring(shapes.shapes, Some(bbox)),
            ..Default::default()
        }
    }
}

/// # Shapes
///
/// A collection of shapes all with the same shape_id and ordered by shape_pt_sequence
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSShapes {
    /// The ID of the shape
    pub shape_id: String,
    /// The collection of shapes
    pub shapes: VectorLineString,
}
impl From<&Vec<GTFSShape>> for GTFSShapes {
    fn from(shapes: &Vec<GTFSShape>) -> Self {
        let mut res = GTFSShapes::default();
        if shapes.is_empty() {
            return res;
        }
        // set id
        let id = shapes[0].shape_id.clone();
        res.shape_id = id;
        // store shapes
        for shape in shapes {
            let mvalue: Option<GTFSShapeMValue> = shape.shape_dist_traveled.map(Into::into);
            let mvalue: Option<MValue> = mvalue.map(Into::into);
            res.shapes.push(VectorPoint::new_xy(shape.shape_pt_lon, shape.shape_pt_lat, mvalue));
        }
        res
    }
}

/// # Shapes
///
/// ## Details
/// **Optional** - Primary key (shape_id, shape_pt_sequence)
///
/// Shapes describe the path that a vehicle travels along a route alignment, and are defined in the
/// file shapes.txt. Shapes are associated with Trips, and consist of a sequence of points through
/// which the vehicle passes in order. Shapes do not need to intercept the location of Stops
/// exactly, but all Stops on a trip should lie within a small distance of the shape for that trip,
/// i.e. close to straight line segments connecting the shape points. The shapes.txt file should be
/// included for all route-based services (not for zone-based demand-responsive services).
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSShape {
    /// **Required**
    /// Identifies a shape. E.g. "A_shp"
    pub shape_id: String,
    /// **Required**
    /// Latitude of a shape point. Each record in shapes.txt represents a shape point used to define
    /// the shape.
    pub shape_pt_lat: f64,
    /// **Required**
    /// Longitude of a shape point.
    pub shape_pt_lon: f64,
    /// **Required**
    /// Sequence in which the shape points connect to form the shape. Values must increase along the
    /// trip but do not need to be consecutive.
    ///
    /// Example: If the shape "A_shp" has three points in its definition, the shapes.txt file might contain these records to define the shape:
    /// shape_id,shape_pt_lat,shape_pt_lon,shape_pt_sequence
    /// A_shp,37.61956,-122.48161,0
    /// A_shp,37.64430,-122.41070,6
    /// A_shp,37.65863,-122.30839,11
    pub shape_pt_sequence: u64,
    /// **Optional**
    /// Actual distance traveled along the shape from the first shape point to the point specified
    /// in this record. Used by trip planners to show the correct portion of the shape on a map.
    /// Values must increase along with shape_pt_sequence; they must not be used to show reverse
    /// travel along a route. Distance units must be consistent with those used in stop_times.txt.
    ///
    /// Recommended for routes that have looping or inlining (the vehicle crosses or travels over
    /// the same portion of alignment in one trip).
    ///
    /// If a vehicle retraces or crosses the route alignment at points in the course of a trip,
    /// shape_dist_traveled is important to clarify how portions of the points in shapes.txt line
    /// up correspond with records in stop_times.txt.
    ///
    /// Example: If a bus travels along the three points defined above for A_shp, the additional shape_dist_traveled values (shown here in kilometers) would look like this:
    /// shape_id,shape_pt_lat,shape_pt_lon,shape_pt_sequence,shape_dist_traveled
    /// A_shp,37.61956,-122.48161,0,0
    /// A_shp,37.64430,-122.41070,6,6.8310
    /// A_shp,37.65863,-122.30839,11,15.8765
    pub shape_dist_traveled: Option<f64>,
}
impl GTFSShape {
    /// Create a new GTFSShape
    pub fn new(source: &str) -> BTreeMap<String, Vec<GTFSShape>> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSShape>(source, None, None) {
            let v = res.entry(record.shape_id.clone()).or_insert(Vec::new());
            v.push(record);
        }
        // iterate through each and sort by shape_pt_sequence
        for shapes in res.values_mut() {
            shapes.sort_by(|a, b| a.shape_pt_sequence.cmp(&b.shape_pt_sequence));
        }
        res
    }
}
