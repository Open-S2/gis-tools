#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use gistools::{
        parsers::{BufferReader, FeatureReader},
        readers::{
            FieldTagNames, GTiffDataType, GeoKeyDirectoryKeys, GeoPixelScale, GeoTIFFOptions,
            GeoTIFFReader, GeoTiePoint,
        },
    };
    use s2json::BBox;
    use std::{collections::BTreeMap, path::PathBuf};

    #[test]
    fn test_initial_geotiff() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/initial.tiff");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);

        // read header variables
        let header = &geotiff.header;
        let image_dir = &header.image_directories[0];

        // The pixel scale
        assert_eq!(
            image_dir.pixel_scale,
            GeoPixelScale { x: 0.03139662310517357, y: 0.031362900240700195, z: 0.0 }
        );
        // The tie point
        assert_eq!(
            image_dir.tie_point,
            GeoTiePoint { i: 0., j: 0., k: 0., x: 11.361065864562988, y: 46.25202560424805, z: 0. }
        );

        // geo_key_directory
        assert_eq!(
            image_dir.geo_key_directory.get_short(GeoKeyDirectoryKeys::GTModelTypeGeoKey as u16),
            Some(2)
        );
        assert_eq!(
            image_dir.geo_key_directory.get_short(GeoKeyDirectoryKeys::GTRasterTypeGeoKey as u16),
            Some(1)
        );
        assert_eq!(
            image_dir
                .geo_key_directory
                .get_short(GeoKeyDirectoryKeys::GeogAngularUnitsGeoKey as u16),
            Some(9102)
        );
        assert_eq!(
            image_dir.geo_key_directory.get_short(GeoKeyDirectoryKeys::GeographicTypeGeoKey as u16),
            Some(4326)
        );
        assert_eq!(
            image_dir.geo_key_directory.get_string(GeoKeyDirectoryKeys::GeogCitationGeoKey as u16),
            Some("WGS 84".into())
        );

        // variables
        assert_eq!(image_dir.variables.get_short(FieldTagNames::ImageWidth as u16), Some(539));
        assert_eq!(image_dir.variables.get_short(FieldTagNames::ImageLength as u16), Some(448));
        assert_eq!(
            image_dir.variables.get_u16s(FieldTagNames::BitsPerSample as u16),
            Some([16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16].to_vec())
        );
        assert_eq!(image_dir.variables.get_short(FieldTagNames::Compression as u16), Some(5));
        assert_eq!(
            image_dir.variables.get_short(FieldTagNames::PhotometricInterpretation as u16),
            Some(1)
        );
        assert_eq!(
            image_dir.variables.get_string(FieldTagNames::ImageDescription as u16),
            Some(
                "ENVISAT-MER_FRS_1PNPDE20060816_090929_000001972050_00222_23322_0058_uint16".into()
            )
        );
        assert_eq!(
            image_dir.variables.get_u32s(FieldTagNames::XResolution as u16),
            Some([1, 1].to_vec())
        );
        assert_eq!(
            image_dir.variables.get_u32s(FieldTagNames::YResolution as u16),
            Some([1, 1].to_vec())
        );
        assert_eq!(
            image_dir.variables.get_u16s(FieldTagNames::ExtraSamples as u16),
            Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec())
        );
        assert_eq!(
            image_dir.variables.get_u16s(FieldTagNames::SampleFormat as u16),
            Some([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].to_vec())
        );
        assert_eq!(
            image_dir.variables.get_short(FieldTagNames::PlanarConfiguration as u16),
            Some(1)
        );
        assert_eq!(image_dir.variables.get_short(FieldTagNames::ResolutionUnit as u16), Some(1));
        assert_eq!(image_dir.variables.get_short(FieldTagNames::Predictor as u16), Some(1));
        assert_eq!(image_dir.variables.get_short(FieldTagNames::SamplesPerPixel as u16), Some(15));
        assert_eq!(image_dir.variables.get_short(FieldTagNames::RowsPerStrip as u16), Some(1));
        assert_eq!(
            image_dir.variables.get_u32s(FieldTagNames::StripOffsets as u16),
            Some(
                [
                    4158, 4363, 4672, 5277, 6177, 7392, 8977, 10874, 13048, 15482, 18275, 21410,
                    24813, 28461, 32464, 36739, 41315, 46136, 51270, 56650, 62263, 68103, 74215,
                    80558, 87160, 94028, 101171, 108466, 115959, 123754, 131825, 140046, 148542,
                    157239, 166221, 175372, 184732, 194212, 203962, 213959, 224160, 234550, 245161,
                    255937, 266917, 278024, 289329, 300836, 312518, 324302, 336317, 348508, 360959,
                    373507, 386291, 399222, 412344, 425645, 439120, 452792, 466675, 480746, 495015,
                    509498, 524062, 538869, 553762, 568884, 584236, 599699, 615427, 631302, 647416,
                    663694, 680082, 696612, 713346, 730269, 747238, 764399, 781785, 799252, 816920,
                    834628, 852535, 870375, 888297, 906097, 923852, 941444, 958946, 976285, 993890,
                    1011543, 1029276, 1046793, 1064259, 1081696, 1099194, 1116715, 1134152,
                    1151438, 1168812, 1185999, 1203281, 1220542, 1237814, 1255023, 1272078,
                    1289221, 1306251, 1323211, 1340259, 1357244, 1374185, 1391065, 1407941,
                    1424847, 1441668, 1458459, 1475157, 1492017, 1508882, 1525743, 1542494,
                    1559314, 1576109, 1592933, 1609719, 1626517, 1643302, 1660116, 1676982,
                    1693826, 1710702, 1727608, 1744533, 1761477, 1778483, 1795479, 1812407,
                    1829400, 1846364, 1863289, 1880254, 1897177, 1914020, 1930789, 1947605,
                    1964423, 1981166, 1997947, 2014703, 2031504, 2048304, 2065088, 2081804,
                    2098590, 2115381, 2132176, 2148856, 2165604, 2182365, 2199140, 2215914,
                    2232712, 2249518, 2266292, 2283042, 2299795, 2316526, 2333325, 2350071,
                    2366840, 2383519, 2400263, 2417027, 2433748, 2450511, 2467292, 2484051,
                    2500755, 2517469, 2534314, 2551152, 2567997, 2584831, 2601655, 2618388,
                    2635143, 2651762, 2668303, 2684807, 2701258, 2717752, 2734237, 2750659,
                    2767047, 2783409, 2799730, 2816015, 2832372, 2848643, 2864836, 2880884,
                    2897080, 2913092, 2929120, 2945144, 2961085, 2977002, 2992915, 3008922,
                    3024938, 3040906, 3056919, 3072976, 3089144, 3105177, 3121282, 3137442,
                    3153563, 3169539, 3185558, 3201537, 3217591, 3233714, 3249768, 3265832,
                    3281875, 3297830, 3313821, 3329711, 3345640, 3361557, 3377461, 3393330,
                    3409069, 3424898, 3440613, 3456316, 3472184, 3488007, 3503815, 3519611,
                    3535377, 3551122, 3566900, 3582630, 3598375, 3614136, 3629885, 3645582,
                    3661309, 3677018, 3692707, 3708432, 3724178, 3739981, 3755759, 3771607,
                    3787419, 3803186, 3818973, 3834740, 3850567, 3866379, 3882163, 3897906,
                    3913654, 3929429, 3945262, 3961077, 3976814, 3992542, 4008176, 4023822,
                    4039472, 4055115, 4070704, 4086249, 4101721, 4117238, 4132702, 4148124,
                    4163563, 4178996, 4194499, 4209867, 4225316, 4240770, 4256188, 4271564,
                    4286848, 4302123, 4317434, 4332703, 4348023, 4363285, 4378536, 4393858,
                    4409138, 4424490, 4439894, 4455315, 4470619, 4485813, 4501060, 4516271,
                    4531432, 4546572, 4561675, 4576886, 4592019, 4607195, 4622362, 4637445,
                    4652564, 4667655, 4682668, 4697810, 4712854, 4728126, 4743373, 4758564,
                    4773772, 4789019, 4804365, 4819646, 4834923, 4850215, 4865480, 4880766,
                    4896040, 4911201, 4926427, 4941779, 4957108, 4972493, 4987921, 5003366,
                    5018802, 5034218, 5049663, 5065106, 5080569, 5095981, 5111351, 5126769,
                    5142248, 5157649, 5173059, 5188337, 5203624, 5218860, 5234042, 5249134,
                    5264142, 5279155, 5294127, 5309117, 5324062, 5338965, 5353862, 5368747,
                    5383553, 5398392, 5413220, 5428032, 5442814, 5457593, 5472465, 5487119,
                    5501604, 5515763, 5529720, 5543314, 5556815, 5570011, 5582789, 5595289,
                    5607375, 5619175, 5630701, 5641972, 5652997, 5663903, 5674575, 5685081,
                    5695471, 5705750, 5715908, 5725908, 5735704, 5745325, 5754703, 5763889,
                    5772944, 5781803, 5790390, 5798929, 5807237, 5815331, 5823239, 5830889,
                    5838359, 5845615, 5852710, 5859619, 5866374, 5872831, 5879093, 5885166,
                    5891029, 5896697, 5902229, 5907522, 5912623, 5917588, 5922356, 5926967,
                    5931428, 5935709, 5939798, 5943767, 5947497, 5951148, 5954652, 5957946,
                    5961078, 5964085, 5966842, 5969450, 5971914, 5974192, 5976327, 5978253,
                    5979985, 5981568, 5982988, 5984205, 5985264, 5986164, 5986879, 5987445,
                    5987827, 5988067, 5988272,
                ]
                .to_vec()
            )
        );
        assert_eq!(
            image_dir.variables.get_u32s(FieldTagNames::StripByteCounts as u16),
            Some(
                [
                    205, 309, 605, 900, 1215, 1585, 1897, 2174, 2434, 2793, 3135, 3403, 3648, 4003,
                    4275, 4576, 4821, 5134, 5380, 5613, 5840, 6112, 6343, 6602, 6868, 7143, 7295,
                    7493, 7795, 8071, 8221, 8496, 8697, 8982, 9151, 9360, 9480, 9750, 9997, 10201,
                    10390, 10611, 10776, 10980, 11107, 11305, 11507, 11682, 11784, 12015, 12191,
                    12451, 12548, 12784, 12931, 13122, 13301, 13475, 13672, 13883, 14071, 14269,
                    14483, 14564, 14807, 14893, 15122, 15352, 15463, 15728, 15875, 16114, 16278,
                    16388, 16530, 16734, 16923, 16969, 17161, 17386, 17467, 17668, 17708, 17907,
                    17840, 17922, 17800, 17755, 17592, 17502, 17339, 17605, 17653, 17733, 17517,
                    17466, 17437, 17498, 17521, 17437, 17286, 17374, 17187, 17282, 17261, 17272,
                    17209, 17055, 17143, 17030, 16960, 17048, 16985, 16941, 16880, 16876, 16906,
                    16821, 16791, 16698, 16860, 16865, 16861, 16751, 16820, 16795, 16824, 16786,
                    16798, 16785, 16814, 16866, 16844, 16876, 16906, 16925, 16944, 17006, 16996,
                    16928, 16993, 16964, 16925, 16965, 16923, 16843, 16769, 16816, 16818, 16743,
                    16781, 16756, 16801, 16800, 16784, 16716, 16786, 16791, 16795, 16680, 16748,
                    16761, 16775, 16774, 16798, 16806, 16774, 16750, 16753, 16731, 16799, 16746,
                    16769, 16679, 16744, 16764, 16721, 16763, 16781, 16759, 16704, 16714, 16845,
                    16838, 16845, 16834, 16824, 16733, 16755, 16619, 16541, 16504, 16451, 16494,
                    16485, 16422, 16388, 16362, 16321, 16285, 16357, 16271, 16193, 16048, 16196,
                    16012, 16028, 16024, 15941, 15917, 15913, 16007, 16016, 15968, 16013, 16057,
                    16168, 16033, 16105, 16160, 16121, 15976, 16019, 15979, 16054, 16123, 16054,
                    16064, 16043, 15955, 15991, 15890, 15929, 15917, 15904, 15869, 15739, 15829,
                    15715, 15703, 15868, 15823, 15808, 15796, 15766, 15745, 15778, 15730, 15745,
                    15761, 15749, 15697, 15727, 15709, 15689, 15725, 15746, 15803, 15778, 15848,
                    15812, 15767, 15787, 15767, 15827, 15812, 15784, 15743, 15748, 15775, 15833,
                    15815, 15737, 15728, 15634, 15646, 15650, 15643, 15589, 15545, 15472, 15517,
                    15464, 15422, 15439, 15433, 15503, 15368, 15449, 15454, 15418, 15376, 15284,
                    15275, 15311, 15269, 15320, 15262, 15251, 15322, 15280, 15352, 15404, 15421,
                    15304, 15194, 15247, 15211, 15161, 15140, 15103, 15211, 15133, 15176, 15167,
                    15083, 15119, 15091, 15013, 15142, 15044, 15272, 15247, 15191, 15208, 15247,
                    15346, 15281, 15277, 15292, 15265, 15286, 15274, 15161, 15226, 15352, 15329,
                    15385, 15428, 15445, 15436, 15416, 15445, 15443, 15463, 15412, 15370, 15418,
                    15479, 15401, 15410, 15278, 15287, 15236, 15182, 15092, 15008, 15013, 14972,
                    14990, 14945, 14903, 14897, 14885, 14806, 14839, 14828, 14812, 14782, 14779,
                    14872, 14654, 14485, 14159, 13957, 13594, 13501, 13196, 12778, 12500, 12086,
                    11800, 11526, 11271, 11025, 10906, 10672, 10506, 10390, 10279, 10158, 10000,
                    9796, 9621, 9378, 9186, 9055, 8859, 8587, 8539, 8308, 8094, 7908, 7650, 7470,
                    7256, 7095, 6909, 6755, 6457, 6262, 6073, 5863, 5668, 5532, 5293, 5101, 4965,
                    4768, 4611, 4461, 4281, 4089, 3969, 3730, 3651, 3504, 3294, 3132, 3007, 2757,
                    2608, 2464, 2278, 2135, 1926, 1732, 1583, 1420, 1217, 1059, 900, 715, 566, 382,
                    240, 205, 205,
                ]
                .to_vec()
            )
        );

        let mut image = geotiff.get_image(None).unwrap();
        let raster = image.raster_data(None);

        assert_eq!(raster.width, 539);
        assert_eq!(raster.height, 448);
        assert_eq!(raster.r#type, GTiffDataType::U16);
        // left: 3622080
        // right: 724416
        assert_eq!(raster.data.len(), 3_622_080);
        assert!(!raster.alpha);
        assert_eq!(raster.min, 0.0);
        assert_eq!(raster.max, 65_535.0);
    }

    #[test]
    fn test_initial_epsg_27563_only_pcs_code() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/projections/epsg_27563_only_pcs_code.tif");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(
            BufferReader::from(bytes),
            Some(GeoTIFFOptions {
                epsg_codes: BTreeMap::from([(
                    "27563".into(),
                    "PROJCRS[\"NTF (Paris) / Lambert Sud France\",BASEGEOGCRS[\"NTF \
                     (Paris)\",DATUM[\"Nouvelle Triangulation Francaise \
                     (Paris)\",ELLIPSOID[\"Clarke 1880 \
                     (IGN)\",6378249.2,293.466021293627,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],\
                     ID[\"EPSG\",7011]],ID[\"EPSG\",6807]],PRIMEM[\"Paris\",0.040792344,\
                     ANGLEUNIT[\"radian\",1,ID[\"EPSG\",9101]],ID[\"EPSG\",8903]],ID[\"EPSG\",\
                     4807]],CONVERSION[\"Lambert Sud France\",METHOD[\"Lambert Conic Conformal \
                     (1SP)\",ID[\"EPSG\",9801]],PARAMETER[\"Latitude of natural \
                     origin\",49,ANGLEUNIT[\"grad\",0.015707963267949,ID[\"EPSG\",9105]],ID[\"\
                     EPSG\",8801]],PARAMETER[\"Longitude of natural \
                     origin\",0,ANGLEUNIT[\"grad\",0.015707963267949,ID[\"EPSG\",9105]],ID[\"EPSG\\
                     \
                     \",8802]],PARAMETER[\"Scale factor at natural \
                     origin\",0.999877499,SCALEUNIT[\"unity\",1,ID[\"EPSG\",9201]],ID[\"EPSG\",\
                     8805]],PARAMETER[\"False \
                     easting\",600000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",\
                     8806]],PARAMETER[\"False \
                     northing\",200000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",\
                     8807]],ID[\"EPSG\",18093]],CS[Cartesian,2,ID[\"EPSG\",4499]],AXIS[\"Easting \
                     (X)\",east],AXIS[\"Northing \
                     (Y)\",north],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",27563]]"
                        .into(),
                )]),
            }),
        );

        // read header variables
        let header = &geotiff.header;
        let image_dir = &header.image_directories[0];

        // The pixel scale
        assert_eq!(
            image_dir.pixel_scale,
            GeoPixelScale { x: 0.5006369999959134, y: 0.5006370000017342, z: 0.0 }
        );
        // The tie point
        assert_eq!(
            image_dir.tie_point,
            GeoTiePoint { i: 0., j: 0., k: 0., x: 827294.1414437726, y: 523985.64443166065, z: 0. }
        );

        let mut first_image = geotiff.get_image(None).unwrap();
        let bounds = first_image.get_bbox(false);
        assert_eq!(
            bounds,
            BBox::new(827294.1414437726, 523980.63806166063, 827299.1478137725, 523985.64443166065)
        );
        let bounds_corrected = first_image.get_bbox(true);
        assert_eq!(
            bounds_corrected,
            BBox::new(5.321526135724707, 46.97709562609581, 5.321594191757158, 46.97713894791725)
        );
    }

    #[test]
    fn test_initial_geotiff_8441_oblique_mercator_laborde() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/projections/ProjectedCSTypeGeoKey_8441_oblique_mercator_laborde.tif");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(
            BufferReader::from(bytes),
            Some(GeoTIFFOptions {
                epsg_codes: BTreeMap::from([(
                    "8441".into(),
                    "PROJCRS[\"Tananarive / Laborde Grid\",BASEGEOGCRS[\"Tananarive\",DATUM[\"Tananarive 1925\",ELLIPSOID[\"International 1924\",6378388,297,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",7022]],ID[\"EPSG\",6297]],ID[\"EPSG\",4297]],CONVERSION[\"Laborde Grid (Greenwich)\",METHOD[\"Laborde Oblique Mercator\",ID[\"EPSG\",9813]],PARAMETER[\"Latitude of projection centre\",-18.9000000000003,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8811]],PARAMETER[\"Longitude of projection centre\",46.4372291666669,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8812]],PARAMETER[\"Azimuth at projection centre\",18.9000000000003,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8813]],PARAMETER[\"Scale factor at projection centre\",0.9995,SCALEUNIT[\"unity\",1,ID[\"EPSG\",9201]],ID[\"EPSG\",8815]],PARAMETER[\"False easting\",400000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8806]],PARAMETER[\"False northing\",800000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],ID[\"EPSG\",8440]],CS[Cartesian,2,ID[\"EPSG\",4530]],AXIS[\"Northing (X)\",north],AXIS[\"Easting (Y)\",east],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8441]]"
                        .into(),
                )]),
            }),
        );

        // read header variables
        let header = &geotiff.header;
        let image_dir = &header.image_directories[0];

        // The pixel scale
        assert_eq!(image_dir.pixel_scale, GeoPixelScale { x: 60., y: 60., z: 0.0 });
        // The tie point
        assert_eq!(
            image_dir.tie_point,
            GeoTiePoint { i: 0.0, j: 0.0, k: 0.0, x: 440720.0, y: 3751320.0, z: 0.0 }
        );

        let mut first_image = geotiff.get_image(None).unwrap();
        let bounds = first_image.get_bbox(false);
        assert_eq!(bounds, BBox::new(440720.0, 3751260.0, 440780.0, 3751320.0));
        let bounds_corrected = first_image.get_bbox(true);
        assert_eq!(
            bounds_corrected,
            BBox::new(46.51427439279896, 7.634598997212268, 46.514794240617476, 7.635150371503749)
        );
    }

    #[test]
    fn test_geotiff_rgba() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/RGBA.tiff");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);

        {
            let mut image = geotiff.get_image(None).unwrap();
            let raster = image.raster_data(None);
            let rgb = image.get_rgba();

            assert_eq!(raster.width, 2_474);
            assert_eq!(raster.height, 2_624);
            assert_eq!(raster.alpha, false);
            assert_eq!(raster.min, 0.);
            assert_eq!(raster.max, 200.);
            assert_eq!(rgb.width, 2_474);
            assert_eq!(rgb.height, 2_624);
            assert_eq!(rgb.alpha, true);
            assert_eq!(rgb.min, 0.);
            assert_eq!(rgb.max, 200.);

            // pull in raw data to compare:
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("tests/readers/geotiff/fixtures/RGBA_raster.rgba");
            let expected = std::fs::read(path).unwrap();
            assert_eq!(raster.to_u8s(), expected);

            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("tests/readers/geotiff/fixtures/RGBA_rgba.rgba");
            let expected = std::fs::read(path).unwrap();
            assert_eq!(rgb.to_u8s(), expected);
        }

        // let grid: Vec<_> = geotiff.iter().collect();
        // assert_eq!(grid.len(), 1);

        // let grid: Vec<_> = geotiff.par_iter(1, 0).collect();
        // assert_eq!(grid.len(), 1);
    }

    #[test]
    fn test_geotiff_ycbcr() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/ycbcr.tif");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);

        {
            let mut image = geotiff.get_image(None).unwrap();
            let raster = image.raster_data(None);
            let rgb = image.get_rgba();

            assert_eq!(raster.width, 541);
            assert_eq!(raster.height, 449);
            assert_eq!(raster.alpha, false);
            assert_eq!(raster.min, 0.);
            assert_eq!(raster.max, 255.0);
            assert_eq!(rgb.width, 541);
            assert_eq!(rgb.height, 449);
            assert_eq!(rgb.alpha, false);
            assert_eq!(rgb.min, 0.);
            assert_eq!(rgb.max, 255.0);

            // pull in raw data to compare:
            // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            // path.push("tests/readers/geotiff/fixtures/ycbcr_raster.rgba");
            // let expected = std::fs::read(path).unwrap();
            // assert_eq!(raster.to_u8s()[0..500], expected[0..500]);

            // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            // path.push("tests/readers/geotiff/fixtures/ycbcr_rgba.rgba");
            // let expected = std::fs::read(path).unwrap();
            // assert_eq!(rgb.to_u8s()[0..500], expected[0..500]);
        }

        let grid: Vec<_> = geotiff.iter().collect();
        assert_eq!(grid.len(), 1);

        let grid: Vec<_> = geotiff.par_iter(1, 0).collect();
        assert_eq!(grid.len(), 1);
    }

    #[test]
    fn test_geotiff_i32() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/int32.tiff");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);

        {
            let mut image = geotiff.get_image(None).unwrap();
            let raster = image.raster_data(None);
            // let data = raster.to_i32s();

            assert_eq!(raster.width, 539);
            assert_eq!(raster.height, 448);
            assert_eq!(raster.alpha, false);
            assert_eq!(raster.min, -32768.0);
            assert_eq!(raster.max, 32766.0);

            // // pull in raw data to compare:
            // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            // path.push("tests/readers/geotiff/fixtures/int32_raster.i32");
            // let expected = std::fs::read(path).unwrap();
            // let expected: Vec<i32> = expected
            //     .chunks_exact(4)
            //     .map(|chunk| i32::from_le_bytes(chunk.try_into().unwrap()))
            //     .collect();
            // assert_eq!(data, expected);
        }

        // let grid: Vec<_> = geotiff.iter().collect();
        // assert_eq!(grid.len(), 1);

        // let grid: Vec<_> = geotiff.par_iter(1, 0).collect();
        // assert_eq!(grid.len(), 1);
    }

    #[test]
    fn test_geotiff_cmyk() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/cmyk.tif");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);
        assert_eq!(geotiff.len(), 1);
        assert!(!geotiff.is_empty());

        {
            let mut image = geotiff.get_image(None).unwrap();
            let raster = image.raster_data(None);
            let rgb = image.get_rgba();

            assert_eq!(raster.width, 541);
            assert_eq!(raster.height, 449);
            assert_eq!(raster.alpha, false);
            assert_eq!(raster.min, 0.);
            assert_eq!(raster.max, 255.0);
            assert_eq!(rgb.width, 541);
            assert_eq!(rgb.height, 449);
            assert_eq!(rgb.alpha, false);
            assert_eq!(rgb.min, 0.);
            assert_eq!(rgb.max, 255.0);

            // pull in raw data to compare:
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("tests/readers/geotiff/fixtures/cmyk_raster.rgba");
            let expected = std::fs::read(path).unwrap();
            assert_eq!(raster.to_u8s(), expected);

            // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            // path.push("tests/readers/geotiff/fixtures/cmyk_rgba.rgba");
            // let expected = std::fs::read(path).unwrap();
            // assert!(rgb.data.len() == expected.len());
            // assert_eq!(rgb.to_u8s(), expected);
        }
    }

    #[test]
    fn test_geotiff_lzw_predictor() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/lzw_predictor.tiff");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);

        let mut image = geotiff.get_image(None).unwrap();
        let raster = image.raster_data(None);

        assert_eq!(raster.width, 539);
        assert_eq!(raster.height, 448);
        assert_eq!(raster.alpha, false);
        assert_eq!(raster.min, 0.0);
        assert_eq!(raster.max, 65535.0);
    }
}
