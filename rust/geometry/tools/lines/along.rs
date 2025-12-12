use crate::geometry::{bearing, destination};
use libm::hypot;
use s2json::{
    Feature, Features, Geometry, GetXY, LineString, LineStringGeometry, NewXY, VectorFeature,
    VectorGeometry, VectorLineString, VectorLineStringGeometry,
};

// TODO: If GetZ returns a value, treat each point as a 3D point. Then we can support 3DGeometry

/// Given a linestring in degrees and a distance, create a Point along the line
///
/// If no radius is provided, defaults to the Earth's radius
///
/// NOTE: If your feature/geometry isn't a line, the point returned will have an x and y of [`f64::NAN`]
///
/// This trait is implemented for:
/// - [`Feature`]
/// - [`Geometry`]
/// - [`LineStringGeometry`]
/// - [`LineString`]
/// - [`VectorFeature`]
/// - [`VectorGeometry`]
/// - [`VectorLineStringGeometry`]
/// - [`VectorLineString`]
/// - [`Features`]
/// - `&[P]` where P implements [`GetXY`] and [`GetZ`]
///
/// And all specific geometries of the above enums
///
/// If you want to work with the function directly use [`along_line`]
pub trait Along<P: NewXY> {
    /// Get the total euclidean distance of a line or lines
    fn along_line(&self, distance: f64, radius: Option<f64>) -> P;
}

// Relative Trait

impl<P: GetXY + NewXY, Q: GetXY> Along<P> for &[Q] {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> P {
        along_line(self, distance, radius)
    }
}

// Features

impl<M, P: Clone + Default, D: Clone + Default, Q: NewXY> Along<Q> for Feature<M, P, D> {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        self.geometry.along_line(distance, radius)
    }
}
impl<M: Clone + Default, Q: NewXY> Along<Q> for Geometry<M> {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        match self {
            Geometry::LineString(g) => g.along_line(distance, radius),
            _ => Q::new_xy(f64::NAN, f64::NAN),
        }
    }
}
impl<M: Clone + Default, Q: NewXY> Along<Q> for LineStringGeometry<M> {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        self.coordinates.along_line(distance, radius)
    }
}
impl<Q: NewXY> Along<Q> for LineString {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        along_line(self, distance, radius)
    }
}

// Vector Features

impl<M, P: Clone + Default, D: Clone + Default, Q: NewXY> Along<Q> for VectorFeature<M, P, D> {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        self.geometry.along_line(distance, radius)
    }
}
impl<M: Clone + Default, Q: NewXY> Along<Q> for VectorGeometry<M> {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        match self {
            VectorGeometry::LineString(g) => g.along_line(distance, radius),
            _ => Q::new_xy(f64::NAN, f64::NAN),
        }
    }
}
impl<M: Clone + Default, Q: NewXY> Along<Q> for VectorLineStringGeometry<M> {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        self.coordinates.along_line(distance, radius)
    }
}
impl<M: Clone + Default, Q: NewXY> Along<Q> for VectorLineString<M> {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        along_line(self, distance, radius)
    }
}

// Features

impl<M, P: Clone + Default, D: Clone + Default, Q: NewXY> Along<Q> for Features<M, P, D> {
    fn along_line(&self, distance: f64, radius: Option<f64>) -> Q {
        match self {
            Features::Feature(f) => f.along_line(distance, radius),
            Features::VectorFeature(f) => f.along_line(distance, radius),
        }
    }
}

/// Given a linestring in degrees and a distance, create a Point along the line
///
/// If no radius is provided, defaults to the Earth's radius
pub fn along_line<P: GetXY, Q: NewXY>(coords: &[P], distance: f64, radius: Option<f64>) -> Q {
    let mut travelled = 0.;
    for i in 0..coords.len() {
        if distance >= travelled && i == coords.len() - 1 {
            break;
        } else if travelled >= distance {
            let overshot = distance - travelled;
            if overshot == 0. {
                return Q::new_xy(coords[i].x(), coords[i].y());
            } else {
                let direction = bearing(&coords[i], &coords[i - 1]) - 180.;
                let interpolated = destination(&coords[i], overshot, direction, radius);
                return interpolated;
            }
        } else {
            travelled +=
                hypot(coords[i + 1].x() - coords[i].x(), coords[i + 1].y() - coords[i].y());
        }
    }
    let last = coords.len() - 1;
    Q::new_xy(coords[last].x(), coords[last].y())
}
