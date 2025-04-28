use super::GPXProperties;
use crate::{
    data_structures::HasLayer,
    readers::{XMLTagItem, xml_find_tag_by_name, xml_find_tags_by_name, xml_get_attribute},
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt, str::FromStr};
use s2json::{
    MValueCompatible, PrimitiveValue, ValuePrimitive, ValueType, VectorFeature, VectorFeatureType,
    VectorGeometry, VectorLineString, VectorMultiLineString, VectorPoint,
};
use serde::{Deserialize, Serialize};

/// Represents the root GPX document.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPX {
    /// Fixed GPX version
    pub version: String, // "1.1",
    /// Name or URL of the software that created the GPX document
    pub creator: String,
    /// Optional metadata about the file
    pub metadata: Option<GPXMetadata>,
    /// Array of waypoints
    pub wpt: Option<Vec<GPXWaypoint>>,
    /// Array of routes
    pub rte: Option<Vec<GPXRoute>>,
    /// Array of tracks
    pub trk: Option<Vec<GPXTrack>>,
    // /// Custom extensions for additional data
    // pub extensions: Option<MValue>,
}
impl GPX {
    /// Creates a new GPX from an XML string
    pub fn new(gpx_xml: &str) -> GPX {
        let root_tag = xml_find_tag_by_name(gpx_xml, "gpx", None);

        if let Some(root) = root_tag {
            let version =
                xml_get_attribute(&XMLTagItem::XMLTag(root.clone()), "version").unwrap_or_default();
            let creator =
                xml_get_attribute(&XMLTagItem::XMLTag(root.clone()), "creator").unwrap_or_default();

            let metadata_tag = xml_find_tag_by_name(&root.outer, "metadata", None);
            let metadata = metadata_tag.map(|tag| GPXMetadata::new(XMLTagItem::XMLTag(tag)));

            let wpt = {
                let wpt_tags = xml_find_tags_by_name(&root.outer, "wpt", None);
                if !wpt_tags.is_empty() {
                    Some(
                        wpt_tags
                            .into_iter()
                            .map(|wpt| GPXWaypoint::new(XMLTagItem::XMLTag(wpt)))
                            .collect(),
                    )
                } else {
                    None
                }
            };

            let rte = {
                let rte_tags = xml_find_tags_by_name(&root.outer, "rte", None);
                if !rte_tags.is_empty() {
                    Some(
                        rte_tags
                            .into_iter()
                            .map(|rte| GPXRoute::new(XMLTagItem::XMLTag(rte)))
                            .collect(),
                    )
                } else {
                    None
                }
            };

            let trk = {
                let trk_tags = xml_find_tags_by_name(&root.outer, "trk", None);
                if !trk_tags.is_empty() {
                    Some(
                        trk_tags
                            .into_iter()
                            .map(|trk| GPXTrack::new(XMLTagItem::XMLTag(trk)))
                            .collect(),
                    )
                } else {
                    None
                }
            };

            GPX { version, creator, metadata, wpt, rte, trk }
        } else {
            GPX::default()
        }
    }
}

/// Contains metadata information about the GPX file.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPXMetadata {
    /// Name of the GPX file
    pub name: Option<String>,
    /// Description of the file's contents
    pub desc: Option<String>,
    /// Person or organization responsible for the file
    pub author: Option<GPXPerson>,
    /// Copyright and license information
    pub copyright: Option<GPXCopyright>,
    /// URLs associated with the GPX file
    pub link: Option<GPXLink>,
    /// Creation timestamp in ISO 8601 format
    pub time: Option<String>,
    /// Keywords for classification
    pub keywords: Option<String>,
    /// Bounding box of the data
    pub bounds: Option<GPXBounds>,
    // /// Custom extensions
    // pub extensions: Option<MValue>,
}
impl GPXMetadata {
    /// Creates a new GPXMetadata from an XMLTagItem
    pub fn new(metadata_xml: XMLTagItem) -> Self {
        let inner = match &metadata_xml {
            XMLTagItem::XMLTag(tag) => tag.inner.clone().unwrap_or_default(),
            XMLTagItem::String(s) => s.clone(),
        };

        let name = xml_find_tag_by_name(&inner, "name", None).and_then(|tag| tag.inner);
        let desc = xml_find_tag_by_name(&inner, "desc", None).and_then(|tag| tag.inner);
        let author = xml_find_tag_by_name(&inner, "author", None)
            .map(|tag| GPXPerson::new(XMLTagItem::XMLTag(tag)));
        let copyright = xml_find_tag_by_name(&inner, "copyright", None)
            .map(|tag| GPXCopyright::new(XMLTagItem::XMLTag(tag)));
        let link = xml_find_tag_by_name(&inner, "link", None)
            .map(|tag| GPXLink::new(XMLTagItem::XMLTag(tag)));
        let time = xml_find_tag_by_name(&inner, "time", None).and_then(|tag| tag.inner);
        let keywords = xml_find_tag_by_name(&inner, "keywords", None).and_then(|tag| tag.inner);
        let bounds = xml_find_tag_by_name(&inner, "bounds", None)
            .map(|tag| GPXBounds::new(XMLTagItem::XMLTag(tag)));

        GPXMetadata { name, desc, author, copyright, link, time, keywords, bounds }
    }
}
impl HasLayer for GPXMetadata {
    fn get_layer(&self) -> Option<String> {
        None
    }
}

/// Represents a route, which is an ordered list of waypoints leading to a destination.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPXRoute {
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
    pub r#type: Option<String>,
    /// Ordered list of route waypoints
    pub rtept: Option<Vec<GPXWaypoint>>,
    // /// Custom extensions
    // pub extensions: Option<MValue>,
}
impl GPXRoute {
    /// Creates a new GPXRoute from an XMLTagItem
    pub fn new(route_xml: XMLTagItem) -> Self {
        let inner = match &route_xml {
            XMLTagItem::XMLTag(tag) => tag.inner.clone().unwrap_or_default(),
            XMLTagItem::String(s) => s.clone(),
        };

        let name = xml_find_tag_by_name(&inner, "name", None).and_then(|tag| tag.inner);
        let cmt = xml_find_tag_by_name(&inner, "cmt", None).and_then(|tag| tag.inner);
        let desc = xml_find_tag_by_name(&inner, "desc", None).and_then(|tag| tag.inner);
        let src = xml_find_tag_by_name(&inner, "src", None).and_then(|tag| tag.inner);
        let link = {
            let link_tags = xml_find_tags_by_name(&inner, "link", None);
            if !link_tags.is_empty() {
                Some(
                    link_tags
                        .into_iter()
                        .map(|tag| GPXLink::new(XMLTagItem::XMLTag(tag)))
                        .collect(),
                )
            } else {
                None
            }
        };
        let number = xml_find_tag_by_name(&inner, "number", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<usize>().ok()));
        let r#type = xml_find_tag_by_name(&inner, "type", None).and_then(|tag| tag.inner);
        let rtept = {
            let rtept_tags = xml_find_tags_by_name(&inner, "rtept", None);
            if !rtept_tags.is_empty() {
                Some(
                    rtept_tags
                        .into_iter()
                        .map(|tag| GPXWaypoint::new(XMLTagItem::XMLTag(tag)))
                        .collect(),
                )
            } else {
                None
            }
        };

        GPXRoute { name, cmt, desc, src, link, number, r#type, rtept }
    }

    /// Create a linestring of waypoints
    pub fn line(&self) -> VectorLineString<GPXWaypoint> {
        self.rtept.as_ref().map(|r| r.iter().map(|w| w.point()).collect()).unwrap_or_default()
    }

    /// Create a Vector Feature from the Route
    pub fn feature(&self) -> VectorFeature<(), GPXProperties, GPXWaypoint> {
        VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            properties: self.into(),
            geometry: VectorGeometry::new_linestring(self.line(), None),
            ..Default::default()
        }
    }
}
impl From<&GPXRoute> for GPXProperties {
    fn from(route: &GPXRoute) -> Self {
        GPXProperties {
            name: route.name.clone(),
            cmt: route.cmt.clone(),
            desc: route.desc.clone(),
            src: route.src.clone(),
            link: route.link.clone(),
            number: route.number,
            track_type: None,
            route_type: route.r#type.clone(),
        }
    }
}

/// Represents a track, which is an ordered list of points describing a path.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPXTrack {
    /// Track name
    pub name: Option<String>,
    /// Track comment
    pub cmt: Option<String>,
    /// Track description
    pub desc: Option<String>,
    /// Source of data
    pub src: Option<String>,
    /// Links to external information
    pub link: Option<Vec<GPXLink>>,
    /// Track number
    pub number: Option<usize>,
    /// Classification type of the track
    pub r#type: Option<String>,
    /// Ordered list of track segments
    pub trkseg: Option<Vec<GPXTrackSegment>>,
    // /// Custom extensions
    // pub extensions: Option<MValue>,
}
impl GPXTrack {
    /// Creates a new GPXTrack from an XMLTagItem
    pub fn new(track_xml: XMLTagItem) -> Self {
        let inner = match &track_xml {
            XMLTagItem::XMLTag(tag) => tag.inner.clone().unwrap_or_default(),
            XMLTagItem::String(s) => s.clone(),
        };

        let name = xml_find_tag_by_name(&inner, "name", None).and_then(|tag| tag.inner);
        let cmt = xml_find_tag_by_name(&inner, "cmt", None).and_then(|tag| tag.inner);
        let desc = xml_find_tag_by_name(&inner, "desc", None).and_then(|tag| tag.inner);
        let src = xml_find_tag_by_name(&inner, "src", None).and_then(|tag| tag.inner);
        let link = {
            let link_tags = xml_find_tags_by_name(&inner, "link", None);
            if !link_tags.is_empty() {
                Some(
                    link_tags
                        .into_iter()
                        .map(|tag| GPXLink::new(XMLTagItem::XMLTag(tag)))
                        .collect(),
                )
            } else {
                None
            }
        };
        let number = xml_find_tag_by_name(&inner, "number", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<usize>().ok()));
        let r#type = xml_find_tag_by_name(&inner, "type", None).and_then(|tag| tag.inner);
        let trkseg = {
            let trkseg_tags = xml_find_tags_by_name(&inner, "trkseg", None);
            if !trkseg_tags.is_empty() {
                Some(
                    trkseg_tags
                        .into_iter()
                        .map(|tag| GPXTrackSegment::new(XMLTagItem::XMLTag(tag)))
                        .collect(),
                )
            } else {
                None
            }
        };

        GPXTrack { name, cmt, desc, src, link, number, r#type, trkseg }
    }

    /// create a multi-linestring
    pub fn multiline(&self) -> VectorMultiLineString<GPXWaypoint> {
        self.trkseg
            .as_ref()
            .map(|r| {
                r.iter()
                    .map(|s| {
                        s.trkpt
                            .as_ref()
                            .map(|t| t.iter().map(|w| w.point()).collect())
                            .unwrap_or_default()
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Create a Vector Feature from the Route
    pub fn feature(&self) -> VectorFeature<(), GPXProperties, GPXWaypoint> {
        VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            properties: self.into(),
            geometry: VectorGeometry::new_multilinestring(self.multiline(), None),
            ..Default::default()
        }
    }
}
impl From<&GPXTrack> for GPXProperties {
    fn from(track: &GPXTrack) -> Self {
        GPXProperties {
            name: track.name.clone(),
            cmt: track.cmt.clone(),
            desc: track.desc.clone(),
            src: track.src.clone(),
            link: track.link.clone(),
            number: track.number,
            track_type: track.r#type.clone(),
            route_type: None,
        }
    }
}

/// Represents a track segment, which holds a list of track points logically connected in order.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPXTrackSegment {
    /// Ordered list of track points
    pub trkpt: Option<Vec<GPXWaypoint>>,
    // /// Custom extensions
    // pub extensions: Option<MValue>,
}
impl GPXTrackSegment {
    /// Creates a new GPXTrackSegment from an XMLTagItem
    pub fn new(trkseg_xml: XMLTagItem) -> Self {
        let inner = match &trkseg_xml {
            XMLTagItem::XMLTag(tag) => tag.inner.clone().unwrap_or_default(),
            XMLTagItem::String(s) => s.clone(),
        };

        let trkpt = {
            let trkpt_tags = xml_find_tags_by_name(&inner, "trkpt", None);
            if !trkpt_tags.is_empty() {
                Some(
                    trkpt_tags
                        .into_iter()
                        .map(|tag| GPXWaypoint::new(XMLTagItem::XMLTag(tag)))
                        .collect(),
                )
            } else {
                None
            }
        };

        GPXTrackSegment { trkpt }
    }
}

/// Represents a waypoint, point of interest, or named feature on a map.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, MValueCompatible)]
pub struct GPXWaypoint {
    /// Latitude in decimal degrees (WGS84)
    pub lat: f64,
    /// Longitude in decimal degrees (WGS84)
    pub lon: f64,
    /// Optional elevation in meters
    pub ele: Option<f64>,
    /// Optional timestamp in ISO 8601 format
    pub time: Option<String>,
    /// Optional magnetic variation in degrees
    pub magvar: Option<f64>,
    /// Height of geoid above WGS84 ellipsoid
    pub geoidheight: Option<f64>,
    /// Waypoint name
    pub name: Option<String>,
    /// Waypoint comment
    pub cmt: Option<String>,
    /// Description of the waypoint
    pub desc: Option<String>,
    /// Source of data
    pub src: Option<String>,
    /// Links to additional information
    pub link: Option<Vec<GPXLink>>,
    /// Symbol name for the waypoint
    pub sym: Option<String>,
    /// Classification type of the waypoint
    pub r#type: Option<String>,
    /// Type of GPS fix
    pub fix: Option<GPXFixType>,
    /// Number of satellites used for the fix
    pub sat: Option<usize>,
    /// Horizontal dilution of precision
    pub hdop: Option<f64>,
    /// Vertical dilution of precision
    pub vdop: Option<f64>,
    /// Position dilution of precision
    pub pdop: Option<f64>,
    /// Time since last DGPS update in seconds
    pub ageofdgpsdata: Option<f64>,
    /// ID of DGPS station used
    pub dgpsid: Option<f64>,
    // /// Custom extensions
    // pub extensions: Option<MValue>,
}
impl GPXWaypoint {
    /// Creates a new GPXWaypoint from an XMLTagItem
    pub fn new(waypoint_xml: XMLTagItem) -> Self {
        let lat = xml_get_attribute(&waypoint_xml, "lat")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let lon = xml_get_attribute(&waypoint_xml, "lon")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let fix =
            xml_get_attribute(&waypoint_xml, "fix").and_then(|s| s.parse::<GPXFixType>().ok());

        let inner = match waypoint_xml {
            XMLTagItem::XMLTag(tag) => tag.inner.unwrap_or_default(),
            XMLTagItem::String(s) => s,
        };

        let ele = xml_find_tag_by_name(&inner, "ele", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<f64>().ok()));
        let time = xml_find_tag_by_name(&inner, "time", None).and_then(|tag| tag.inner);
        let magvar = xml_find_tag_by_name(&inner, "magvar", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<f64>().ok()));
        let geoidheight = xml_find_tag_by_name(&inner, "geoidheight", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<f64>().ok()));
        let name = xml_find_tag_by_name(&inner, "name", None).and_then(|tag| tag.inner);
        let cmt = xml_find_tag_by_name(&inner, "cmt", None).and_then(|tag| tag.inner);
        let desc = xml_find_tag_by_name(&inner, "desc", None).and_then(|tag| tag.inner);
        let src = xml_find_tag_by_name(&inner, "src", None).and_then(|tag| tag.inner);
        let link = {
            let link_tags = xml_find_tags_by_name(&inner, "link", None);
            if !link_tags.is_empty() {
                Some(
                    link_tags
                        .into_iter()
                        .map(|tag| GPXLink::new(XMLTagItem::XMLTag(tag)))
                        .collect(),
                )
            } else {
                None
            }
        };
        let sym = xml_find_tag_by_name(&inner, "sym", None).and_then(|tag| tag.inner);
        let r#type = xml_find_tag_by_name(&inner, "type", None).and_then(|tag| tag.inner);
        let sat = xml_find_tag_by_name(&inner, "sat", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<usize>().ok()));
        let hdop = xml_find_tag_by_name(&inner, "hdop", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<f64>().ok()));
        let vdop = xml_find_tag_by_name(&inner, "vdop", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<f64>().ok()));
        let pdop = xml_find_tag_by_name(&inner, "pdop", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<f64>().ok()));
        let ageofdgpsdata = xml_find_tag_by_name(&inner, "ageofdgpsdata", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<f64>().ok()));
        let dgpsid = xml_find_tag_by_name(&inner, "dgpsid", None)
            .and_then(|tag| tag.inner.and_then(|s| s.parse::<f64>().ok()));

        GPXWaypoint {
            lat,
            lon,
            ele,
            time,
            magvar,
            geoidheight,
            name,
            cmt,
            desc,
            src,
            link,
            sym,
            r#type,
            fix,
            sat,
            hdop,
            vdop,
            pdop,
            ageofdgpsdata,
            dgpsid,
        }
    }

    /// Create a vector point from the Waypoint
    pub fn point(&self) -> VectorPoint<GPXWaypoint> {
        VectorPoint { x: self.lon, y: self.lat, z: self.ele, m: Some(self.clone()), t: None }
    }

    /// Create a vector point from the Waypoint
    pub fn feature(&self) -> VectorFeature<(), GPXProperties, GPXWaypoint> {
        VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::new_point(self.point(), None),
            ..Default::default()
        }
    }
}

/// Defines copyright and license information.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPXCopyright {
    /// Copyright holder
    pub author: String,
    /// Year of copyright
    pub year: Option<String>,
    /// License URL
    pub license: Option<String>,
}
impl GPXCopyright {
    /// Creates a new GPXCopyright from an XMLTagItem
    pub fn new(copyright_xml: XMLTagItem) -> Self {
        let inner = match copyright_xml {
            XMLTagItem::XMLTag(tag) => tag.inner.unwrap_or_default(),
            XMLTagItem::String(s) => s,
        };

        let author = xml_find_tag_by_name(&inner, "author", None)
            .and_then(|tag| tag.inner)
            .unwrap_or_default();
        let year = xml_find_tag_by_name(&inner, "year", None).and_then(|tag| tag.inner);
        let license = xml_find_tag_by_name(&inner, "license", None).and_then(|tag| tag.inner);

        GPXCopyright { author, year, license }
    }
}

/// Represents a hyperlink with optional text and MIME type.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ValuePrimitive)]
pub struct GPXLink {
    /// URL of the link
    pub href: String,
    /// Optional hyperlink text
    pub text: Option<String>,
    /// MIME type of the linked content
    pub r#type: Option<String>,
}
impl GPXLink {
    /// Creates a new GPXLink from an XMLTagItem
    pub fn new(link_xml: XMLTagItem) -> Self {
        let href = xml_get_attribute(&link_xml, "href").unwrap_or_default();

        let inner = match link_xml {
            XMLTagItem::XMLTag(tag) => tag.inner.unwrap_or_default(),
            XMLTagItem::String(s) => s,
        };

        let text = xml_find_tag_by_name(&inner, "text", None).and_then(|tag| tag.inner);
        let r#type = xml_find_tag_by_name(&inner, "type", None).and_then(|tag| tag.inner);

        GPXLink { href, text, r#type }
    }
}

/// Defines a person or organization associated with the GPX file.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPXPerson {
    /// Name of the person or organization
    pub name: Option<String>,
    /// Email address (split into ID and domain)
    pub email: Option<GPXEmail>,
    /// Link to external information about the person
    pub link: Option<GPXLink>,
}
impl GPXPerson {
    /// Creates a new GPXPerson from an XMLTagItem
    pub fn new(person_xml: XMLTagItem) -> Self {
        let inner = match person_xml {
            XMLTagItem::XMLTag(tag) => tag.inner.unwrap_or_default(),
            XMLTagItem::String(s) => s,
        };

        let name = xml_find_tag_by_name(&inner, "name", None).and_then(|tag| tag.inner);
        let email = xml_find_tag_by_name(&inner, "email", None)
            .map(|tag| GPXEmail::new(XMLTagItem::XMLTag(tag)));
        let link = xml_find_tag_by_name(&inner, "link", None)
            .map(|tag| GPXLink::new(XMLTagItem::XMLTag(tag)));

        GPXPerson { name, email, link }
    }
}

/// Represents an email address, split into ID and domain parts.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPXEmail {
    /// Local part of the email address
    pub id: String,
    /// Domain part of the email address
    pub domain: String,
}
impl GPXEmail {
    /// Creates a new GPXEmail from an XMLTagItem
    pub fn new(email_xml: XMLTagItem) -> Self {
        let id = xml_get_attribute(&email_xml, "id").unwrap_or_default();
        let domain = xml_get_attribute(&email_xml, "domain").unwrap_or_default();

        GPXEmail { id, domain }
    }
}

/// Defines the bounding box of the GPX data.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GPXBounds {
    /// Minimum latitude
    pub minlat: f64,
    /// Minimum longitude
    pub minlon: f64,
    /// Maximum latitude
    pub maxlat: f64,
    /// Maximum longitude
    pub maxlon: f64,
}
impl GPXBounds {
    /// Creates a new GPXBounds from an XMLTagItem
    pub fn new(bounds_xml: XMLTagItem) -> Self {
        let minlat = xml_get_attribute(&bounds_xml, "minlat")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let minlon = xml_get_attribute(&bounds_xml, "minlon")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let maxlat = xml_get_attribute(&bounds_xml, "maxlat")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let maxlon = xml_get_attribute(&bounds_xml, "maxlon")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        GPXBounds { minlat, minlon, maxlat, maxlon }
    }
}

/// Enumeration of GPS fix types.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GPXFixType {
    /// No fix
    #[default]
    None,
    /// 2D fix
    #[serde(rename = "2d")]
    D2,
    /// 3D fix
    #[serde(rename = "3d")]
    D3,
    /// Differential GPS
    Dgps,
    /// Precise Positioning System
    Pps,
}
impl fmt::Display for GPXFixType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            GPXFixType::None => "none",
            GPXFixType::D2 => "2d",
            GPXFixType::D3 => "3d",
            GPXFixType::Dgps => "dgps",
            GPXFixType::Pps => "pps",
        };
        write!(f, "{}", s)
    }
}
impl From<&str> for GPXFixType {
    fn from(s: &str) -> Self {
        match s {
            "none" => GPXFixType::None,
            "2d" => GPXFixType::D2,
            "3d" => GPXFixType::D3,
            "dgps" => GPXFixType::Dgps,
            "pps" => GPXFixType::Pps,
            _ => GPXFixType::None, // Default case for unrecognized strings
        }
    }
}
impl FromStr for GPXFixType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GPXFixType::from(s))
    }
}
impl TryFrom<String> for GPXFixType {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        GPXFixType::from_str(&s)
    }
}
impl From<GPXFixType> for ValueType {
    fn from(v: GPXFixType) -> Self {
        ValueType::Primitive(PrimitiveValue::String(v.to_string()))
    }
}
impl From<&ValueType> for GPXFixType {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::String(s)) => match s.to_lowercase().as_str() {
                "none" => GPXFixType::None,
                "2d" => GPXFixType::D2,
                "3d" => GPXFixType::D3,
                "dgps" => GPXFixType::Dgps,
                "pps" => GPXFixType::Pps,
                _ => GPXFixType::None,
            },
            _ => GPXFixType::None,
        }
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;
    use crate::readers::xml::XMLTag;

    #[test]
    fn test_gpx_link_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<link href="https://example.com"><text>Example Link</text><type>text/html</type></link>"#.into(),
            inner: Some(r#"<text>Example Link</text><type>text/html</type>"#.into()),
            start: 0,
            end: 78,
        });

        let link = GPXLink::new(xml_tag);

        assert_eq!(link.href, "https://example.com");
        assert_eq!(link.text, Some("Example Link".into()));
        assert_eq!(link.r#type, Some("text/html".into()));

        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<link href="https://example.com"/>"#.into(),
            inner: None,
            start: 0,
            end: 32,
        });

        let link = GPXLink::new(xml_tag);

        assert_eq!(link.href, "https://example.com");
        assert_eq!(link.text, None);
        assert_eq!(link.r#type, None);
    }

    #[test]
    fn test_gpx_bounds_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<bounds minlat="1.0" minlon="2.0" maxlat="3.0" maxlon="4.0"/>"#.into(),
            inner: None,
            start: 0,
            end: 58,
        });

        let bounds = GPXBounds::new(xml_tag);

        assert_eq!(bounds.minlat, 1.0);
        assert_eq!(bounds.minlon, 2.0);
        assert_eq!(bounds.maxlat, 3.0);
        assert_eq!(bounds.maxlon, 4.0);

        let xml_tag_invalid = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<bounds minlat="invalid" minlon="2.0" maxlat="3.0" maxlon="4.0"/>"#.into(),
            inner: None,
            start: 0,
            end: 67,
        });

        let bounds_invalid = GPXBounds::new(xml_tag_invalid);

        assert_eq!(bounds_invalid.minlat, 0.0);
        assert_eq!(bounds_invalid.minlon, 2.0);
        assert_eq!(bounds_invalid.maxlat, 3.0);
        assert_eq!(bounds_invalid.maxlon, 4.0);

        let xml_tag_missing = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<bounds/>"#.into(),
            inner: None,
            start: 0,
            end: 10,
        });

        let bounds_missing = GPXBounds::new(xml_tag_missing);

        assert_eq!(bounds_missing.minlat, 0.0);
        assert_eq!(bounds_missing.minlon, 0.0);
        assert_eq!(bounds_missing.maxlat, 0.0);
        assert_eq!(bounds_missing.maxlon, 0.0);
    }

    #[test]
    fn test_gpx_email_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<email id="user" domain="example.com"/>"#.into(),
            inner: None,
            start: 0,
            end: 42,
        });

        let email = GPXEmail::new(xml_tag);

        assert_eq!(email.id, "user");
        assert_eq!(email.domain, "example.com");

        let xml_tag_missing = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<email/>"#.into(),
            inner: None,
            start: 0,
            end: 8,
        });

        let email_missing = GPXEmail::new(xml_tag_missing);

        assert_eq!(email_missing.id, "");
        assert_eq!(email_missing.domain, "");

        let xml_tag_empty = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<email id="" domain=""/>"#.into(),
            inner: None,
            start: 0,
            end: 23,
        });

        let email_empty = GPXEmail::new(xml_tag_empty);

        assert_eq!(email_empty.id, "");
        assert_eq!(email_empty.domain, "");
    }

    #[test]
    fn test_gpx_person_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<person><name>John Doe</name><email id="user" domain="example.com"/><link href="https://example.com"/></person>"#.into(),
            inner: Some(r#"<name>John Doe</name><email id="user" domain="example.com"/><link href="https://example.com"/>"#.into()),
            start: 0,
            end: 121,
        });

        let person = GPXPerson::new(xml_tag);

        assert_eq!(person.name, Some("John Doe".into()));
        assert_eq!(
            person.email,
            Some(GPXEmail { id: "user".into(), domain: "example.com".into() })
        );
        assert_eq!(
            person.link,
            Some(GPXLink { href: "https://example.com".into(), text: None, r#type: None })
        );

        let xml_tag_missing = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<person/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 9,
        });

        let person_missing = GPXPerson::new(xml_tag_missing);

        assert_eq!(person_missing.name, None);
        assert_eq!(person_missing.email, None);
        assert_eq!(person_missing.link, None);

        let xml_tag_partial = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<person><name>John Doe</name></person>"#.into(),
            inner: Some(r#"<name>John Doe</name>"#.into()),
            start: 0,
            end: 36,
        });

        let person_partial = GPXPerson::new(xml_tag_partial);

        assert_eq!(person_partial.name, Some("John Doe".into()));
        assert_eq!(person_partial.email, None);
        assert_eq!(person_partial.link, None);
    }

    #[test]
    fn test_gpx_copyright_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<copyright><author>John Doe</author><year>2023</year><license>https://example.com/license</license></copyright>"#.into(),
            inner: Some(r#"<author>John Doe</author><year>2023</year><license>https://example.com/license</license>"#.into()),
            start: 0,
            end: 112,
        });

        let copyright = GPXCopyright::new(xml_tag);

        assert_eq!(copyright.author, "John Doe");
        assert_eq!(copyright.year, Some("2023".into()));
        assert_eq!(copyright.license, Some("https://example.com/license".into()));

        let xml_tag_missing = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<copyright/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 11,
        });

        let copyright_missing = GPXCopyright::new(xml_tag_missing);

        assert_eq!(copyright_missing.author, "");
        assert_eq!(copyright_missing.year, None);
        assert_eq!(copyright_missing.license, None);

        let xml_tag_partial = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<copyright><author>John Doe</author></copyright>"#.into(),
            inner: Some(r#"<author>John Doe</author>"#.into()),
            start: 0,
            end: 42,
        });

        let copyright_partial = GPXCopyright::new(xml_tag_partial);

        assert_eq!(copyright_partial.author, "John Doe");
        assert_eq!(copyright_partial.year, None);
        assert_eq!(copyright_partial.license, None);
    }

    #[test]
    fn test_gpx_waypoint_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<wpt lat="1.0" lon="2.0"><ele>10.0</ele><time>2023-10-27T12:00:00Z</time><name>Test Waypoint</name><link href="https://example.com"/></wpt>"#.into(),
            inner: Some(r#"<ele>10.0</ele><time>2023-10-27T12:00:00Z</time><name>Test Waypoint</name><link href="https://example.com"/>"#.into()),
            start: 0,
            end: 181,
        });

        let waypoint = GPXWaypoint::new(xml_tag);

        assert_eq!(waypoint.lat, 1.0);
        assert_eq!(waypoint.lon, 2.0);
        assert_eq!(waypoint.ele, Some(10.0));
        assert_eq!(waypoint.time, Some("2023-10-27T12:00:00Z".into()));
        assert_eq!(waypoint.name, Some("Test Waypoint".into()));
        assert_eq!(waypoint.link.as_ref().unwrap()[0].href, "https://example.com");
    }
    #[test]
    fn test_gpx_waypoint_new_minimal() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<wpt lat="1.0" lon="2.0"/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 23,
        });

        let waypoint = GPXWaypoint::new(xml_tag);

        assert_eq!(waypoint.lat, 1.0);
        assert_eq!(waypoint.lon, 2.0);
        assert_eq!(waypoint.ele, None);
        assert_eq!(waypoint.time, None);
        assert_eq!(waypoint.name, None);
        assert_eq!(waypoint.link, None);
    }

    #[test]
    fn test_gpx_waypoint_multiple_links() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
        outer: r#"<wpt lat="1.0" lon="2.0"><link href="https://example.com"/><link href="https://example2.com"/></wpt>"#.into(),
        inner: Some(r#"<link href="https://example.com"/><link href="https://example2.com"/>"#.into()),
        start: 0,
        end: 100,
      });

        let waypoint = GPXWaypoint::new(xml_tag);
        assert_eq!(waypoint.link.as_ref().unwrap().len(), 2);
        assert_eq!(waypoint.link.as_ref().unwrap()[0].href, "https://example.com");
        assert_eq!(waypoint.link.as_ref().unwrap()[1].href, "https://example2.com");
    }

    #[test]
    fn test_gpx_track_segment_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<trkseg><trkpt lat="1.0" lon="2.0"/><trkpt lat="3.0" lon="4.0"/></trkseg>"#
                .into(),
            inner: Some(r#"<trkpt lat="1.0" lon="2.0"/><trkpt lat="3.0" lon="4.0"/>"#.into()),
            start: 0,
            end: 80,
        });

        let track_segment = GPXTrackSegment::new(xml_tag);

        assert_eq!(track_segment.trkpt.as_ref().unwrap().len(), 2);
        assert_eq!(track_segment.trkpt.as_ref().unwrap()[0].lat, 1.0);
        assert_eq!(track_segment.trkpt.as_ref().unwrap()[0].lon, 2.0);
        assert_eq!(track_segment.trkpt.as_ref().unwrap()[1].lat, 3.0);
        assert_eq!(track_segment.trkpt.as_ref().unwrap()[1].lon, 4.0);
    }

    #[test]
    fn test_gpx_track_segment_new_empty() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<trkseg/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 10,
        });

        let track_segment = GPXTrackSegment::new(xml_tag);

        assert_eq!(track_segment.trkpt, None);
    }

    #[test]
    fn test_gpx_route_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<rte><name>Test Route</name><rtept lat="1.0" lon="2.0"/><rtept lat="3.0" lon="4.0"/></rte>"#.into(),
            inner: Some(r#"<name>Test Route</name><rtept lat="1.0" lon="2.0"/><rtept lat="3.0" lon="4.0"/>"#.into()),
            start: 0,
            end: 93,
        });

        let route = GPXRoute::new(xml_tag);

        assert_eq!(route.name, Some("Test Route".into()));
        assert_eq!(route.rtept.as_ref().unwrap().len(), 2);
        assert_eq!(route.rtept.as_ref().unwrap()[0].lat, 1.0);
        assert_eq!(route.rtept.as_ref().unwrap()[0].lon, 2.0);
        assert_eq!(route.rtept.as_ref().unwrap()[1].lat, 3.0);
        assert_eq!(route.rtept.as_ref().unwrap()[1].lon, 4.0);
    }

    #[test]
    fn test_gpx_route_new_empty() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<rte/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 6,
        });

        let route = GPXRoute::new(xml_tag);

        assert_eq!(route.name, None);
        assert_eq!(route.rtept, None);
    }

    #[test]
    fn test_gpx_route_new_links() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<rte><link href="https://example.com"/><link href="https://example2.com"/></rte>"#.into(),
            inner: Some(r#"<link href="https://example.com"/><link href="https://example2.com"/>"#.into()),
            start: 0,
            end: 80,
        });

        let route = GPXRoute::new(xml_tag);

        assert_eq!(route.link.as_ref().unwrap().len(), 2);
        assert_eq!(route.link.as_ref().unwrap()[0].href, "https://example.com");
        assert_eq!(route.link.as_ref().unwrap()[1].href, "https://example2.com");
    }

    #[test]
    fn test_gpx_metadata_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<metadata><name>Test Metadata</name><author><name>John Doe</name></author><bounds minlat="1.0" minlon="2.0" maxlat="3.0" maxlon="4.0"/></metadata>"#.into(),
            inner: Some(r#"<name>Test Metadata</name><author><name>John Doe</name></author><bounds minlat="1.0" minlon="2.0" maxlat="3.0" maxlon="4.0"/>"#.into()),
            start: 0,
            end: 181,
        });

        let metadata = GPXMetadata::new(xml_tag);

        assert_eq!(metadata.name, Some("Test Metadata".into()));
        assert_eq!(metadata.author.as_ref().unwrap().name, Some("John Doe".into()));
        assert_eq!(metadata.bounds.as_ref().unwrap().minlat, 1.0);
        assert_eq!(metadata.bounds.as_ref().unwrap().minlon, 2.0);
        assert_eq!(metadata.bounds.as_ref().unwrap().maxlat, 3.0);
        assert_eq!(metadata.bounds.as_ref().unwrap().maxlon, 4.0);
    }

    #[test]
    fn test_gpx_metadata_new_empty() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<metadata/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 11,
        });

        let metadata = GPXMetadata::new(xml_tag);

        assert_eq!(metadata.name, None);
        assert_eq!(metadata.author, None);
        assert_eq!(metadata.bounds, None);
    }

    #[test]
    fn test_gpx_track_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer:
                r#"<trk><name>Test Track</name><trkseg><trkpt lat="1.0" lon="2.0"/></trkseg></trk>"#
                    .into(),
            inner: Some(
                r#"<name>Test Track</name><trkseg><trkpt lat="1.0" lon="2.0"/></trkseg>"#.into(),
            ),
            start: 0,
            end: 80,
        });

        let track = GPXTrack::new(xml_tag);

        assert_eq!(track.name, Some("Test Track".into()));
        assert_eq!(track.trkseg.as_ref().unwrap().len(), 1);
        assert_eq!(track.trkseg.as_ref().unwrap()[0].trkpt.as_ref().unwrap()[0].lat, 1.0);
        assert_eq!(track.trkseg.as_ref().unwrap()[0].trkpt.as_ref().unwrap()[0].lon, 2.0);
    }

    #[test]
    fn test_gpx_track_new_empty() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<trk/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 6,
        });

        let track = GPXTrack::new(xml_tag);

        assert_eq!(track.name, None);
        assert_eq!(track.trkseg, None);
    }

    #[test]
    fn test_gpx_track_new_links() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<trk><link href="https://example.com"/><link href="https://example2.com"/></trk>"#.into(),
            inner: Some(r#"<link href="https://example.com"/><link href="https://example2.com"/>"#.into()),
            start: 0,
            end: 80,
        });

        let track = GPXTrack::new(xml_tag);

        assert_eq!(track.link.as_ref().unwrap().len(), 2);
        assert_eq!(track.link.as_ref().unwrap()[0].href, "https://example.com");
        assert_eq!(track.link.as_ref().unwrap()[1].href, "https://example2.com");
    }

    #[test]
    fn test_gpx_new() {
        let gpx_xml = r#"
            <gpx version="1.1" creator="Test Creator">
                <metadata><name>Test Metadata</name></metadata>
                <wpt lat="1.0" lon="2.0"/>
                <rte><name>Test Route</name></rte>
                <trk><name>Test Track</name></trk>
            </gpx>
        "#;

        let gpx = GPX::new(gpx_xml);

        assert_eq!(gpx.version, "1.1");
        assert_eq!(gpx.creator, "Test Creator");
        assert_eq!(gpx.metadata.as_ref().unwrap().name, Some("Test Metadata".into()));
        assert_eq!(gpx.wpt.as_ref().unwrap().len(), 1);
        assert_eq!(gpx.rte.as_ref().unwrap().len(), 1);
        assert_eq!(gpx.trk.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_gpx_new_empty() {
        let gpx_xml = r#"<gpx version="1.1" creator="Test Creator"></gpx>"#;

        let gpx = GPX::new(gpx_xml);

        assert_eq!(gpx.version, "1.1");
        assert_eq!(gpx.creator, "Test Creator");
        assert_eq!(gpx.metadata, None);
        assert_eq!(gpx.wpt, None);
        assert_eq!(gpx.rte, None);
        assert_eq!(gpx.trk, None);
    }

    #[test]
    fn test_gpx_fix_type_default() {
        assert_eq!(GPXFixType::default(), GPXFixType::None);
    }

    #[test]
    fn test_gpx_fix_type_display() {
        assert_eq!(GPXFixType::None.to_string(), "none");
        assert_eq!(GPXFixType::D2.to_string(), "2d");
        assert_eq!(GPXFixType::D3.to_string(), "3d");
        assert_eq!(GPXFixType::Dgps.to_string(), "dgps");
        assert_eq!(GPXFixType::Pps.to_string(), "pps");
    }

    #[test]
    fn test_gpx_fix_type_from_str() {
        assert_eq!(GPXFixType::from_str("none").unwrap(), GPXFixType::None);
        assert_eq!(GPXFixType::from_str("2d").unwrap(), GPXFixType::D2);
        assert_eq!(GPXFixType::from_str("3d").unwrap(), GPXFixType::D3);
        assert_eq!(GPXFixType::from_str("dgps").unwrap(), GPXFixType::Dgps);
        assert_eq!(GPXFixType::from_str("pps").unwrap(), GPXFixType::Pps);
        assert_eq!(GPXFixType::from_str("invalid").unwrap(), GPXFixType::None);
    }

    #[test]
    fn test_gpx_fix_type_from_string() {
        assert_eq!(GPXFixType::try_from("none".to_string()).unwrap(), GPXFixType::None);
        assert_eq!(GPXFixType::try_from("2d".to_string()).unwrap(), GPXFixType::D2);
        assert_eq!(GPXFixType::try_from("3d".to_string()).unwrap(), GPXFixType::D3);
        assert_eq!(GPXFixType::try_from("dgps".to_string()).unwrap(), GPXFixType::Dgps);
        assert_eq!(GPXFixType::try_from("pps".to_string()).unwrap(), GPXFixType::Pps);
        assert_eq!(GPXFixType::try_from("invalid".to_string()).unwrap(), GPXFixType::None);
    }

    #[test]
    fn test_gpx_fix_type_to_value_type() {
        let none_value: ValueType = GPXFixType::None.into();
        assert_eq!(none_value, ValueType::Primitive(PrimitiveValue::String("none".to_string())));

        let d2_value: ValueType = GPXFixType::D2.into();
        assert_eq!(d2_value, ValueType::Primitive(PrimitiveValue::String("2d".to_string())));

        let d3_value: ValueType = GPXFixType::D3.into();
        assert_eq!(d3_value, ValueType::Primitive(PrimitiveValue::String("3d".to_string())));

        let dgps_value: ValueType = GPXFixType::Dgps.into();
        assert_eq!(dgps_value, ValueType::Primitive(PrimitiveValue::String("dgps".to_string())));

        let pps_value: ValueType = GPXFixType::Pps.into();
        assert_eq!(pps_value, ValueType::Primitive(PrimitiveValue::String("pps".to_string())));
    }

    #[test]
    fn test_gpx_fix_type_from_value_type() {
        let none_value = ValueType::Primitive(PrimitiveValue::String("none".to_string()));
        assert_eq!(GPXFixType::from(&none_value), GPXFixType::None);

        let d2_value = ValueType::Primitive(PrimitiveValue::String("2d".to_string()));
        assert_eq!(GPXFixType::from(&d2_value), GPXFixType::D2);

        let d3_value = ValueType::Primitive(PrimitiveValue::String("3d".to_string()));
        assert_eq!(GPXFixType::from(&d3_value), GPXFixType::D3);

        let dgps_value = ValueType::Primitive(PrimitiveValue::String("dgps".to_string()));
        assert_eq!(GPXFixType::from(&dgps_value), GPXFixType::Dgps);

        let pps_value = ValueType::Primitive(PrimitiveValue::String("pps".to_string()));
        assert_eq!(GPXFixType::from(&pps_value), GPXFixType::Pps);

        let invalid_value = ValueType::Primitive(PrimitiveValue::String("invalid".to_string()));
        assert_eq!(GPXFixType::from(&invalid_value), GPXFixType::None);

        let non_string_value = ValueType::Nested(Default::default());
        assert_eq!(GPXFixType::from(&non_string_value), GPXFixType::None);
    }

    #[test]
    fn test_gpx_fix_type_serde_json() {
        let none_json = serde_json::to_string(&GPXFixType::None).unwrap();
        assert_eq!(none_json, "\"none\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&none_json).unwrap(), GPXFixType::None);

        let d2_json = serde_json::to_string(&GPXFixType::D2).unwrap();
        assert_eq!(d2_json, "\"2d\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&d2_json).unwrap(), GPXFixType::D2);

        let d3_json = serde_json::to_string(&GPXFixType::D3).unwrap();
        assert_eq!(d3_json, "\"3d\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&d3_json).unwrap(), GPXFixType::D3);

        let dgps_json = serde_json::to_string(&GPXFixType::Dgps).unwrap();
        assert_eq!(dgps_json, "\"dgps\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&dgps_json).unwrap(), GPXFixType::Dgps);

        let pps_json = serde_json::to_string(&GPXFixType::Pps).unwrap();
        assert_eq!(pps_json, "\"pps\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&pps_json).unwrap(), GPXFixType::Pps);
    }
}
