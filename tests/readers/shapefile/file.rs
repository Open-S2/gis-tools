#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        parsers::{FeatureReader, FileReader},
        readers::{
            SHPHeader, ShapeFileReader,
            file::{shapefile_from_gzip, shapefile_from_path},
        },
    };
    use s2json::{BBox3D, MValue, VectorFeature, VectorFeatureType, VectorGeometry, VectorPoint};
    use std::{collections::BTreeMap, path::PathBuf};

    #[test]
    fn test_shapefile_path() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/utf.shp");
        let path_str = path.to_str().unwrap();

        #[derive(Default, Debug, Clone, MValue, PartialEq)]
        struct Props {
            field: String,
        }

        let shp: ShapeFileReader<FileReader, Props> =
            shapefile_from_path(path_str, BTreeMap::from([("a".into(), "b".into())]));

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(
                    -108.97956848144531,
                    41.244772343082076,
                    -108.6328125,
                    41.253032440653186,
                    0.,
                    0.
                ),
                length: 156,
                shp_code: 1,
                version: 1000
            }
        );

        let features: Vec<VectorFeature<(), Props, ()>> = shp.iter().collect();
        assert_eq!(features.len(), 2);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(1),
                    properties: Props { field: "💩".into() },
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.6328125, 41.244772343082076, Some(())),
                        None
                    ),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(2),
                    properties: Props { field: "Hněvošický háj".into() },
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.97956848144531, 41.253032440653186, Some(())),
                        None
                    ),
                    ..Default::default()
                }
            ]
        );
    }

    #[test]
    fn test_shapefile_from_gzip() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/utf.zip");
        let path_str = path.to_str().unwrap();

        #[derive(Default, Debug, Clone, MValue, PartialEq)]
        struct Props {
            field: String,
        }

        let shp = shapefile_from_gzip(path_str, BTreeMap::from([("a".into(), "b".into())]));

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(
                    -108.97956848144531,
                    41.244772343082076,
                    -108.6328125,
                    41.253032440653186,
                    0.,
                    0.
                ),
                length: 156,
                shp_code: 1,
                version: 1000
            }
        );

        let features: Vec<VectorFeature<(), Props, ()>> = shp.par_iter(1, 0).collect();
        assert_eq!(features.len(), 2);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(1),
                    properties: Props { field: "💩".into() },
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.6328125, 41.244772343082076, Some(())),
                        None
                    ),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(2),
                    properties: Props { field: "Hněvošický háj".into() },
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.97956848144531, 41.253032440653186, Some(())),
                        None
                    ),
                    ..Default::default()
                }
            ]
        )
    }

    #[test]
    fn test_shapefile_from_gzip_senate() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/senate.zip");
        let path_str = path.to_str().unwrap();

        #[derive(Default, Debug, Clone, MValue, PartialEq)]
        #[allow(non_snake_case)]
        struct Props {
            SENATOR: String,
            SENDISTNUM: f64,
            SEN_DIST: String,
            SEN_FIRST: String,
            SEN_LAST: String,
            SEN_PARTY: String,
            SHAPE_AREA: f64,
            SHAPE_LEN: f64,
            URL: String,
        }

        let shp = shapefile_from_gzip(path_str, BTreeMap::from([("a".into(), "b".into())]));

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(
                    33861.259999997914,
                    777514.3099999987,
                    330846.0900999978,
                    959747.4400000013,
                    0.,
                    0.
                ),
                length: 408928,
                shp_code: 5,
                version: 1000
            }
        );

        let features: Vec<VectorFeature<(), Props, ()>> = shp.par_iter(1, 1).collect();
        assert_eq!(features.len(), 39);

        let first_feature = features.first().unwrap();
        assert_eq!(first_feature.id, Some(2));
        // it's a polygon. Not worth the time to extract. lmao
        assert_eq!(
            first_feature.geometry,
            VectorGeometry::new_polygon(
                vec![vec![
                    VectorPoint {
                        x: -71.67011500036428,
                        y: 42.27876299992934,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65914199993863,
                        y: 42.26679599959208,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67063299943503,
                        y: 42.26108199983574,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67644099961863,
                        y: 42.254172000200136,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67445499956249,
                        y: 42.25236499977019,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67417200010638,
                        y: 42.2485909996388,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67093200048626,
                        y: 42.244701000264676,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64902300057712,
                        y: 42.247968999806595,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64443799964454,
                        y: 42.22662600021372,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.62106999961958,
                        y: 42.230870000160316,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.59931800031788,
                        y: 42.22595899972013,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.60222099962537,
                        y: 42.21810099988459,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.58290999822319,
                        y: 42.19555899986989,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.57154799994105,
                        y: 42.19465299988569,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.57233799818465,
                        y: 42.19222800030383,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55620399949944,
                        y: 42.19185699986459,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55582899989223,
                        y: 42.18898500043653,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55654400042813,
                        y: 42.188104000404685,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55613599859751,
                        y: 42.18789300003738,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.5569509991651,
                        y: 42.18593800002868,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55578299944744,
                        y: 42.18574800019334,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55561400031259,
                        y: 42.184969000429255,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55512999947283,
                        y: 42.184960999877376,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55589799825063,
                        y: 42.18281000000751,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55549100004363,
                        y: 42.18238200040585,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55579799975216,
                        y: 42.18215900044724,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55384600010257,
                        y: 42.1819599999467,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55283999994111,
                        y: 42.18122100009251,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55259399982921,
                        y: 42.179880000427154,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55178699981973,
                        y: 42.17949799963431,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55162600040822,
                        y: 42.178847999561846,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55255600024867,
                        y: 42.177471999944366,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55392300010874,
                        y: 42.178283999977374,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55485299982239,
                        y: 42.17806000010194,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55537600051079,
                        y: 42.17739099985762,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55604399956864,
                        y: 42.17742099959793,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55716599928196,
                        y: 42.178431999573355,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55804300011499,
                        y: 42.17845800025292,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.55864899838637,
                        y: 42.17910099973691,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56133199920158,
                        y: 42.17891500014733,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.5619460002855,
                        y: 42.17849599964826,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56159299958429,
                        y: 42.17713500030381,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56191399834826,
                        y: 42.17646600009185,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56167699945212,
                        y: 42.1745759997191,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56238399944392,
                        y: 42.17416400042464,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56242200059798,
                        y: 42.17279000033901,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56311399851185,
                        y: 42.17221999991695,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56136199863275,
                        y: 42.17049899967755,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.5604989994026,
                        y: 42.168695000209496,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56093699964126,
                        y: 42.167887999567846,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56268000014488,
                        y: 42.16805199983402,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56363299982283,
                        y: 42.167179000409085,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56419300052492,
                        y: 42.16633800005792,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56440599945037,
                        y: 42.164375000437786,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56372799957501,
                        y: 42.16399399966803,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56350499955033,
                        y: 42.1632920001244,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56388199974978,
                        y: 42.16239600010849,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56349799874243,
                        y: 42.16053200003899,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56232199964526,
                        y: 42.160080999643746,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56187700013363,
                        y: 42.15901499963706,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.56103099947012,
                        y: 42.158340000193235,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.57140199947696,
                        y: 42.15184499999684,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.57159100008018,
                        y: 42.14236700032818,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.57000500029724,
                        y: 42.13918399964418,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.57341399952156,
                        y: 42.135790000445155,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.59264799873156,
                        y: 42.121720999906486,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.5960290002311,
                        y: 42.122421999816616,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.5973660000361,
                        y: 42.12300399984544,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.59910099847463,
                        y: 42.123091000139524,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.60137900013312,
                        y: 42.12452999973943,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6033899997919,
                        y: 42.124533999898,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.60542299945982,
                        y: 42.12553499970492,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.60579700027574,
                        y: 42.12490699965176,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6280529994511,
                        y: 42.149993999917726,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63300000049544,
                        y: 42.144214000164204,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6364350001977,
                        y: 42.14244699966078,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6383909995237,
                        y: 42.1406810004204,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63907199907224,
                        y: 42.138921999915304,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6395630001209,
                        y: 42.13910599957458,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64054199861546,
                        y: 42.13704299987173,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64214799972751,
                        y: 42.13540300002644,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64247899845864,
                        y: 42.1345670003847,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64260399896189,
                        y: 42.131457999626676,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63987800007143,
                        y: 42.13136299985172,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6362629984477,
                        y: 42.129783999936365,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6367709993554,
                        y: 42.128372000417585,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63608000008942,
                        y: 42.12768999982321,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63612299970303,
                        y: 42.12509600029567,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63526299978781,
                        y: 42.12415199968276,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63513399961286,
                        y: 42.12267799967273,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63420899822962,
                        y: 42.12174500008714,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63316599947578,
                        y: 42.12163400037306,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6341060000775,
                        y: 42.12163799977405,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63518099870643,
                        y: 42.12266199957345,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63538199987865,
                        y: 42.12427400004084,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63652100044898,
                        y: 42.12550900010304,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63625000029286,
                        y: 42.12764799960546,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63702700057739,
                        y: 42.128249000033954,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63656599900094,
                        y: 42.129686999934194,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64014800021432,
                        y: 42.131268999931926,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64292799951693,
                        y: 42.131292000405594,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64251499984732,
                        y: 42.13511199994815,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6406010003114,
                        y: 42.13722300000346,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64027499944824,
                        y: 42.13847699965091,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.63962199953467,
                        y: 42.139133000249416,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64189300012872,
                        y: 42.140301999894724,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6449869995883,
                        y: 42.14003799974214,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64664200023572,
                        y: 42.13785500040267,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64713999881047,
                        y: 42.13662900011749,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.64808400042457,
                        y: 42.13654699957131,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6485739999376,
                        y: 42.13486900027394,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65095500019652,
                        y: 42.13535600007464,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65140299969889,
                        y: 42.136238999947665,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65351100031292,
                        y: 42.13547200030868,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65589699924276,
                        y: 42.13400599999887,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6582859999686,
                        y: 42.133353000436635,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65720999976604,
                        y: 42.13165300039963,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6547810000458,
                        y: 42.130292000239905,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65326399993359,
                        y: 42.12872599996916,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65201300008297,
                        y: 42.12813300037116,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65148100022162,
                        y: 42.12447400007882,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6478529994855,
                        y: 42.12194100031264,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65281100004444,
                        y: 42.11684399999517,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65401800045879,
                        y: 42.11516700038326,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.65556300052471,
                        y: 42.11394800041919,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.66423799903075,
                        y: 42.11047799968769,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.66499399994329,
                        y: 42.110466999840895,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.66836499945711,
                        y: 42.11464700028434,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6691180004603,
                        y: 42.11946000015912,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67294799842408,
                        y: 42.12659999971225,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67610399856285,
                        y: 42.1250109998393,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67805300042015,
                        y: 42.12440299968975,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68043099960425,
                        y: 42.12424000037773,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68285999977253,
                        y: 42.12364399995851,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68476899882013,
                        y: 42.12204700025635,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68638899990458,
                        y: 42.12131199969854,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68686999979568,
                        y: 42.12066500021622,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6884660000317,
                        y: 42.12002999971797,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69135900053874,
                        y: 42.12133300033441,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69424200058012,
                        y: 42.121816999605485,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69413200010888,
                        y: 42.121125999879546,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.70465099845413,
                        y: 42.12522600022305,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.693712998554,
                        y: 42.136159000069625,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69136900054137,
                        y: 42.14937800035184,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69359599869486,
                        y: 42.16128999967394,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69135700055875,
                        y: 42.16329000002575,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69520399959269,
                        y: 42.175788000263985,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.71861400035924,
                        y: 42.172966999773365,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.723534999071,
                        y: 42.19148399986623,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.7339609996064,
                        y: 42.18677499968574,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.73377099916061,
                        y: 42.18350400036717,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.74053299969313,
                        y: 42.1792059995513,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.74411399837281,
                        y: 42.18250499985389,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82944399974329,
                        y: 42.144927999989775,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82485200011746,
                        y: 42.1553320003913,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83047999979557,
                        y: 42.15608199962085,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83410500043638,
                        y: 42.15498999971986,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8345719995436,
                        y: 42.15725600030999,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8254929997677,
                        y: 42.15997699963754,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82595899944798,
                        y: 42.16582000018228,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83142600045697,
                        y: 42.16435300012194,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83493900047259,
                        y: 42.15988700019173,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.85809600013589,
                        y: 42.16070600024462,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.87580199941023,
                        y: 42.170120000135185,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.89513500027499,
                        y: 42.192425000091326,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.9393559999623,
                        y: 42.189237999797825,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.96063299962017,
                        y: 42.281420999902515,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.88564099936771,
                        y: 42.28457899995849,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.87678300004613,
                        y: 42.2666940003523,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.86234999997285,
                        y: 42.26596800018514,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8612529984976,
                        y: 42.26750300027587,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.86160799999755,
                        y: 42.269826999789856,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.86102300059282,
                        y: 42.27118100031233,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.86007399965862,
                        y: 42.2718889997122,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.85674399885674,
                        y: 42.27335299983291,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.85470799849163,
                        y: 42.27381599969943,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8548689993116,
                        y: 42.27004299995632,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.85401500033805,
                        y: 42.26692900002015,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.85165399853295,
                        y: 42.263332999920955,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.85129899948323,
                        y: 42.26290499977867,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.85019400025595,
                        y: 42.26299299996399,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.84802099972451,
                        y: 42.26213999963369,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.84538199979765,
                        y: 42.26022099971338,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8439530001547,
                        y: 42.26045600017397,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8442609993084,
                        y: 42.261755000207685,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8417279997096,
                        y: 42.26205699993955,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8412839998018,
                        y: 42.26131099974773,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83901499843016,
                        y: 42.262183999778784,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83961400013598,
                        y: 42.2630949999331,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83756700033052,
                        y: 42.26396699980659,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83291500015419,
                        y: 42.26011899999936,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82989499982652,
                        y: 42.258785999758196,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82957099928795,
                        y: 42.26017700016616,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82852800028155,
                        y: 42.26004699979059,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82817799940958,
                        y: 42.26267300004973,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82377399941855,
                        y: 42.262257000090486,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82067500030243,
                        y: 42.26149300039157,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82309599902108,
                        y: 42.25700899957407,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83163499877965,
                        y: 42.24966099968025,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83084199957,
                        y: 42.249603999825716,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83114099971353,
                        y: 42.24716600012871,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8333979993763,
                        y: 42.246323000437904,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83298799997246,
                        y: 42.245068000211695,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83160499971304,
                        y: 42.24534000038396,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.83170199993191,
                        y: 42.24510699996096,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82918499952153,
                        y: 42.24412500021891,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8257850000424,
                        y: 42.24357399963564,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82088199988476,
                        y: 42.24358899975378,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.82126700033793,
                        y: 42.244188000385456,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.81949399947506,
                        y: 42.24424099973974,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.81937899904217,
                        y: 42.244677999655764,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.81593500028286,
                        y: 42.24672300002954,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.81366499971955,
                        y: 42.24503600032525,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.8094520004319,
                        y: 42.25121199982363,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.80886599856,
                        y: 42.25100300021392,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.80802299891427,
                        y: 42.252332999602984,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.80720799969978,
                        y: 42.250620000005966,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.80542499881356,
                        y: 42.25159099979763,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.80593799941691,
                        y: 42.252650999790674,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.79969899949388,
                        y: 42.2527790002865,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.79787499939529,
                        y: 42.25503800002017,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.79723199955511,
                        y: 42.254056000298355,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.79525599958392,
                        y: 42.25593799970915,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.792764000215,
                        y: 42.254660000379474,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.79156199982147,
                        y: 42.25472099984924,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.79166399963727,
                        y: 42.25550200041592,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.78916600013824,
                        y: 42.25566299985663,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.78919799871196,
                        y: 42.256403999963396,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.78485799950892,
                        y: 42.25583900038476,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.77228399853001,
                        y: 42.25642000008032,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.76733800001391,
                        y: 42.25789999976817,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.7643749987247,
                        y: 42.25943499974368,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.76168600011903,
                        y: 42.25887700005463,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.75886900053565,
                        y: 42.25971799994148,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.75226899955217,
                        y: 42.26018900013611,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.75854100032424,
                        y: 42.29622099986549,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.76821499994116,
                        y: 42.311292000177474,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.76723899978434,
                        y: 42.311443000423274,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.767198999781,
                        y: 42.31322600009722,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.75628899951931,
                        y: 42.31211600024882,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.741451999956,
                        y: 42.31717499956217,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.74174499988908,
                        y: 42.31572600009113,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.73171499818204,
                        y: 42.318024000366655,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.73007100040411,
                        y: 42.321187000288454,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69867299957293,
                        y: 42.33240599963147,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69547699949005,
                        y: 42.330267999619814,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69764300010311,
                        y: 42.32917900044148,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69758900022003,
                        y: 42.327552000266735,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69096500049618,
                        y: 42.32895900014815,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.69113899933505,
                        y: 42.326905000065175,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6895369992302,
                        y: 42.325972999924154,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68793799999051,
                        y: 42.325516000011106,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68974400029506,
                        y: 42.318861000217694,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68436200032774,
                        y: 42.31137100022295,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.68042100014422,
                        y: 42.30517100032452,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.6769960005073,
                        y: 42.28614799996217,
                        z: Some(0.0),
                        m: None,
                        t: None
                    },
                    VectorPoint {
                        x: -71.67011500036428,
                        y: 42.27876299992934,
                        z: Some(0.0),
                        m: None,
                        t: None
                    }
                ]],
                Some(BBox3D {
                    left: -71.95938728677226,
                    bottom: 42.109664675281365,
                    right: -71.55172019826269,
                    top: 42.33256604003715,
                    near: 0.0,
                    far: 0.0
                })
            )
        );
        assert_eq!(
            first_feature.properties,
            Props {
                SENATOR: "Michael O. Moore (D)".into(),
                SENDISTNUM: 38.0,
                SEN_DIST: "Second Worcester".into(),
                SEN_FIRST: "Michael O.".into(),
                SEN_LAST: "Moore".into(),
                SEN_PARTY: "(D)".into(),
                SHAPE_AREA: 381353016.695,
                SHAPE_LEN: 146050.896413,
                URL: "http://www.malegislature.gov/People/Profile/MOM0".into(),
            }
        );
    }
}
