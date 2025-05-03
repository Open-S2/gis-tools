use super::GPXProperties;
use crate::{
    data_structures::HasLayer,
    parsers::{XMLTagItem, xml_find_tag_by_name, xml_find_tags_by_name, xml_get_attribute},
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
