use crate::{
    geometry::convert,
    parsers::{FeatureReader, Writer},
    readers::{
        GPXBounds, GPXCopyright, GPXFixType, GPXLink, GPXMetadata, GPXPerson, GPXRoute, GPXTrack,
        GPXTrackSegment, GPXWaypoint,
    },
    writers::build_xml_element,
};
use alloc::collections::BTreeMap;
use s2json::{
    BBox3D, JSONCollection, MValue, MValueCompatible, Projection, VectorFeature, VectorGeometry,
    VectorPoint,
};
use serde::Serialize;

/// User defined options on how to store the features
pub struct ToGPXOptions<M: Clone, P: MValueCompatible, D: MValueCompatible> {
    /// Name of the GPX file
    pub name: Option<String>,
    /// User defined metadata
    pub metadata: Option<GPXMetadata>,
    /// On a feature, determine if it should be written as a [`GPXWaypoint`].
    pub on_feature_waypoint: Option<fn(feature: &VectorFeature<M, P, D>) -> Option<GPXWaypoint>>,
    /// On a feature, determine if it should be written as a [`GPXRoute`].
    pub on_feature_route: Option<fn(feature: &VectorFeature<M, P, D>) -> Option<GPXRoute>>,
    /// On a feature, determine if it should be written as a [`GPXTrack`].
    pub on_feature_rrack: Option<fn(feature: &VectorFeature<M, P, D>) -> Option<GPXTrack>>,
}
impl<M: Clone, P: MValueCompatible, D: MValueCompatible> Default for ToGPXOptions<M, P, D> {
    fn default() -> Self {
        Self {
            name: None,
            metadata: None,
            on_feature_waypoint: None,
            on_feature_route: None,
            on_feature_rrack: None,
        }
    }
}

/// # GPX Writer
///
/// ## Description
/// Given a writer and an array of iterators, write the input features to the writer as a GPX data
///
/// ## Usage
///
/// ### Data Prep
/// Be sure to prep the various data. GPX Assumes 3 types: Waypoints (Points), Routes (LineStrings),
/// and Tracks (MultiLineStrings).
///
/// #### Waypoints
/// Check [`GPXWaypoint`]. The variables ported are `ele`, `time`, `name`, `magvar`, `geoidheight`,
/// `name`, `cmt`, `desc`, `src`, `link`, `sym`, `type`, `fix`, `sat`, `hdop`, `vdop`, `pdop`,
/// `ageofdgpsdata`, `dgpsid`. These variables are optional and should be stored in the m-values of
/// the points
///
/// #### Routes
/// Check [`GPXRoute`]. The variables ported are `name`, `cmt`, `desc`, `src`, `link`, `number`,
/// `type`, `rtept`. These variables are optional and should be stored in the properties of the feature.
///
/// #### Tracks
/// Check [`GPXTrack`]. The variables ported are `name`, `cmt`, `desc`, `src`, `link`, `number`,
/// `type`, `trkseg`. These variables are optional and should be stored in the properties of the feature.
///
/// ```ts
/// import { toGPX, JSONReader } from 'gis-tools-ts';
/// import { FileReader, FileWriter } from 'gis-tools-ts/file';
/// // or use mmap reader if using bun
/// // import { MMapReader } from 'gis-tools-ts/mmap';
///
/// const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
/// const jsonReader = new JSONReader(fileReader);
/// const bufWriter = new FileWriter(`${__dirname}/fixtures/points.gpx`);
///
/// // store to singular output
/// await toGPX(bufWriter, [jsonReader]);
/// ```
///
/// ## Links
/// https://www.topografix.com/gpx.asp
///
/// ## Parameters
/// - `writer`: the writer to append strings to
/// - `iterators`: the collection of iterators to write
/// - `opts`: user defined options [optional]
pub fn to_gpx<
    T: Writer,
    M: Clone + Serialize,
    P: MValueCompatible,
    D: MValueCompatible,
    I: FeatureReader<M, P, D>,
>(
    writer: &mut T,
    iterators: Vec<&I>,
    opts: Option<ToGPXOptions<M, P, D>>,
) {
    let opts = opts.unwrap_or_default();
    let mut bbox = BBox3D::default();
    let name = opts.name.clone().unwrap_or("GIS Tools Output Data".into());
    let feature_waypoint = opts.on_feature_waypoint.unwrap_or(on_feature_waypoint);
    let feature_route = opts.on_feature_route.unwrap_or(on_feature_route);
    let feature_track = opts.on_feature_rrack.unwrap_or(on_feature_track);

    let mut waypoints: Vec<String> = vec![];
    let mut routes: Vec<String> = vec![];
    let mut tracks: Vec<String> = vec![];

    for iterator in iterators {
        for feature in iterator.iter() {
            let converted_features =
                convert(Projection::WG, &JSONCollection::VectorFeature(feature), Some(true), None);
            for mut converted_feature in converted_features {
                bbox.merge_in_place(&converted_feature.geometry.bbox());
                // handle waypoints
                if let Some(waypoint) = feature_waypoint(&converted_feature) {
                    waypoints.push(write_waypoint(waypoint, None));
                }
                // handle routes
                if let Some(route) = feature_route(&converted_feature) {
                    routes.push(write_route(route));
                }
                // handle tracks
                if let Some(track) = feature_track(&converted_feature) {
                    tracks.push(write_track(track));
                }
            }
        }
    }

    let metadata = write_metadata(opts.metadata.unwrap_or(setup_metadata(name, bbox)));
    let mut inner_content: Vec<String> = vec![metadata];
    inner_content.extend(waypoints.into_iter());
    inner_content.extend(routes.into_iter());
    inner_content.extend(tracks.into_iter());
    let output = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n{}",
        build_xml_element(
            "gpx",
            &BTreeMap::from([
                ("version".into(), "1.1".into()),
                ("creator".into(), "GIS_TOOLS".into()),
            ]),
            &inner_content
        )
    );

    writer.append_string(&output);
}

fn on_feature_waypoint<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    feature: &VectorFeature<M, P, D>,
) -> Option<GPXWaypoint> {
    let VectorFeature { geometry, properties, .. } = feature;
    match geometry {
        VectorGeometry::Point(point) => Some(encode_point(&point.coordinates, properties)),
        _ => None,
    }
}

fn encode_point<P: MValueCompatible, D: MValueCompatible>(
    point: &VectorPoint<D>,
    properties: &P,
) -> GPXWaypoint {
    let VectorPoint { x: lon, y: lat, z: ele, m, .. } = point;
    let p_value: MValue = properties.clone().into();
    let m_value: MValue = m.clone().unwrap_or_default().into();

    let name = get_string(&p_value, "name").or(get_string(&m_value, "name"));
    let r#type = get_string(&p_value, "type").or(get_string(&m_value, "type"));
    let time = get_string(&p_value, "time").or(get_string(&m_value, "time"));
    let magvar = get_f64(&p_value, "magvar").or(get_f64(&m_value, "magvar"));
    let geoidheight = get_f64(&p_value, "geoidheight").or(get_f64(&m_value, "geoidheight"));
    let cmt = get_string(&p_value, "cmt").or(get_string(&m_value, "cmt"));
    let desc = get_string(&p_value, "desc").or(get_string(&m_value, "desc"));
    let src = get_string(&p_value, "src").or(get_string(&m_value, "src"));
    let link = get_link(&p_value).or(get_link(&m_value));
    let sym = get_string(&p_value, "sym").or(get_string(&m_value, "sym"));
    let fix = get_string(&p_value, "fix")
        .or(get_string(&m_value, "fix"))
        .map(|s| GPXFixType::from(s.as_str()));
    let sat = get_usize(&p_value, "sat").or(get_usize(&m_value, "sat"));
    let hdop = get_f64(&p_value, "hdop").or(get_f64(&m_value, "hdop"));
    let vdop = get_f64(&p_value, "vdop").or(get_f64(&m_value, "vdop"));
    let pdop = get_f64(&p_value, "pdop").or(get_f64(&m_value, "pdop"));
    let ageofdgpsdata = get_f64(&p_value, "ageofdgpsdata").or(get_f64(&m_value, "ageofdgpsdata"));
    let dgpsid = get_f64(&p_value, "dgpsid").or(get_f64(&m_value, "dgpsid"));

    GPXWaypoint {
        lon: *lon,
        lat: *lat,
        ele: ele.clone(),
        name,
        time,
        magvar,
        geoidheight,
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

fn on_feature_route<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    feature: &VectorFeature<M, P, D>,
) -> Option<GPXRoute> {
    let VectorFeature { geometry, properties, .. } = feature;
    let props_value: MValue = properties.clone().into();

    let name = get_string(&props_value, "name");
    let cmt = get_string(&props_value, "cmt");
    let desc = get_string(&props_value, "desc");
    let src = get_string(&props_value, "src");
    let r#type = get_string(&props_value, "type");
    let number = get_usize(&props_value, "number");
    let link = get_link(&props_value);

    let rtept: Vec<GPXWaypoint> = match geometry {
        VectorGeometry::LineString(line) => {
            line.coordinates.iter().map(|p| encode_point(p, properties)).collect()
        }
        _ => return None,
    };

    Some(GPXRoute { name, number, cmt, desc, src, link, r#type, rtept: Some(rtept) })
}

fn on_feature_track<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    feature: &VectorFeature<M, P, D>,
) -> Option<GPXTrack> {
    let VectorFeature { geometry, properties, .. } = feature;
    let props_value: MValue = properties.clone().into();

    let name = get_string(&props_value, "name");
    let cmt = get_string(&props_value, "cmt");
    let desc = get_string(&props_value, "desc");
    let src = get_string(&props_value, "src");
    let r#type = get_string(&props_value, "type"); // Changed from unwrap to safe Option
    let number = get_usize(&props_value, "number");
    let link = get_link(&props_value);

    // Extract geometry safely
    let mut trkseg: Vec<GPXTrackSegment> = vec![];
    match geometry {
        VectorGeometry::MultiLineString(line) => {
            for line in &line.coordinates {
                let trkpt: Vec<GPXWaypoint> =
                    line.iter().map(|p| encode_point(p, properties)).collect();
                trkseg.push(GPXTrackSegment { trkpt: Some(trkpt) });
            }
        }
        _ => return None,
    };

    Some(GPXTrack { name, number, cmt, desc, src, link, r#type, trkseg: Some(trkseg) })
}

fn setup_metadata(name: String, bounds: BBox3D) -> GPXMetadata {
    let BBox3D { left: minlat, bottom: minlon, right: maxlat, top: maxlon, .. } = bounds;
    GPXMetadata {
        name: Some(name),
        bounds: Some(GPXBounds { minlat, minlon, maxlat, maxlon }),
        desc: None,
        author: None,
        link: None,
        copyright: None,
        keywords: None,
        time: None,
    }
}

fn write_waypoint(waypoint: GPXWaypoint, tag_name: Option<String>) -> String {
    let tag_name = tag_name.unwrap_or("wpt".into());
    let GPXWaypoint {
        lon,
        lat,
        name,
        ele,
        time,
        magvar,
        geoidheight,
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
    } = waypoint;

    let mut inner: Vec<String> = vec![];
    if let Some(name) = name {
        inner.push(build_xml_element("name", &BTreeMap::new(), &vec![name]));
    }
    if let Some(ele) = ele {
        inner.push(build_xml_element("ele", &BTreeMap::new(), &vec![ele.to_string()]));
    }
    if let Some(time) = time {
        inner.push(build_xml_element("time", &BTreeMap::new(), &vec![time]));
    }
    if let Some(magvar) = magvar {
        inner.push(build_xml_element("magvar", &BTreeMap::new(), &vec![magvar.to_string()]));
    }
    if let Some(geoidheight) = geoidheight {
        inner.push(build_xml_element(
            "geoidheight",
            &BTreeMap::new(),
            &vec![geoidheight.to_string()],
        ));
    }
    if let Some(cmt) = cmt {
        inner.push(build_xml_element("cmt", &BTreeMap::new(), &vec![cmt]));
    }
    if let Some(desc) = desc {
        inner.push(build_xml_element("desc", &BTreeMap::new(), &vec![desc]));
    }
    if let Some(src) = src {
        inner.push(build_xml_element("src", &BTreeMap::new(), &vec![src]));
    }
    if let Some(link) = link {
        for l in link {
            inner.push(write_link(l));
        }
    }
    if let Some(sym) = sym {
        inner.push(build_xml_element("sym", &BTreeMap::new(), &vec![sym]));
    }
    if let Some(r#type) = r#type {
        inner.push(build_xml_element("type", &BTreeMap::new(), &vec![r#type]));
    }
    if let Some(fix) = fix {
        inner.push(build_xml_element("fix", &BTreeMap::new(), &vec![fix.to_string()]));
    }
    if let Some(sat) = sat {
        inner.push(build_xml_element("sat", &BTreeMap::new(), &vec![sat.to_string()]));
    }
    if let Some(hdop) = hdop {
        inner.push(build_xml_element("hdop", &BTreeMap::new(), &vec![hdop.to_string()]));
    }
    if let Some(vdop) = vdop {
        inner.push(build_xml_element("vdop", &BTreeMap::new(), &vec![vdop.to_string()]));
    }
    if let Some(pdop) = pdop {
        inner.push(build_xml_element("pdop", &BTreeMap::new(), &vec![pdop.to_string()]));
    }
    if let Some(ageofdgpsdata) = ageofdgpsdata {
        inner.push(build_xml_element(
            "ageofdgpsdata",
            &BTreeMap::new(),
            &vec![ageofdgpsdata.to_string()],
        ));
    }
    if let Some(dgpsid) = dgpsid {
        inner.push(build_xml_element("dgpsid", &BTreeMap::new(), &vec![dgpsid.to_string()]));
    }
    build_xml_element(
        &tag_name,
        &BTreeMap::from([("lat".into(), lat.to_string()), ("lon".into(), lon.to_string())]),
        &inner,
    )
}

fn write_route(route: GPXRoute) -> String {
    let GPXRoute { name, cmt, desc, src, link, number, r#type, rtept } = route;

    let mut inner: Vec<String> = vec![];

    if let Some(name) = name {
        inner.push(build_xml_element("name", &BTreeMap::new(), &vec![name]));
    }
    if let Some(cmt) = cmt {
        inner.push(build_xml_element("cmt", &BTreeMap::new(), &vec![cmt]));
    }
    if let Some(desc) = desc {
        inner.push(build_xml_element("desc", &BTreeMap::new(), &vec![desc]));
    }
    if let Some(src) = src {
        inner.push(build_xml_element("src", &BTreeMap::new(), &vec![src]));
    }
    if let Some(link) = link {
        for l in link {
            inner.push(write_link(l));
        }
    }
    if let Some(number) = number {
        inner.push(build_xml_element("number", &BTreeMap::new(), &vec![number.to_string()]));
    }
    if let Some(r#type) = r#type {
        inner.push(build_xml_element("type", &BTreeMap::new(), &vec![r#type]));
    }
    if let Some(rtept) = rtept {
        for w in rtept {
            inner.push(write_waypoint(w, Some("rtept".into())));
        }
    }

    build_xml_element("rte", &BTreeMap::new(), &inner)
}

fn write_track(track: GPXTrack) -> String {
    let GPXTrack { name, cmt, desc, src, link, number, r#type, trkseg } = track;

    let mut inner = vec![];

    if let Some(name) = name {
        inner.push(build_xml_element("name", &BTreeMap::new(), &vec![name]));
    }
    if let Some(cmt) = cmt {
        inner.push(build_xml_element("cmt", &BTreeMap::new(), &vec![cmt]));
    }
    if let Some(desc) = desc {
        inner.push(build_xml_element("desc", &BTreeMap::new(), &vec![desc]));
    }
    if let Some(src) = src {
        inner.push(build_xml_element("src", &BTreeMap::new(), &vec![src]));
    }
    if let Some(link) = link {
        for l in link {
            inner.push(write_link(l));
        }
    }
    if let Some(number) = number {
        inner.push(build_xml_element("number", &BTreeMap::new(), &vec![number.to_string()]));
    }
    if let Some(r#type) = r#type {
        inner.push(build_xml_element("type", &BTreeMap::new(), &vec![r#type]));
    }
    if let Some(trkseg) = trkseg {
        let mut trkpts: Vec<String> = vec![];
        for segment in trkseg {
            if let Some(trkpt) = segment.trkpt {
                for point in trkpt {
                    trkpts.push(write_waypoint(point, Some("trkpt".into())));
                }
            }
        }
        inner.push(build_xml_element("trkseg", &BTreeMap::new(), &trkpts));
    }

    build_xml_element("trk", &BTreeMap::new(), &inner)
}

fn write_metadata(metadata: GPXMetadata) -> String {
    let GPXMetadata { name, desc, author, copyright, link, time, keywords, bounds } = metadata;

    let mut inner = vec![];
    if let Some(name) = name {
        inner.push(build_xml_element("name", &BTreeMap::new(), &vec![name]));
    }
    if let Some(desc) = desc {
        inner.push(build_xml_element("desc", &BTreeMap::new(), &vec![desc]));
    }
    if let Some(link) = link {
        for l in link {
            inner.push(write_link(l));
        }
    }
    if let Some(time) = time {
        inner.push(build_xml_element("time", &BTreeMap::new(), &vec![time]));
    }
    if let Some(keywords) = keywords {
        inner.push(build_xml_element("keywords", &BTreeMap::new(), &vec![keywords]));
    }
    if let Some(author) = author {
        inner.push(write_person(&author));
    }
    if let Some(copyright) = copyright {
        inner.push(write_copyright(&copyright));
    }
    if let Some(bounds) = bounds {
        inner.push(write_bounds(&bounds));
    }

    build_xml_element("metadata", &BTreeMap::new(), &inner)
}

fn write_copyright(copyright: &GPXCopyright) -> String {
    let GPXCopyright { author, year, license } = copyright;

    let mut inner = vec![];
    inner.push(build_xml_element("author", &BTreeMap::new(), &vec![author.clone()]));
    if let Some(year) = year {
        inner.push(build_xml_element("year", &BTreeMap::new(), &vec![year.clone()]));
    }
    if let Some(license) = license {
        inner.push(build_xml_element("license", &BTreeMap::new(), &vec![license.clone()]));
    }

    build_xml_element("copyright", &BTreeMap::new(), &inner)
}

fn write_person(person: &GPXPerson) -> String {
    let GPXPerson { name, email, link } = person;

    let mut inner = vec![];
    if let Some(name) = name {
        inner.push(build_xml_element("name", &BTreeMap::new(), &vec![name.clone()]));
    }
    if let Some(email) = email {
        inner.push(build_xml_element(
            "email",
            &BTreeMap::from([
                ("id".into(), email.id.clone()),
                ("domain".into(), email.domain.clone()),
            ]),
            &vec![],
        ));
    }
    if let Some(link) = link {
        inner.push(write_link(link.clone()));
    }

    build_xml_element("author", &BTreeMap::new(), &inner)
}

fn write_bounds(bounds: &GPXBounds) -> String {
    let GPXBounds { minlon, minlat, maxlon, maxlat } = bounds;
    build_xml_element(
        "bounds",
        &BTreeMap::from([
            ("minlon".into(), minlon.to_string()),
            ("minlat".into(), minlat.to_string()),
            ("maxlon".into(), maxlon.to_string()),
            ("maxlat".into(), maxlat.to_string()),
        ]),
        &vec![],
    )
}

fn write_link(link: GPXLink) -> String {
    let GPXLink { href, text, r#type } = link;

    let mut inner = vec![];
    if let Some(text) = text {
        inner.push(build_xml_element("text", &BTreeMap::new(), &vec![text]));
    }
    if let Some(r#type) = r#type {
        inner.push(build_xml_element("type", &BTreeMap::new(), &vec![r#type]));
    }

    build_xml_element("link", &BTreeMap::from([("href".into(), href)]), &inner)
}

fn get_link(map: &MValue) -> Option<Vec<GPXLink>> {
    map.get("link").and_then(|v| v.to_vec()).map(|l_values| {
        l_values
            .into_iter()
            .filter_map(|l| {
                let nested_l = l.to_nested()?;
                // fallback to default string if internal conversions yield None
                let href = nested_l.get("href").and_then(|v| v.to_string()).unwrap_or_default();
                let text = nested_l.get("text").and_then(|v| v.to_string());
                let r#type = nested_l.get("type").and_then(|v| v.to_string());

                Some(GPXLink { href, text, r#type })
            })
            .collect()
    })
}

fn get_string(map: &MValue, key: &str) -> Option<String> {
    map.get(key).and_then(|v| v.to_prim()).map(|s| s.to_string()).flatten()
}

fn get_usize(map: &MValue, key: &str) -> Option<usize> {
    map.get(key).and_then(|v| v.to_prim()).and_then(|p| p.to_u64()).map(|n| n as usize)
}

fn get_f64(map: &MValue, key: &str) -> Option<f64> {
    map.get(key).and_then(|v| v.to_prim()).and_then(|p| p.to_f64())
}
