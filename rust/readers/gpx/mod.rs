/// 1.1 Specification for the GPX format
pub mod spec;

use crate::parsers::FeatureReader;
use alloc::{string::String, vec::Vec};
use s2json::{MValueCompatible, VectorFeature};
pub use spec::*;

/// Represents a route, which is an ordered list of waypoints leading to a destination.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GPXProperties {
    /// Route name
    pub name: Option<String>,
    /// Route comment
    pub cmt: Option<String>,
    /// Route description
    pub desc: Option<String>,
    /// Source of data
    pub src: Option<String>,
    /// Links to external information
    pub link: Option<Vec<GPXLink>>,
    /// Route number
    pub number: Option<usize>,
    /// Classification type of the route
    pub route_type: Option<String>,
    /// Classification type of the track
    pub track_type: Option<String>,
}

/// A GPX Shaped Vector Feature
pub type GPXVectorFeature = VectorFeature<(), GPXProperties, GPXWaypoint>;

/// # GPX Reader
///
/// ## Description
/// The GPX Reader is an XML-based GPS Exchange Format (GPX) reader.
///
/// GPX (the GPS Exchange Format) is a light-weight XML data format for the interchange of GPS data
/// (waypoints, routes, and tracks) between applications and Web services on the Internet.
///
/// ## Links
/// https://www.topografix.com/gpx.asp
#[derive(Debug)]
pub struct GPXReader {
    /// GPX object
    pub gpx: GPX,
}
impl GPXReader {
    /// Create a new GPX Reader
    pub fn new(input: &str) -> Self {
        Self { gpx: GPX::new(input) }
    }
    /// Grab the metadata
    pub fn metadata(&self) -> GPXMetadata {
        self.gpx.metadata.clone().unwrap_or_default()
    }
}
/// The GPX Iterator tool
#[derive(Debug)]
pub struct GPXIterator<'a> {
    reader: &'a GPXReader,
    wpt_offset: usize,
    wpt_count: usize,
    rte_offset: usize,
    rte_count: usize,
    trk_offset: usize,
    trk_count: usize,
}
impl Iterator for GPXIterator<'_> {
    type Item = GPXVectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        let gpx = &self.reader.gpx;
        if self.wpt_offset < self.wpt_count {
            self.wpt_offset += 1;
            return gpx.wpt.as_ref().map(|w| w[self.wpt_offset - 1].feature());
        }
        if self.rte_offset < self.rte_count {
            self.rte_offset += 1;
            return gpx.rte.as_ref().map(|w| w[self.rte_offset - 1].feature());
        }
        if self.trk_offset < self.trk_count {
            self.trk_offset += 1;
            return gpx.trk.as_ref().map(|w| w[self.trk_offset - 1].feature());
        }
        None
    }
}
/// A feature reader trait with a callback-based approach
impl FeatureReader<(), GPXProperties, GPXWaypoint> for GPXReader {
    type FeatureIterator<'a> = GPXIterator<'a>;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        GPXIterator {
            reader: self,
            wpt_offset: 0,
            wpt_count: self.gpx.wpt.as_ref().map(|w| w.len()).unwrap_or_default(),
            rte_offset: 0,
            rte_count: self.gpx.rte.as_ref().map(|r| r.len()).unwrap_or_default(),
            trk_offset: 0,
            trk_count: self.gpx.trk.as_ref().map(|t| t.len()).unwrap_or_default(),
        }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}
