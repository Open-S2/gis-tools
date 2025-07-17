#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{Coords, Transformer};

    #[test]
    fn test_transformer() {
        let mut transformer = Transformer::new();

        transformer.set_destination(r#"PROJCRS["WGS 84 / Pseudo-Mercator",
            BASEGEOGCRS["WGS 84",
                ENSEMBLE["World Geodetic System 1984 ensemble",
                    MEMBER["World Geodetic System 1984 (Transit)", ID["EPSG",1166]],
                    MEMBER["World Geodetic System 1984 (G730)", ID["EPSG",1152]],
                    MEMBER["World Geodetic System 1984 (G873)", ID["EPSG",1153]],
                    MEMBER["World Geodetic System 1984 (G1150)", ID["EPSG",1154]],
                    MEMBER["World Geodetic System 1984 (G1674)", ID["EPSG",1155]],
                    MEMBER["World Geodetic System 1984 (G1762)", ID["EPSG",1156]],
                    MEMBER["World Geodetic System 1984 (G2139)", ID["EPSG",1309]],
                    MEMBER["World Geodetic System 1984 (G2296)", ID["EPSG",1383]],
                    ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7030]],
                    ENSEMBLEACCURACY[2],
                    ID["EPSG",6326]],
                ID["EPSG",4326]],
            CONVERSION["Popular Visualisation Pseudo-Mercator",
                METHOD["Popular Visualisation Pseudo Mercator",ID["EPSG",1024]],
                PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],
                PARAMETER["Longitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],
                PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],
                PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],
                ID["EPSG",3856]],
            CS[Cartesian,2,ID["EPSG",4499]],
            AXIS["Easting (X)",east],
            AXIS["Northing (Y)",north],
            LENGTHUNIT["metre",1,ID["EPSG",9001]],
            ID["EPSG",3857]]"#.to_string()
        );

        let mut point = Coords::new_xy(0., 0.);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(0., 0.));

        point = Coords::new_xy(1., 1.);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(111319.49079327357, 111325.1428663851));

        // INVERSE
        point = Coords::new_xy(111319.49079327357, 111325.1428663851);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(0.9999999999999998, 0.9999999999999998));
    }

    #[test]
    fn test_transform_utm() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCRS[\"NAD83 / UTM zone 15N\",BASEGEOGCRS[\"NAD83\",DATUM[\"North American Datum \
             1983\",ELLIPSOID[\"GRS \
             1980\",6378137,298.257222101,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",\
             7019]],ID[\"EPSG\",6269]],ID[\"EPSG\",4269]],CONVERSION[\"UTM zone \
             15N\",METHOD[\"Transverse Mercator\",ID[\"EPSG\",9807]],PARAMETER[\"Latitude of \
             natural origin\",0,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"\
             EPSG\",8801]],PARAMETER[\"Longitude of natural \
             origin\",-93,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",\
             8802]],PARAMETER[\"Scale factor at natural \
             origin\",0.9996,SCALEUNIT[\"unity\",1,ID[\"EPSG\",9201]],ID[\"EPSG\",8805]],\
             PARAMETER[\"False \
             easting\",500000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8806]],\
             PARAMETER[\"False \
             northing\",0,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],ID[\"EPSG\"\
             ,16015]],CS[Cartesian,2,ID[\"EPSG\",4400]],AXIS[\"Easting \
             (E)\",east],AXIS[\"Northing \
             (N)\",north],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",26915]]"
                .into(),
        );

        let mut point = Coords::new_xy(470692.44, 4602888.9);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-93.35156259019989, 41.577148395419115));

        // INVERSE
        point = Coords::new_xy(-93.35156259019989, 41.577148395419115);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(470692.44000000035, 4602888.900000001));
    }

    #[test]
    fn test_transform_paris_lcc() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCRS[\"NTF (Paris) / Lambert Sud France\",BASEGEOGCRS[\"NTF \
             (Paris)\",DATUM[\"Nouvelle Triangulation Francaise (Paris)\",ELLIPSOID[\"Clarke 1880 \
             (IGN)\",6378249.2,293.466021293627,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"\
             EPSG\",7011]],ID[\"EPSG\",6807]],PRIMEM[\"Paris\",0.040792344,ANGLEUNIT[\"radian\",1,\
             ID[\"EPSG\",9101]],ID[\"EPSG\",8903]],ID[\"EPSG\",4807]],CONVERSION[\"Lambert Sud \
             France\",METHOD[\"Lambert Conic Conformal \
             (1SP)\",ID[\"EPSG\",9801]],PARAMETER[\"Latitude of natural \
             origin\",49,ANGLEUNIT[\"grad\",0.015707963267949,ID[\"EPSG\",9105]],ID[\"EPSG\",\
             8801]],PARAMETER[\"Longitude of natural \
             origin\",0,ANGLEUNIT[\"grad\",0.015707963267949,ID[\"EPSG\",9105]],ID[\"EPSG\",\
             8802]],PARAMETER[\"Scale factor at natural \
             origin\",0.999877499,SCALEUNIT[\"unity\",1,ID[\"EPSG\",9201]],ID[\"EPSG\",8805]],\
             PARAMETER[\"False \
             easting\",600000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8806]],\
             PARAMETER[\"False \
             northing\",200000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],ID[\"\
             EPSG\",18093]],CS[Cartesian,2,ID[\"EPSG\",4499]],AXIS[\"Easting \
             (X)\",east],AXIS[\"Northing \
             (Y)\",north],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",27563]]"
                .into(),
        );

        let mut point = Coords::new_xy(827294.1414437726, 523980.63806166063);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(5.321526135724707, 46.97709562609581));

        // INVERSE
        let mut point = Coords::new_xy(5.321526135724707, 46.97709562609581);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(827294.1414437726, 523842.8507979368));
    }

    #[test]
    fn test_transform_cassini_soldner() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCRS[\"Palestine 1923 / Palestine Grid\",BASEGEOGCRS[\"Palestine \
             1923\",DATUM[\"Palestine 1923\",ELLIPSOID[\"Clarke 1880 \
             (Benoit)\",6378300.789,293.46631553898,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\\
             \
             \"EPSG\",7010]],ID[\"EPSG\",6281]],ID[\"EPSG\",4281]],CONVERSION[\"Palestine \
             Grid\",METHOD[\"Cassini-Soldner\",ID[\"EPSG\",9806]],PARAMETER[\"Latitude of natural \
             origin\",31.7340969444447,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],\
             ID[\"EPSG\",8801]],PARAMETER[\"Longitude of natural \
             origin\",35.2120805555558,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],\
             ID[\"EPSG\",8802]],PARAMETER[\"False \
             easting\",170251.555,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8806]],\
             PARAMETER[\"False \
             northing\",126867.909,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],\
             ID[\"EPSG\",18201]],CS[Cartesian,2,ID[\"EPSG\",4400]],AXIS[\"Easting \
             (E)\",east],AXIS[\"Northing \
             (N)\",north],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",28191]]"
                .into(),
        );

        let mut point = Coords::new_xy(440720.0, 3751260.0);
        transformer.forward_mut(&mut point);
        // X: 40.7910238690 Y: 64.2243433975
        assert_eq!(point, Coords::new_xy(40.79425900508108, 64.24034708725664));

        // INVERSE
        let mut point = Coords::new_xy(40.79425900508108, 64.24034708725664);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(440720.00000036013, 3751259.999999994));
    }

    #[test]
    fn test_transform_polyconic() {
        // TODO: It needs to implement proper Cartesian conversion (handle the CS[Cartesian,2,ID[\"EPSG\",4499]],)
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCRS[\"SAD69 / Brazil Polyconic\",BASEGEOGCRS[\"SAD69\",DATUM[\"South American \
             Datum 1969\",ELLIPSOID[\"GRS 1967 \
             Modified\",6378160,298.25,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",\
             7050]],ID[\"EPSG\",6618]],ID[\"EPSG\",4618]],CONVERSION[\"Brazil \
             Polyconic\",METHOD[\"American Polyconic\",ID[\"EPSG\",9818]],PARAMETER[\"Latitude of \
             natural origin\",0,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"\
             EPSG\",8801]],PARAMETER[\"Longitude of natural \
             origin\",-54,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",\
             8802]],PARAMETER[\"False \
             easting\",5000000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8806]],\
             PARAMETER[\"False \
             northing\",10000000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],ID[\\
             \
             \"EPSG\",19941]],CS[Cartesian,2,ID[\"EPSG\",4499]],AXIS[\"Easting \
             (X)\",east],AXIS[\"Northing \
             (Y)\",north],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",29101]]"
                .into(),
        );
        let mut point = Coords::new_xy(440720.0, 3751260.0);
        transformer.forward_mut(&mut point);
        // -113.3199662151629, -41.66728402682234
        // X: -113.3200940304 Y: -41.6672566361
        assert_eq!(point, Coords::new_xy(-113.20585647093749, -41.5811270257062));

        // INVERSE
        let mut point = Coords::new_xy(-113.20585647093749, -41.5811270257062);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(440720.00000000093, 3616613.806924667));

        // "PROJCRS[
        //    \"SAD69 / Brazil Polyconic\",
        //    BASEGEOGCRS[
        //.       \"SAD69\",
        //.       DATUM[
        //.           \"South American Datum 1969\",
        //.           ELLIPSOID[\"GRS 1967 Modified\",6378160,298.25,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],
        //.           ID[\"EPSG\",7050]
        //.       ],
        //.       ID[\"EPSG\",6618]
        //.   ],
        //.   ID[\"EPSG\",4618]],
        //  CONVERSION[
        //.    \"Brazil Polyconic\",
        //.    METHOD[\"American Polyconic\",ID[\"EPSG\",9818]],
        //.    PARAMETER[\"Latitude of natural origin\",0,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8801]],
        //.    PARAMETER[\"Longitude of natural origin\",-54,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8802]],
        //.    PARAMETER[\"False easting\",5000000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8806]],
        //.    PARAMETER[\"False northing\",10000000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],
        //.    ID[\"EPSG\",19941]
        //.  ],
        //.  CS[Cartesian,2,ID[\"EPSG\",4499]],
        //.  AXIS[\"Easting (X)\",east],
        //.  AXIS[\"Northing (Y)\",north],
        //.  LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",29101]]"
    }

    #[test]
    fn test_transform_hotine_oblique() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"CH1903 / \
             LV03\",GEOGCS[\"CH1903\",DATUM[\"D_CH1903\",SPHEROID[\"Bessel_1841\",6377397.155,299.\
             1528128]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.017453292519943295]],PROJECTION[\
             \"Hotine_Oblique_Mercator_Azimuth_Center\"],PARAMETER[\"latitude_of_center\",46.\
             95240555555556],PARAMETER[\"longitude_of_center\",7.439583333333333],PARAMETER[\"\
             azimuth\",90],PARAMETER[\"scale_factor\",1],PARAMETER[\"false_easting\",600000],\
             PARAMETER[\"false_northing\",200000],UNIT[\"Meter\",1]]"
                .into(),
        );

        let mut point = Coords::new_xy(660013.4882918689, 185172.17110117766);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(8.226069554179047, 46.81630130834831));

        // INVERSE
        let mut point = Coords::new_xy(8.226069554179047, 46.81630130834831);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(660013.4882918688, 185172.17110117723));
    }

    #[test]
    fn test_transform_lcc() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"NAD83 / Massachusetts \
             Mainland\",GEOGCS[\"GCS_North_American_1983\",DATUM[\"D_North_American_1983\",\
             SPHEROID[\"GRS_1980\",6378137,298.257222101]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\"\
             ,0.017453292519943295]],PROJECTION[\"Lambert_Conformal_Conic\"],PARAMETER[\"\
             standard_parallel_1\",42.68333333333333],PARAMETER[\"standard_parallel_2\",41.\
             71666666666667],PARAMETER[\"latitude_of_origin\",41],PARAMETER[\"central_meridian\",\
             -71.5],PARAMETER[\"false_easting\",200000],PARAMETER[\"false_northing\",750000],\
             UNIT[\"Meter\",1]]"
                .into(),
        );
        let mut point = Coords::new_xy(231394.84, 902621.11);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-71.11881762226217, 42.373462604845976));

        // INVERSE
        let mut point = Coords::new_xy(-71.11881762226217, 42.373462604845976);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(231394.83999999953, 902636.0638467963));
    }

    #[test]
    fn test_transform_nec() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"Asia_North_Equidistant_Conic\",GEOGCS[\"GCS_WGS_1984\",DATUM[\"D_WGS_1984\",\
             SPHEROID[\"WGS_1984\",6378137,298.257223563]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\"\
             ,0.017453292519943295]],PROJECTION[\"Equidistant_Conic\"],PARAMETER[\"False_Easting\"\
             ,0],PARAMETER[\"False_Northing\",0],PARAMETER[\"Central_Meridian\",95],PARAMETER[\"\
             Standard_Parallel_1\",15],PARAMETER[\"Standard_Parallel_2\",65],PARAMETER[\"\
             Latitude_Of_Origin\",30],UNIT[\"Meter\",1]]"
                .into(),
        );
        let mut point = Coords::new_xy(88280.59904432714, 111340.90165417176);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(96.0005539986119, 31.12514852006117));

        // TODO: INVERSE
        // let mut point = Coords::new_xy(96.0005539986119, 31.12514852006117);
        // transformer.inverse_mut(&mut point);
        // assert_eq!(point, Coords::new_xy(88280.59904432714, 111340.90165417176));
    }

    #[test]
    fn test_transform_sin() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"World_Sinusoidal\",GEOGCS[\"GCS_WGS_1984\",DATUM[\"WGS_1984\",SPHEROID[\"\
             WGS_1984\",6378137,298.257223563]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.\
             017453292519943295]],PROJECTION[\"Sinusoidal\"],PARAMETER[\"False_Easting\",0],\
             PARAMETER[\"False_Northing\",0],PARAMETER[\"Central_Meridian\",0],UNIT[\"Meter\",1],\
             AUTHORITY[\"EPSG\",\"54008\"]]"
                .into(),
        );
        let mut point = Coords::new_xy(738509.49, 5874620.38);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(10.999999980589731, 53.00000003761102));

        // INVERSE
        let mut point = Coords::new_xy(10.999999980589731, 53.00000003761102);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(738509.49, 5890046.489219537));
    }

    #[test]
    fn test_transform_laea() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"ETRS89 / \
             ETRS-LAEA\",GEOGCS[\"ETRS89\",DATUM[\"D_ETRS_1989\",SPHEROID[\"GRS_1980\",6378137,\
             298.257222101]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.017453292519943295]],\
             PROJECTION[\"Lambert_Azimuthal_Equal_Area\"],PARAMETER[\"latitude_of_origin\",52],\
             PARAMETER[\"central_meridian\",10],PARAMETER[\"false_easting\",4321000],PARAMETER[\"\
             false_northing\",3210000],UNIT[\"Meter\",1]]"
                .into(),
        );
        let mut point = Coords::new_xy(4388138.6, 3321736.46);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(11.00016692710655, 53.12718464234152));

        // INVERSE
        let mut point = Coords::new_xy(11.00016692710655, 53.12718464234152);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(4387947.452306732, 3335394.0260556303));
    }

    #[test]
    fn test_transform_nad83() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"NAD83(CSRS) / UTM zone \
             17N\",GEOGCS[\"NAD83(CSRS)\",DATUM[\"D_North_American_1983_CSRS98\",SPHEROID[\"\
             GRS_1980\",6378137,298.257222101]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.\
             017453292519943295]],PROJECTION[\"Transverse_Mercator\"],PARAMETER[\"\
             latitude_of_origin\",0],PARAMETER[\"central_meridian\",-81],PARAMETER[\"scale_factor\
             \",0.9996],PARAMETER[\"false_easting\",500000],PARAMETER[\"false_northing\",0],UNIT[\
             \"Meter\",1]]"
                .into(),
        );
        let mut point = Coords::new_xy(411461.807497, 4700123.744402);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords(-82.07666015625497, 42.44838867188127, 0., 0.));

        // INVERSE
        let mut point = Coords(-82.07666015625497, 42.44838867188127, 0., 0.);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(411461.8074970003, 4700123.744402));
    }

    #[test]
    fn test_transform_utm_32n() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"ETRS89 / UTM zone \
             32N\",GEOGCS[\"ETRS89\",DATUM[\"European_Terrestrial_Reference_System_1989\",\
             SPHEROID[\"GRS \
             1980\",6378137,298.257222101,AUTHORITY[\"EPSG\",\"7019\"]],TOWGS84[0,0,0,0,0,0,0],\
             AUTHORITY[\"EPSG\",\"6258\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],\
             UNIT[\"degree\",0.0174532925199433,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\\
             \
             \
             \
             \"4258\"]],PROJECTION[\"Extended_Transverse_Mercator\"],PARAMETER[\"\
             latitude_of_origin\",0],PARAMETER[\"central_meridian\",9],PARAMETER[\"scale_factor\",\
             0.9996],PARAMETER[\"false_easting\",500000],PARAMETER[\"false_northing\",0],UNIT[\"\
             metre\",1,AUTHORITY[\"EPSG\",\"9001\"]],AXIS[\"Easting\",EAST],AXIS[\"Northing\",\
             NORTH],AUTHORITY[\"EPSG\",\"25832\"]]"
                .into(),
        );
        let mut point = Coords::new_xy(-1877994.66, 3932281.56);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords(-16.100000002370027, 32.879999998811684, 0., 0.));

        // INVERSE:
        let mut point = Coords(-16.100000002370027, 32.879999998811684, 0., 0.);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-1877994.6600000006, 3932281.56));
    }

    #[test]
    fn test_transform_utm_moll() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"World_Mollweide\",GEOGCS[\"GCS_WGS_1984\",DATUM[\"WGS_1984\",SPHEROID[\"\
             WGS_1984\",6378137,298.257223563]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.\
             017453292519943295]],PROJECTION[\"Mollweide\"],PARAMETER[\"False_Easting\",0],\
             PARAMETER[\"False_Northing\",0],PARAMETER[\"Central_Meridian\",0],UNIT[\"Meter\",1],\
             AUTHORITY[\"EPSG\",\"54009\"]]"
                .into(),
        );
        let mut point = Coords::new_xy(3891383.58309223, 6876758.9933288);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(59.99999999999991, 59.999999999999986));

        // TODO: INVERSE
        // let mut point = Coords::new_xy(59.99999999999991, 59.999999999999986);
        // transformer.inverse_mut(&mut point);
        // assert_eq!(point, Coords::new_xy(3891383.58309223, 6876758.9933288));
    }

    #[test]
    fn test_transform_bc_albers() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"NAD83 / BC \
             Albers\",GEOGCS[\"NAD83\",DATUM[\"North_American_Datum_1983\",SPHEROID[\"GRS \
             1980\",6378137,298.257222101,AUTHORITY[\"EPSG\",\"7019\"]],AUTHORITY[\"EPSG\",\"6269\\
             \
             \
             \
             \"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.\
             01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4269\"]],UNIT[\"\
             metre\",1,AUTHORITY[\"EPSG\",\"9001\"]],PROJECTION[\"Albers_Conic_Equal_Area\"],\
             PARAMETER[\"standard_parallel_1\",50],PARAMETER[\"standard_parallel_2\",58.5],\
             PARAMETER[\"latitude_of_center\",45],PARAMETER[\"longitude_of_center\",-126],\
             PARAMETER[\"false_easting\",1000000],PARAMETER[\"false_northing\",0],AUTHORITY[\"\
             EPSG\",\"3005\"],AXIS[\"Easting\",EAST],AXIS[\"Northing\",NORTH]]"
                .into(),
        );
        let mut point = Coords::new_xy(964813.103719, 1016486.305862);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-126.53999999999243, 54.15000000000263));

        // INVERSE
        let mut point = Coords::new_xy(-126.53999999999243, 54.15000000000263);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(964813.1037189998, 1016486.3058619986));
    }

    #[test]
    fn test_transform_np_azimuthal_eq() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"North_Pole_Azimuthal_Equidistant\",GEOGCS[\"GCS_WGS_1984\",DATUM[\"WGS_1984\\
             \
             \
             \
             \",SPHEROID[\"WGS_1984\",6378137,298.257223563]],PRIMEM[\"Greenwich\",0],UNIT[\"\
             Degree\",0.017453292519943295]],PROJECTION[\"Azimuthal_Equidistant\"],PARAMETER[\"\
             False_Easting\",0],PARAMETER[\"False_Northing\",0],PARAMETER[\"Central_Meridian\",0],\
             PARAMETER[\"Latitude_Of_Origin\",90],UNIT[\"Meter\",1],AUTHORITY[\"EPSG\",\"102016\"\
             ]]"
            .into(),
        );
        let mut point = Coords::new_xy(5112279.911077, -4143196.76625);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(50.977303830209756, 30.91526009373788));

        // INVERSE
        let mut point = Coords::new_xy(50.977303830209756, 30.91526009373788);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(5101306.389509294, -4134303.3840673887));
    }

    #[test]
    fn test_transform_mount_dillon_tobago_grid_cassini() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"Mount Dillon / Tobago Grid\",GEOGCS[\"Mount \
             Dillon\",DATUM[\"Mount_Dillon\",SPHEROID[\"Clarke \
             1858\",6378293.645208759,294.2606763692654,AUTHORITY[\"EPSG\",\"7007\"]],AUTHORITY[\"\
             EPSG\",\"6157\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\\
             \
             \
             \
             \",0.01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4157\"]],\
             UNIT[\"Clarke\'s \
             link\",0.201166195164,AUTHORITY[\"EPSG\",\"9039\"]],PROJECTION[\"Cassini_Soldner\"],\
             PARAMETER[\"latitude_of_origin\",11.25217861111111],PARAMETER[\"central_meridian\",\
             -60.68600888888889],PARAMETER[\"false_easting\",187500],PARAMETER[\"false_northing\",\
             180000],AUTHORITY[\"EPSG\",\"2066\"],AXIS[\"Easting\",EAST],AXIS[\"Northing\",NORTH]]"
                .into(),
        );
        let mut point = Coords::new_xy(192524.3061766178, 178100.2740019509);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-60.63999998717283, 11.235080931047047));

        // INVERSE:
        let mut point = Coords::new_xy(-60.63999998717283, 11.235080931047047);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(192524.30617711754, 178100.2740019492));
    }

    #[test]
    fn test_transform_bc_albers_nad83() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"NAD83 / BC \
             Albers\",GEOGCS[\"GCS_North_American_1983\",DATUM[\"D_North_American_1983\",\
             SPHEROID[\"GRS_1980\",6378137,298.257222101]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\"\
             ,0.017453292519943295]],PROJECTION[\"Albers\"],PARAMETER[\"standard_parallel_1\",50],\
             PARAMETER[\"standard_parallel_2\",58.5],PARAMETER[\"latitude_of_origin\",45],\
             PARAMETER[\"central_meridian\",-126],PARAMETER[\"false_easting\",1000000],PARAMETER[\\
             \
             \"false_northing\",0],UNIT[\"Meter\",1]]"
                .into(),
        );
        let mut point = Coords::new_xy(964813.103719, 1016486.305862);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-126.53999999999243, 54.15000000000263));

        // INVERSE:
        let mut point = Coords::new_xy(-126.53999999999243, 54.15000000000263);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(964813.1037189998, 1016486.3058619986));
    }

    #[test]
    fn test_transform_ferro_krovak() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"S-JTSK (Ferro) / Krovak\",GEOGCS[\"S-JTSK \
             (Ferro)\",DATUM[\"S_JTSK_Ferro\",SPHEROID[\"Bessel \
             1841\",6377397.155,299.1528128,AUTHORITY[\"EPSG\",\"7004\"]],AUTHORITY[\"EPSG\",\"\
             6818\"]],PRIMEM[\"Ferro\",-17.66666666666667,AUTHORITY[\"EPSG\",\"8909\"]],UNIT[\"\
             degree\",0.01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4818\\
             \
             \
             \
             \"]],UNIT[\"metre\",1,AUTHORITY[\"EPSG\",\"9001\"]],PROJECTION[\"Krovak\"],\
             PARAMETER[\\
             \
             \"latitude_of_center\",49.5],PARAMETER[\"longitude_of_center\",42.5],PARAMETER[\"\
             azimuth\",30.28813972222222],PARAMETER[\"pseudo_standard_parallel_1\",78.5],\
             PARAMETER[\"scale_factor\",0.9999],PARAMETER[\"false_easting\",0],PARAMETER[\"\
             false_northing\",0],AUTHORITY[\"EPSG\",\"2065\"],AXIS[\"Y\",WEST],AXIS[\"X\",SOUTH]]"
                .into(),
        );
        let mut point = Coords::new_xy(-544115.474379, -1144058.330762);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(37.08956301296606, 69.72361147765521));

        // TODO: INVERSE:
        // let mut point = Coords::new_xy(37.08956301296606, 69.72361147765521);
        // transformer.inverse_mut(&mut point);
        // assert_eq!(point, Coords::new_xy(-544115.474379, -1144058.330762));
    }

    #[test]
    fn test_transform_jtsk_krovak() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"S-JTSK (Ferro) / Krovak\",GEOGCS[\"S-JTSK \
             (Ferro)\",DATUM[\"S_JTSK_Ferro\",SPHEROID[\"Bessel \
             1841\",6377397.155,299.1528128,AUTHORITY[\"EPSG\",\"7004\"]],AUTHORITY[\"EPSG\",\"\
             6818\"]],PRIMEM[\"Ferro\",-17.66666666666667,AUTHORITY[\"EPSG\",\"8909\"]],UNIT[\"\
             degree\",0.01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4818\\
             \
             \
             \
             \"]],UNIT[\"metre\",1,AUTHORITY[\"EPSG\",\"9001\"]],PROJECTION[\"Krovak\"],\
             PARAMETER[\\
             \
             \"latitude_of_center\",49.5],PARAMETER[\"longitude_of_center\",42.5],PARAMETER[\"\
             azimuth\",30.28813972222222],PARAMETER[\"pseudo_standard_parallel_1\",78.5],\
             PARAMETER[\"scale_factor\",0.9999],PARAMETER[\"false_easting\",0],PARAMETER[\"\
             false_northing\",0],AUTHORITY[\"EPSG\",\"2065\"],AXIS[\"Y\",WEST],AXIS[\"X\",SOUTH]]"
                .into(),
        );
        let mut point = Coords::new_xy(-544115.474379, -1144058.330762);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(37.08956301296606, 69.72361147765521));

        // TODO: INVERSE:
        // let mut point = Coords::new_xy(37.08956301296606, 69.72361147765521);
        // transformer.inverse_mut(&mut point);
        // assert_eq!(point, Coords::new_xy(-544115.474379, -1144058.330762));
    }

    #[test]
    fn test_transform_miller_cyl() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"Sphere_Miller_Cylindrical\",GEOGCS[\"GCS_Sphere\",DATUM[\"D_Sphere\",\
             SPHEROID[\"Sphere\",6371000,0]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.\
             017453292519943295]],PROJECTION[\"Miller_Cylindrical\"],PARAMETER[\"False_Easting\",\
             0],PARAMETER[\"False_Northing\",0],PARAMETER[\"Central_Meridian\",0],UNIT[\"Meter\",\
             1]]"
            .into(),
        );
        let mut point = Coords::new_xy(-155375.88535614178, 1404635.2633403721);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-1.3973289073953, 12.567196566642785));

        // INVERSE:
        let mut point = Coords::new_xy(-1.3973289073953, 12.567196566642785);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-155375.88535614178, 1404635.2633403721));
    }

    #[test]
    fn test_transform_new_zealand_grid() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"NZGD49 / New Zealand Map \
             Grid\",GEOGCS[\"NZGD49\",DATUM[\"D_New_Zealand_1949\",SPHEROID[\"International_1924\"\
             ,6378388,297]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.017453292519943295]],\
             PROJECTION[\"New_Zealand_Map_Grid\"],PARAMETER[\"latitude_of_origin\",-41],\
             PARAMETER[\"central_meridian\",173],PARAMETER[\"false_easting\",2510000],PARAMETER[\"\
             false_northing\",6023150],UNIT[\"Meter\",1]]"
                .into(),
        );
        let mut point = Coords::new_xy(2464770.343667, 6056137.861919);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(172.46484795464002, -40.70170504823121));

        // INVERSE:
        let mut point = Coords::new_xy(172.46484795464002, -40.70170504823121);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(2464770.343666981, 6056137.861922069));
    }

    #[test]
    fn test_transform_nakhl_e_taqi_hotine() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"Rassadiran / Nakhl e Taqi\", GEOGCS[\"Rassadiran\", DATUM[\"Rassadiran\", \
             SPHEROID[\"International 1924\",6378388,297, AUTHORITY[\"EPSG\",\"7022\"]], \
             TOWGS84[-133.63,-157.5,-158.62,0,0,0,0], AUTHORITY[\"EPSG\",\"6153\"]], \
             PRIMEM[\"Greenwich\",0, AUTHORITY[\"EPSG\",\"8901\"]], \
             UNIT[\"degree\",0.0174532925199433, AUTHORITY[\"EPSG\",\"9122\"]], \
             AUTHORITY[\"EPSG\",\"4153\"]], \
             PROJECTION[\"Hotine_Oblique_Mercator_Azimuth_Center\"], \
             PARAMETER[\"latitude_of_center\",27.51882880555555], \
             PARAMETER[\"longitude_of_center\",52.60353916666667], \
             PARAMETER[\"azimuth\",0.5716611944444444], \
             PARAMETER[\"rectified_grid_angle\",0.5716611944444444], \
             PARAMETER[\"scale_factor\",0.999895934], PARAMETER[\"false_easting\",658377.437], \
             PARAMETER[\"false_northing\",3044969.194], UNIT[\"metre\",1, \
             AUTHORITY[\"EPSG\",\"9001\"]], AXIS[\"Easting\",EAST], AXIS[\"Northing\",NORTH], \
             AUTHORITY[\"EPSG\",\"2057\"]]"
                .into(),
        );
        let mut point = Coords::new_xy(658511.261946, 3043003.05468);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(52.60489360062432, 27.501084223177624));

        // INVERSE:
        let mut point = Coords::new_xy(52.60489360062432, 27.501084223177624);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(658511.2619459995, 3043003.054680001));
    }

    #[test]
    fn test_transform_polar_stere() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"WGS 84 / UPS North\",GEOGCS[\"WGS 84\",DATUM[\"WGS_1984\",SPHEROID[\"WGS \
             84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],AUTHORITY[\"EPSG\",\"6326\"\
             ]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.\
             01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4326\"]],UNIT[\"\
             metre\",1,AUTHORITY[\"EPSG\",\"9001\"]],PROJECTION[\"Polar_Stereographic\"],\
             PARAMETER[\"latitude_of_origin\",90],PARAMETER[\"central_meridian\",0],PARAMETER[\"\
             scale_factor\",0.994],PARAMETER[\"false_easting\",2000000],PARAMETER[\"\
             false_northing\",2000000],AUTHORITY[\"EPSG\",\"32661\"],AXIS[\"Easting\",UNKNOWN],\
             AXIS[\"Northing\",UNKNOWN]]"
                .into(),
        );
        let mut point = Coords::new_xy(2000000., 325449.806286);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(0., 74.99999999999869));

        // INVERSE:
        let mut point = Coords::new_xy(0., 74.99999999999869);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(2000000.0, 325449.806285999));
    }

    // TODO: Something isn't right
    // #[test]
    // fn test_transform_polar_stere_north() {
    //     let mut transformer = Transformer::new();
    //     transformer.set_source("PROJCS[\"WGS 84 / UPS North\",GEOGCS[\"GCS_WGS_1984\",DATUM[\"D_WGS_1984\",SPHEROID[\"WGS_1984\",6378137,298.257223563]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.017453292519943295]],PROJECTION[\"Stereographic_North_Pole\"],PARAMETER[\"standard_parallel_1\",90],PARAMETER[\"central_meridian\",0],PARAMETER[\"scale_factor\",0.994],PARAMETER[\"false_easting\",2000000],PARAMETER[\"false_northing\",2000000],UNIT[\"Meter\",1]]".into());
    //     let mut point = Coords::new_xy(2000000., 325449.806286);
    //     transformer.forward_mut(&mut point);
    //     assert_eq!(point, Coords::new_xy(0., 74.99999999999869));
    // }

    #[test]
    fn test_transform_van_der_grinten_i() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"Sphere_Van_der_Grinten_I\",GEOGCS[\"GCS_Sphere\",DATUM[\"D_Sphere\",\
             SPHEROID[\"Sphere\",6371000,0]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.\
             017453292519943295]],PROJECTION[\"Van_der_Grinten_I\"],PARAMETER[\"False_Easting\",\
             0],PARAMETER[\"False_Northing\",0],PARAMETER[\"Central_Meridian\",0],UNIT[\"Meter\",\
             1]]"
            .into(),
        );
        let mut point = Coords::new_xy(-125108.675828, 9016899.042114);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-1.4116080195650045, 67.40891366747749));

        // INVERSE:
        let mut point = Coords::new_xy(-1.4116080195650045, 67.40891366747749);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-125108.6758280449, 9016899.042113967));
    }

    #[test]
    fn test_transform_van_der_grinten_base() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCS[\"Sphere_Van_der_Grinten_I\",GEOGCS[\"GCS_Sphere\",DATUM[\"\
             Not_specified_based_on_Authalic_Sphere\",SPHEROID[\"Sphere\",6371000,0]],PRIMEM[\"\
             Greenwich\",0],UNIT[\"Degree\",0.017453292519943295]],PROJECTION[\"VanDerGrinten\"],\
             PARAMETER[\"False_Easting\",0],PARAMETER[\"False_Northing\",0],PARAMETER[\"\
             Central_Meridian\",0],UNIT[\"Meter\",1],AUTHORITY[\"EPSG\",\"53029\"]]"
                .into(),
        );
        let mut point = Coords::new_xy(-125108.675828, 9016899.042114);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-1.4116080195650045, 67.40891366747749));

        // INVERSE:
        let mut point = Coords::new_xy(-1.4116080195650045, 67.40891366747749);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(-125108.6758280449, 9016899.042113967));
    }

    #[test]
    fn test_oblique_mercator_laborde_tif() {
        let mut transformer = Transformer::new();
        transformer.set_source(
            "PROJCRS[\"Tananarive / Laborde Grid\",BASEGEOGCRS[\"Tananarive\",DATUM[\"Tananarive 1925\",ELLIPSOID[\"International 1924\",6378388,297,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",7022]],ID[\"EPSG\",6297]],ID[\"EPSG\",4297]],CONVERSION[\"Laborde Grid (Greenwich)\",METHOD[\"Laborde Oblique Mercator\",ID[\"EPSG\",9813]],PARAMETER[\"Latitude of projection centre\",-18.9000000000003,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8811]],PARAMETER[\"Longitude of projection centre\",46.4372291666669,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8812]],PARAMETER[\"Azimuth at projection centre\",18.9000000000003,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"EPSG\",8813]],PARAMETER[\"Scale factor at projection centre\",0.9995,SCALEUNIT[\"unity\",1,ID[\"EPSG\",9201]],ID[\"EPSG\",8815]],PARAMETER[\"False easting\",400000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8806]],PARAMETER[\"False northing\",800000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],ID[\"EPSG\",8440]],CS[Cartesian,2,ID[\"EPSG\",4530]],AXIS[\"Northing (X)\",north],AXIS[\"Easting (Y)\",east],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8441]]".into()
        );
        let mut point = Coords::new_xy(440720.0, 3751260.0);
        transformer.forward_mut(&mut point);
        // Should be: X: 46.5138333543 Y: 7.6972635012 (very close)
        assert_eq!(point, Coords::new_xy(46.51427439279896, 7.634598997212268));

        // INVERSE:
        let mut point = Coords::new_xy(46.51427439279896, 7.634598997212268);
        transformer.inverse_mut(&mut point);
        assert_eq!(point, Coords::new_xy(440686.09020763607, 3751216.335492778));
    }

    #[test]
    fn test_should_not_touch() {
        let mut transformer = Transformer::new();
        transformer.set_source("GEOGCS[\"GCS_WGS_1984\",DATUM[\"D_WGS_1984\",SPHEROID[\"WGS_1984\",6378137,298.257223563]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.017453292519943295]]".into());
        let mut point = Coords::new_xy(0.0, 0.0);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(0.0, 0.0));

        let mut point = Coords::new_xy(122.0, -55.0);
        transformer.forward_mut(&mut point);
        assert_eq!(point, Coords::new_xy(122.0, -55.0));
    }

    // #[test]
    // fn test_llc_laz_coverage() {
    //     let mut transformer = Transformer::new();
    //     transformer.set_source("PROJCS[\"NAD_1983_HARN_Lambert_Conformal_Conic\",GEOGCS[\"GCS_North_American_1983_HARN\",DATUM[\"NAD83_High_Accuracy_Regional_Network\",SPHEROID[\"GRS_1980\",6378137,298.257222101,AUTHORITY[\"EPSG\",\"7019\"]],AUTHORITY[\"EPSG\",\"6152\"]],PRIMEM[\"Greenwich\",0],UNIT[\"degree\",0.0174532925199433]],PROJECTION[\"Lambert_Conformal_Conic_2SP\"],PARAMETER[\"standard_parallel_1\",43],PARAMETER[\"standard_parallel_2\",45.5],PARAMETER[\"latitude_of_origin\",41.75],PARAMETER[\"central_meridian\",-120.5],PARAMETER[\"false_easting\",1312335.958005249],PARAMETER[\"false_northing\",0],UNIT[\"foot\",0.3048,AUTHORITY[\"EPSG\",\"9002\"]]]\u{0000}".into());
    //     let mut point = Coords::new_xy(636001.76, 848935.2000000001);
    //     transformer.inverse_mut(&mut point);
    //     assert_eq!(point, Coords::new_xy(120.5, 41.75));
    // }
}

//   {
//     code: 'PROJCS["CH1903 / LV03",GEOGCS["CH1903",DATUM["D_CH1903",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Hotine_Oblique_Mercator_Azimuth_Center"],PARAMETER["latitude_of_center",46.95240555555556],PARAMETER["longitude_of_center",7.439583333333333],PARAMETER["azimuth",90],PARAMETER["scale_factor",1],PARAMETER["false_easting",600000],PARAMETER["false_northing",200000],UNIT["Meter",1]]',
//     xy: [660013.4882918689, 185172.17110117766],
//     ll: [8.225, 46.815],
//     acc: {
//       xy: 0.1,
//       ll: 5,
//     },
//   },
//   {
//     code: 'PROJCS["CH1903 / LV03",GEOGCS["CH1903",DATUM["CH1903",SPHEROID["Bessel 1841",6377397.155,299.1528128,AUTHORITY["EPSG","7004"]],TOWGS84[674.4,15.1,405.3,0,0,0,0],AUTHORITY["EPSG","6149"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4149"]],PROJECTION["Hotine_Oblique_Mercator_Azimuth_Center"],PARAMETER["latitude_of_center",46.95240555555556],PARAMETER["longitude_of_center",7.439583333333333],PARAMETER["azimuth",90],PARAMETER["rectified_grid_angle",90],PARAMETER["scale_factor",1],PARAMETER["false_easting",600000],PARAMETER["false_northing",200000],UNIT["metre",1,AUTHORITY["EPSG","9001"]],AXIS["Y",EAST],AXIS["X",NORTH],AUTHORITY["EPSG","21781"]]',
//     xy: [660013.4882918689, 185172.17110117766],
//     ll: [8.225, 46.815],
//     acc: {
//       xy: 0.1,
//       ll: 5,
//     },
//   },
//   {
//     code: 'PROJCS["NAD83 / Massachusetts Mainland",GEOGCS["NAD83",DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4269"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["standard_parallel_1",42.68333333333333],PARAMETER["standard_parallel_2",41.71666666666667],PARAMETER["latitude_of_origin",41],PARAMETER["central_meridian",-71.5],PARAMETER["false_easting",200000],PARAMETER["false_northing",750000],AUTHORITY["EPSG","26986"],AXIS["X",EAST],AXIS["Y",NORTH]]',
//     xy: [231394.84, 902621.11],
//     ll: [-71.11881762742996, 42.37346263960867],
//   },
//   {
//     code: 'PROJCS["NAD83 / Massachusetts Mainland", GEOGCS["NAD83", DATUM["North American Datum 1983", SPHEROID["GRS 1980", 6378137.0, 298.257222101, AUTHORITY["EPSG","7019"]], TOWGS84[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], AUTHORITY["EPSG","6269"]], PRIMEM["Greenwich", 0.0, AUTHORITY["EPSG","8901"]], UNIT["degree", 0.017453292519943295], AXIS["Geodetic longitude", EAST], AXIS["Geodetic latitude", NORTH], AUTHORITY["EPSG","4269"]], PROJECTION["Lambert_Conformal_Conic_2SP", AUTHORITY["EPSG","9802"]], PARAMETER["central_meridian", -71.5], PARAMETER["latitude_of_origin", 41.0], PARAMETER["standard_parallel_1", 42.68333333333334], PARAMETER["false_easting", 200000.0], PARAMETER["false_northing", 750000.0], PARAMETER["scale_factor", 1.0], PARAMETER["standard_parallel_2", 41.71666666666667], UNIT["m", 1.0], AXIS["Easting", EAST], AXIS["Northing", NORTH], AUTHORITY["EPSG","26986"]]',
//     xy: [231394.84, 902621.11],
//     ll: [-71.11881762742996, 42.37346263960867],
//   },
//   {
//     code: 'PROJCS["Asia_North_Equidistant_Conic",GEOGCS["GCS_WGS_1984",DATUM["WGS_1984",SPHEROID["WGS_1984",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Equidistant_Conic"],PARAMETER["False_Easting",0],PARAMETER["False_Northing",0],PARAMETER["Central_Meridian",95],PARAMETER["Standard_Parallel_1",15],PARAMETER["Standard_Parallel_2",65],PARAMETER["Latitude_Of_Origin",30],UNIT["Meter",1],AUTHORITY["EPSG","102026"]]',
//     xy: [88280.59904432714, 111340.90165417176],
//     ll: [96, 31],
//   },
//   {
//     code: 'PROJCS["World_Sinusoidal",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Sinusoidal"],PARAMETER["False_Easting",0],PARAMETER["False_Northing",0],PARAMETER["Central_Meridian",0],UNIT["Meter",1]]',
//     xy: [738509.49, 5874620.38],
//     ll: [11.0, 53.0],
//   },
//   {
//     code: 'PROJCS["ETRS89 / ETRS-LAEA",GEOGCS["ETRS89",DATUM["European_Terrestrial_Reference_System_1989",SPHEROID["GRS 1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6258"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4258"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Azimuthal_Equal_Area"],PARAMETER["latitude_of_center",52],PARAMETER["longitude_of_center",10],PARAMETER["false_easting",4321000],PARAMETER["false_northing",3210000],AUTHORITY["EPSG","3035"],AXIS["X",EAST],AXIS["Y",NORTH]]',
//     xy: [4388138.6, 3321736.46],
//     ll: [11.0, 53.0],
//   },
//   {
//     code: '+proj=gnom +lat_0=90 +lon_0=0 +x_0=6300000 +y_0=6300000 +ellps=WGS84 +datum=WGS84 +units=m +no_defs',
//     xy: [350577.5930806119, 4705857.070634324],
//     ll: [-75, 46],
//   },
//   {
//     code: '+proj=gnom +lat_0=90 +lon_0=0 +x_0=6300000 +y_0=6300000 +ellps=WGS84 +datum=WGS84 +units=m +no_defs',
//     xy: [350577.5930806119, 4705857.070634324],
//     ll: [-75, 46],
//   },
//   {
//     code: 'PROJCS["NAD83(CSRS) / UTM zone 17N",GEOGCS["NAD83(CSRS)",DATUM["NAD83_Canadian_Spatial_Reference_System",SPHEROID["GRS 1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6140"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4617"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Transverse_Mercator"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",-81],PARAMETER["scale_factor",0.9996],PARAMETER["false_easting",500000],PARAMETER["false_northing",0],AUTHORITY["EPSG","2958"],AXIS["Easting",EAST],AXIS["Northing",NORTH]]',
//     xy: [411461.807497, 4700123.744402],
//     ll: [-82.07666015625, 42.448388671875],
//   },
//   {
//     code: 'PROJCS["NAD27 / UTM zone 14N",GEOGCS["NAD27 Coordinate System",DATUM["D_North American Datum 1927 (NAD27)",SPHEROID["Clarke_1866",6378206.4,294.97869821391]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Extended_Transverse_Mercator"],PARAMETER["false_easting",500000],PARAMETER["false_northing",0],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",-99],PARAMETER["scale_factor",0.9996],UNIT["Meter (m)",1]]',
//     xy: [2026074.9192811155, 12812891.606450122],
//     ll: [51.517955776474096, 61.56941794249017],
//   },
//   {
//     code: 'PROJCS["World_Mollweide",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Mollweide"],PARAMETER["False_Easting",0],PARAMETER["False_Northing",0],PARAMETER["Central_Meridian",0],UNIT["Meter",1]]',
//     xy: [3891383.58309223, 6876758.9933288],
//     ll: [60, 60],
//   },
//   {
//     code: 'PROJCS["Azimuthal_Equidistant",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Azimuthal_Equidistant"],PARAMETER["False_Easting",0],PARAMETER["False_Northing",0],PARAMETER["Central_Meridian",0],PARAMETER["Latitude_Of_Origin",0],UNIT["Meter",1]]',
//     ll: [0, 0],
//     xy: [0, 0],
//   },
//   {
//     code: 'PROJCS["Sphere_Azimuthal_Equidistant",GEOGCS["GCS_Sphere",DATUM["Not_specified_based_on_Authalic_Sphere",SPHEROID["Sphere",6371000,0]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Azimuthal_Equidistant"],PARAMETER["False_Easting",0],PARAMETER["False_Northing",0],PARAMETER["Central_Meridian",0],PARAMETER["Latitude_Of_Origin",0],UNIT["Meter",1]]',
//     ll: [0, 0],
//     xy: [0, 0],
//   },
//   {
//     code: 'PROJCS["North_Pole_Azimuthal_Equidistant",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Azimuthal_Equidistant"],PARAMETER["False_Easting",0],PARAMETER["False_Northing",0],PARAMETER["Central_Meridian",0],PARAMETER["Latitude_Of_Origin",90],UNIT["Meter",1]]',
//     ll: [50.977303830208, 30.915260093747],
//     xy: [5112279.911077, -4143196.76625],
//   },
//   {
//     code: 'PROJCS["Mount Dillon / Tobago Grid",GEOGCS["Mount Dillon",DATUM["D_Mount_Dillon",SPHEROID["Clarke_1858",6378293.645208759,294.2606763692654]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Cassini"],PARAMETER["latitude_of_origin",11.25217861111111],PARAMETER["central_meridian",-60.68600888888889],PARAMETER["false_easting",187500],PARAMETER["false_northing",180000],UNIT["Clarke\'s link",0.201166195164]]',
//     ll: [-60.676753018, 11.2487234308],
//     xy: [192524.3061766178, 178100.2740019509],
//     acc: {
//       ll: 1,
//       xy: -4,
//     },
//   },
// {
//   code:'EPSG:3975',
//   ll:[-9.764450683, 25.751953],
//   xy:[-942135.525095996, 3178441.8667094777]
// },
//   {
//     code: 'PROJCS["World Equidistant Cylindrical (Sphere)",GEOGCS["Unspecified datum based upon the GRS 1980 Authalic Sphere",DATUM["Not_specified_based_on_GRS_1980_Authalic_Sphere",SPHEROID["GRS 1980 Authalic Sphere",6371007,0,AUTHORITY["EPSG","7048"]],AUTHORITY["EPSG","6047"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4047"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Equirectangular"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",0],PARAMETER["false_easting",0],PARAMETER["false_northing",0],AUTHORITY["EPSG","3786"],AXIS["X",EAST],AXIS["Y",NORTH]]',
//     ll: [-1.7539371169976, 12.632997701986],
//     xy: [-195029.12334755991, 1395621.9368162225],
//     acc: {
//       ll: 2,
//     },
//   },
//   {
//     code: 'PROJCS["World Equidistant Cylindrical (Sphere)",GEOGCS["Unspecified datum based upon the GRS 1980 Authalic Sphere",DATUM["D_",SPHEROID["GRS_1980_Authalic_Sphere",6371007,0]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Equidistant_Cylindrical"],PARAMETER["central_meridian",0],PARAMETER["false_easting",0],PARAMETER["false_northing",0],UNIT["Meter",1]]',
//     ll: [-1.7539371169976, 12.632997701986],
//     xy: [-195029.12334755991, 1395621.9368162225],
//     acc: {
//       ll: 2,
//     },
//   },
//   {
//     code: 'PROJCS["Segara / NEIEZ",GEOGCS["Segara",DATUM["Gunung_Segara",SPHEROID["Bessel 1841",6377397.155,299.1528128,AUTHORITY["EPSG","7004"]],AUTHORITY["EPSG","6613"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4613"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Mercator_1SP"],PARAMETER["central_meridian",110],PARAMETER["scale_factor",0.997],PARAMETER["false_easting",3900000],PARAMETER["false_northing",900000],AUTHORITY["EPSG","3000"],AXIS["X",EAST],AXIS["Y",NORTH]]',
//     ll: [116.65547897884308, -0.6595605286983485],
//     xy: [4638523.040740433, 827245.2586932715],
//   },
//   {
//     code: 'PROJCS["Segara / NEIEZ",GEOGCS["Segara",DATUM["D_Gunung_Segara",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Mercator"],PARAMETER["central_meridian",110],PARAMETER["scale_factor",0.997],PARAMETER["false_easting",3900000],PARAMETER["false_northing",900000],UNIT["Meter",1]]',
//     ll: [116.65547897884308, -0.6595605286983485],
//     xy: [4638523.040740433, 827245.2586932715],
//   },
//   {
//     code: 'PROJCS["Beduaram / TM 13 NE",GEOGCS["Beduaram",DATUM["Beduaram",SPHEROID["Clarke 1880 (IGN)",6378249.2,293.4660212936269,AUTHORITY["EPSG","7011"]],TOWGS84[-106,-87,188,0,0,0,0],AUTHORITY["EPSG","6213"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4213"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Transverse_Mercator"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",13],PARAMETER["scale_factor",0.9996],PARAMETER["false_easting",500000],PARAMETER["false_northing",0],AUTHORITY["EPSG","2931"],AXIS["X",EAST],AXIS["Y",NORTH]]',
//     ll: [5, 25],
//     xy: [-308919.1234711099, 2788738.255936392],
//   },
//   {
//     code: 'PROJCS["Beduaram / TM 13 NE",GEOGCS["Beduaram",DATUM["D_Beduaram",SPHEROID["Clarke_1880_IGN",6378249.2,293.4660212936269]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Transverse_Mercator"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",13],PARAMETER["scale_factor",0.9996],PARAMETER["false_easting",500000],PARAMETER["false_northing",0],UNIT["Meter",1]]',
//     ll: [5, 25],
//     xy: [-308919.1234711099, 2788738.255936392],
//   },
//   {
//     code: '+proj=lcc +lat_1=49.5 +lat_0=49.5 +lon_0=0 +k_0=0.999877341 +x_0=600000 +y_0=1200000 +ellps=clrk80ign +pm=paris +towgs84=-168,-60,320,0,0,0,0 +units=m +no_defs +type=crs',
//     ll: [2.294482, 48.859045],
//     xy: [596916.561147926957, 1128733.073948238511],
//   },
//   {
//     code: 'PROJCS["S-JTSK (Ferro) / Krovak",GEOGCS["S-JTSK (Ferro)",DATUM["D_S_JTSK",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Ferro",-17.66666666666667],UNIT["Degree",0.017453292519943295]],PROJECTION["Krovak"],PARAMETER["latitude_of_center",49.5],PARAMETER["longitude_of_center",42.5],PARAMETER["azimuth",30.28813972222222],PARAMETER["pseudo_standard_parallel_1",78.5],PARAMETER["scale_factor",0.9999],PARAMETER["false_easting",0],PARAMETER["false_northing",0],UNIT["Meter",1]]',
//     ll: [17.323583231075897, 49.39440725405376],
//     xy: [-544115.474379, -1144058.330762],
//   },
//   {
//     code: 'PROJCS["Sphere_Miller_Cylindrical",GEOGCS["GCS_Sphere",DATUM["Not_specified_based_on_Authalic_Sphere",SPHEROID["Sphere",6371000,0]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Miller_Cylindrical"],PARAMETER["False_Easting",0],PARAMETER["False_Northing",0],PARAMETER["Central_Meridian",0],UNIT["Meter",1],AUTHORITY["EPSG","53003"]]',
//     ll: [-1.3973289073953, 12.649176474268513],
//     xy: [-155375.88535614178, 1404635.2633403721],
//     acc: {
//       ll: 3,
//     },
//   },
//   {
//     code: 'PROJCS["NZGD49 / New Zealand Map Grid",GEOGCS["NZGD49",DATUM["New_Zealand_Geodetic_Datum_1949",SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],TOWGS84[59.47,-5.04,187.44,0.47,-0.1,1.024,-4.5993],AUTHORITY["EPSG","6272"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4272"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["New_Zealand_Map_Grid"],PARAMETER["latitude_of_origin",-41],PARAMETER["central_meridian",173],PARAMETER["false_easting",2510000],PARAMETER["false_northing",6023150],AUTHORITY["EPSG","27200"],AXIS["Easting",EAST],AXIS["Northing",NORTH]]',
//     ll: [172.465, -40.7],
//     xy: [2464770.343667, 6056137.861919],
//   },
//   {
//     code: 'PROJCS["SAD69 / Brazil Polyconic",GEOGCS["SAD69",DATUM["D_South_American_1969",SPHEROID["GRS_1967_SAD69",6378160,298.25]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Polyconic"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",-54],PARAMETER["false_easting",5000000],PARAMETER["false_northing",10000000],UNIT["Meter",1]]',
//     ll: [-49.221772553812, -0.34551739237581],
//     xy: [5531902.134932, 9961660.779347],
//     acc: {
//       ll: 3,
//       xy: -2,
//     },
//   },
//   {
//     code: 'PROJCS["SAD69 / Brazil Polyconic",GEOGCS["SAD69",DATUM["South_American_Datum_1969",SPHEROID["GRS 1967 (SAD69)",6378160,298.25,AUTHORITY["EPSG","7050"]],AUTHORITY["EPSG","6618"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4618"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Polyconic"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",-54],PARAMETER["false_easting",5000000],PARAMETER["false_northing",10000000],AUTHORITY["EPSG","29101"],AXIS["X",EAST],AXIS["Y",NORTH]]',
//     ll: [-49.221772553812, -0.34551739237581],
//     xy: [5531902.134932, 9961660.779347],
//     acc: {
//       ll: 3,
//       xy: -2,
//     },
//   },

//   {
//     code: '+proj=aeqd +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +datum=WGS84 +units=m +no_defs',
//     ll: [2, 0],
//     xy: [222638.98158654713, 0],
//   },
//   {
//     code: '+proj=aeqd +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +datum=WGS84 +units=m +no_defs',
//     ll: [89, 0],
//     xy: [9907434.680601358, 0],
//   },
//   {
//     //    code:'+proj=aeqd +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +datum=WGS84 +units=m +no_defs',
//     //    ll:[91, 0],
//     //    xy:[10130073.6622, 0]
//     //  },{
//     code: '+proj=aeqd +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +a=6371000 +b=6371000 +units=m +no_defs',
//     ll: [91, 0],
//     xy: [10118738.32, 0.0],
//   },
//   {
//     code: '+proj=laea +lat_0=2 +lon_0=1 +x_0=0 +y_0=0 +a=6371000 +b=6371000  +units=m +no_defs',
//     ll: [1, 2],
//     xy: [0, 0],
//   },
//   {
//     code: '+proj=laea +lat_0=1 +lon_0=1 +x_0=0 +y_0=0 +a=6371000 +b=6371000  +units=m +no_defs',
//     ll: [1, 1],
//     xy: [0, 0],
//   },
//   {
//     code: '+proj=laea +lat_0=1 +lon_0=1 +x_0=0 +y_0=0 +a=6371000 +b=6371000  +units=m +no_defs',
//     ll: [2, 1],
//     xy: [111176.58, 16.93],
//   },
//   {
//     code: '+proj=laea +lat_0=1 +lon_0=1 +x_0=0 +y_0=0 +a=6371000 +b=6371000  +units=m +no_defs',
//     ll: [1, 2],
//     xy: [0.0, 111193.52],
//   },
//   {
//     code: '+proj=laea +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +a=6371000 +b=6371000 +units=m +no_defs',
//     ll: [19, 0],
//     xy: [2103036.59, 0.0],
//   },
//   {
//     code: '+proj=stere +lat_0=-90 +lat_ts=-70 +lon_0=0 +k=1 +x_0=0 +y_0=0 +datum=WGS84 +units=m +no_defs"',
//     ll: [0, -72.5],
//     xy: [0, 1910008.78441421],
//   },
//   {
//     code: '+proj=stere +lat_0=-90 +lon_0=0 +x_0=0 +y_0=0 +a=3396000 +b=3396000 +units=m +no_defs',
//     ll: [0, -72.5],
//     xy: [0, 1045388.79],
//   },
//   {
//     code: '+proj=stere',
//     ll: [0, -72.5],
//     xy: [0, -9334375.897187851],
//   },
//   {
//     // Test that lat_ts at a pole is handled correctly in stere projection
//     code: '+no_defs +units=m +ellps=GRS80 +lon_0=0 +proj=stere +lat_ts=90.0 +lat_0=90 +x_0=0 +y_0=0',
//     ll: [69.6487, 18.955781],
//     xy: [8527917.706, -3163255.729],
//   },
//   {
//     code: 'PROJCS["WGS 84 / NSIDC Sea Ice Polar Stereographic South", GEOGCS["WGS 84", DATUM["World Geodetic System 1984", SPHEROID["WGS 84", 6378137.0, 298.257223563, AUTHORITY["EPSG","7030"]], AUTHORITY["EPSG","6326"]], PRIMEM["Greenwich", 0.0, AUTHORITY["EPSG","8901"]], UNIT["degree", 0.017453292519943295], AXIS["Geodetic longitude", EAST], AXIS["Geodetic latitude", NORTH], AUTHORITY["EPSG","4326"]], PROJECTION["Polar Stereographic (variant B)", AUTHORITY["EPSG","9829"]], PARAMETER["central_meridian", 0.0], PARAMETER["Standard_Parallel_1", -70.0], PARAMETER["false_easting", 0.0], PARAMETER["false_northing", 0.0], UNIT["m", 1.0], AXIS["Easting", "North along 90 deg East"], AXIS["Northing", "North along 0 deg"], AUTHORITY["EPSG","3976"]]',
//     ll: [0, -72.5],
//     xy: [0, 1910008.78441421],
//   },
//   {
//     code: 'PROJCS["NAD83(CSRS98) / New Brunswick Stereo (deprecated)",GEOGCS["NAD83(CSRS98)",DATUM["D_North_American_1983_CSRS98",SPHEROID["GRS_1980",6378137,298.257222101]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Stereographic_North_Pole"],PARAMETER["standard_parallel_1",46.5],PARAMETER["central_meridian",-66.5],PARAMETER["scale_factor",0.999912],PARAMETER["false_easting",2500000],PARAMETER["false_northing",7500000],UNIT["Meter",1]]',
//     ll: [-66.415, 46.34],
//     xy: [2506543.370459, 7482219.546176],
//   },
//   {
//     code: 'PROJCS["NAD83(CSRS98) / New Brunswick Stereo (deprecated)",GEOGCS["NAD83(CSRS98)",DATUM["NAD83_Canadian_Spatial_Reference_System",SPHEROID["GRS 1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],TOWGS84[0,0,0,0,0,0,0],AUTHORITY["EPSG","6140"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9108"]],AUTHORITY["EPSG","4140"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Oblique_Stereographic"],PARAMETER["latitude_of_origin",46.5],PARAMETER["central_meridian",-66.5],PARAMETER["scale_factor",0.999912],PARAMETER["false_easting",2500000],PARAMETER["false_northing",7500000],AUTHORITY["EPSG","2036"],AXIS["Easting",EAST],AXIS["Northing",NORTH]]',
//     ll: [-66.415, 46.34],
//     xy: [2506543.370459, 7482219.546176],
//   },

//   {
//     code: 'PROJCS["NAD_1983_StatePlane_New_Jersey_FIPS_2900_Feet",GEOGCS["GCS_North_American_1983",DATUM["D_North_American_1983",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Transverse_Mercator"],PARAMETER["False_Easting",492125.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",-74.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Latitude_Of_Origin",38.83333333333334],UNIT["Foot_US",0.3048006096012192]]',
//     ll: [-74, 41],
//     xy: [630128.205, 789591.522],
//   },
//   {
//     code: 'PROJCS["WGS_1984_Web_Mercator_Auxiliary_Sphere",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Mercator_Auxiliary_Sphere"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",0.0],PARAMETER["Standard_Parallel_1",0.0],PARAMETER["Auxiliary_Sphere_Type",0.0],UNIT["Meter",1.0]]',
//     ll: [-74, 41],
//     xy: [-8237642.318702244, 5012341.663847514],
//   },
//   {
//     code: '+proj=sinu +lon_0=0 +x_0=0 +y_0=0 +a=6371000 +b=6371000 +units=m +no_defs',
//     xy: [736106.55, 5893331.11],
//     ll: [11.0, 53.0],
//   },
//   {
//     code: 'PROJCS["Belge 1972 / Belgian Lambert 72",GEOGCS["Belge 1972",DATUM["Reseau_National_Belge_1972",SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],TOWGS84[106.869,-52.2978,103.724,-0.33657,0.456955,-1.84218,1],AUTHORITY["EPSG","6313"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4313"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["standard_parallel_1",51.16666723333333],PARAMETER["standard_parallel_2",49.8333339],PARAMETER["latitude_of_origin",90],PARAMETER["central_meridian",4.367486666666666],PARAMETER["false_easting",150000.013],PARAMETER["false_northing",5400088.438],AUTHORITY["EPSG","31370"],AXIS["X",EAST],AXIS["Y",NORTH]]',
//     xy: [104588.196404, 193175.582367],
//     ll: [3.7186701465384533, 51.04642936832842],
//   },
//   {
//     code: 'PROJCS["Belge 1972 / Belgian Lambert 72",GEOGCS["Belge 1972",DATUM["D_Belge_1972",SPHEROID["International_1924",6378388,297]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Lambert_Conformal_Conic"],PARAMETER["standard_parallel_1",51.16666723333333],PARAMETER["standard_parallel_2",49.8333339],PARAMETER["latitude_of_origin",90],PARAMETER["central_meridian",4.367486666666666],PARAMETER["false_easting",150000.013],PARAMETER["false_northing",5400088.438],UNIT["Meter",1]]',
//     xy: [104588.196404, 193175.582367],
//     ll: [3.7186701465384533, 51.04642936832842],
//   },
//   {
//     code: 'PROJCS["Belge 1972 / Belgian Lambert 72",GEOGCS["Belge 1972",DATUM["Reseau_National_Belge_1972",SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],TOWGS84[-106.8686,52.2978,-103.7239,-0.3366,0.457,-1.8422,-1.2747],AUTHORITY["EPSG","6313"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4313"]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["standard_parallel_1",51.16666723333333],PARAMETER["standard_parallel_2",49.8333339],PARAMETER["latitude_of_origin",90],PARAMETER["central_meridian",4.367486666666666],PARAMETER["false_easting",150000.013],PARAMETER["false_northing",5400088.438],UNIT["metre",1,AUTHORITY["EPSG","9001"]],AXIS["X",EAST],AXIS["Y",NORTH],AUTHORITY["EPSG","31370"]]',
//     xy: [104469.69796438649, 193146.39675426576],
//     ll: [3.7186701465384533, 51.04642936832842],
//   },
//   {
//     code: 'PROJCS["Belge 1972 / Belgian Lambert 72",GEOGCS["Belge 1972",DATUM["Reseau_National_Belge_1972",SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],TOWGS84[-99.059,53.322,-112.486,-0.419,0.83,-1.885,-1],AUTHORITY["EPSG","6313"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4313"]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["standard_parallel_1",51.16666723333333],PARAMETER["standard_parallel_2",49.8333339],PARAMETER["latitude_of_origin",90],PARAMETER["central_meridian",4.367486666666666],PARAMETER["false_easting",150000.013],PARAMETER["false_northing",5400088.438],UNIT["metre",1,AUTHORITY["EPSG","9001"]],AXIS["X",EAST],AXIS["Y",NORTH],AUTHORITY["EPSG","31370"]]',
//     xy: [104468.8305227503, 193169.6828284394],
//     ll: [3.7186701465384533, 51.04642936832842],
//   },
//   {
//     code: 'PROJCS["Belge 1972 / Belgian Lambert 72",GEOGCS["Belge 1972",DATUM["Reseau_National_Belge_1972",SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],TOWGS84[-125.8,79.9,-100.5,0,0,0,0],AUTHORITY["EPSG","6313"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4313"]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["standard_parallel_1",51.16666723333333],PARAMETER["standard_parallel_2",49.8333339],PARAMETER["latitude_of_origin",90],PARAMETER["central_meridian",4.367486666666666],PARAMETER["false_easting",150000.013],PARAMETER["false_northing",5400088.438],UNIT["metre",1,AUTHORITY["EPSG","9001"]],AXIS["X",EAST],AXIS["Y",NORTH],AUTHORITY["EPSG","31370"]]',
//     xy: [104412.1099068548, 193116.8535417635],
//     ll: [3.7186701465384533, 51.04642936832842],
//   },
//   {
//     code: '+proj=lcc +lat_1=51.16666723333333 +lat_2=49.8333339 +lat_0=90 +lon_0=4.367486666666666 +x_0=150000.013 +y_0=5400088.438 +ellps=intl +towgs84=106.869,-52.2978,103.724,-0.33657,0.456955,-1.84218,1 +units=m +no_defs ',
//     xy: [104588.196404, 193175.582367],
//     ll: [3.7186701465384533, 51.04642936832842],
//   },
//   {
//     code: 'PROJCS["JAD2001 / Jamaica Metric Grid",GEOGCS["JAD2001",DATUM["Jamaica_2001",SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],TOWGS84[0,0,0,0,0,0,0],AUTHORITY["EPSG","6758"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4758"]],PROJECTION["Lambert_Conformal_Conic_1SP"],PARAMETER["latitude_of_origin",18],PARAMETER["central_meridian",-77],PARAMETER["scale_factor",1],PARAMETER["false_easting",750000],PARAMETER["false_northing",650000],UNIT["metre",1,AUTHORITY["EPSG","9001"]],AXIS["Easting",EAST],AXIS["Northing",NORTH],AUTHORITY["EPSG","3448"]]',
//     xy: [7578825.28673236, 11374595.814939449],
//     ll: [44.2312, 76.486],
//   },
//   {
//     code: '+proj=etmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy +datum=OSGB36 +units=m +no_defs',
//     ll: [-3.20078, 55.96056],
//     xy: [325132.0089586496, 674822.638235305],
//   },
//   {
//     code: '+proj=krovak +lat_0=49.5 +lon_0=24.83333333333333 +alpha=30.28813972222222 +k=0.9999 +x_0=0 +y_0=0 +ellps=bessel +pm=greenwich +units=m +no_defs +towgs84=570.8,85.7,462.8,4.998,1.587,5.261,3.56',
//     ll: [12.806988, 49.452262],
//     xy: [-868208.61, -1095793.64],
//   },
//   {
//     code: '+proj=etmerc +lat_0=40.5 +lon_0=-110.0833333333333 +k=0.9999375 +x_0=800000.0000101599 +y_0=99999.99998983997 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=us-ft +no_defs',
//     ll: [-110.8, 43.5],
//     xy: [2434515.87, 1422072.711],
//   },
//   // QSC WGS84
//   {
//     code: '+proj=qsc +lat_0=0 +lon_0=0 +units=m +datum=WGS84',
//     ll: [2, 1],
//     xy: [304638.4508447283296846, 164123.8709293559950311],
//   },
//   {
//     code: '+proj=qsc +lat_0=0 +lon_0=90 +units=m +datum=WGS84',
//     ll: [2, 1],
//     xy: [-11576764.4717786349356174, 224687.8649776891397778],
//   },
//   {
//     code: '+proj=qsc +lat_0=0 +lon_0=180 +units=m +datum=WGS84',
//     ll: [2, 1],
//     xy: [-15631296.4526007361710072, 8421356.1168374437838793],
//   },
//   {
//     code: '+proj=qsc +lat_0=0 +lon_0=-90 +units=m +datum=WGS84',
//     ll: [2, 1],
//     xy: [11988027.598701536655426, 232669.8736086514254566],
//   },
//   {
//     code: '+proj=qsc +lat_0=90 +lon_0=0 +units=m +datum=WGS84',
//     ll: [2, 1],
//     xy: [456180.4073964518611319, -11678366.591438926756382],
//   },
//   {
//     code: '+proj=qsc +lat_0=-90 +lon_0=0 +units=m +datum=WGS84',
//     ll: [2, 1],
//     xy: [464158.3228444084525108, 11882603.8180405404418707],
//   },
//   // QSC WGS84 WKT
//   {
//     code: 'PROJCS["unnamed",GEOGCS["WGS 84",DATUM["unknown",SPHEROID["WGS84",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]],PROJECTION["Quadrilateralized_Spherical_Cube"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",0],UNIT["Meter",1]]',
//     ll: [2, 1],
//     xy: [304638.4508447283296846, 164123.8709293559950311],
//   },
//   {
//     code: 'PROJCS["unnamed",GEOGCS["WGS 84",DATUM["unknown",SPHEROID["WGS84",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]],PROJECTION["Quadrilateralized_Spherical_Cube"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",90],UNIT["Meter",1]]',
//     ll: [2, 1],
//     xy: [-11576764.4717786349356174, 224687.8649776891397778],
//   },
//   {
//     code: 'PROJCS["unnamed",GEOGCS["WGS 84",DATUM["unknown",SPHEROID["WGS84",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]],PROJECTION["Quadrilateralized_Spherical_Cube"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",180],UNIT["Meter",1]]',
//     ll: [2, 1],
//     xy: [-15631296.4526007361710072, 8421356.1168374437838793],
//   },
//   {
//     code: 'PROJCS["unnamed",GEOGCS["WGS 84",DATUM["unknown",SPHEROID["WGS84",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]],PROJECTION["Quadrilateralized_Spherical_Cube"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",-90],UNIT["Meter",1]]',
//     ll: [2, 1],
//     xy: [11988027.598701536655426, 232669.8736086514254566],
//   },
//   {
//     code: 'PROJCS["unnamed",GEOGCS["WGS 84",DATUM["unknown",SPHEROID["WGS84",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]],PROJECTION["Quadrilateralized_Spherical_Cube"],PARAMETER["latitude_of_origin",90],PARAMETER["central_meridian",0],UNIT["Meter",1]]',
//     ll: [2, 1],
//     xy: [456180.4073964518611319, -11678366.591438926756382],
//   },
//   {
//     code: 'PROJCS["unnamed",GEOGCS["WGS 84",DATUM["unknown",SPHEROID["WGS84",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]],PROJECTION["Quadrilateralized_Spherical_Cube"],PARAMETER["latitude_of_origin",-90],PARAMETER["central_meridian",0],UNIT["Meter",1]]',
//     ll: [2, 1],
//     xy: [464158.3228444084525108, 11882603.8180405404418707],
//   },
//   // QSC Mars
//   {
//     code: '+proj=qsc +units=m +a=3396190 +b=3376200 +lat_0=0 +lon_0=0',
//     ll: [2, 1],
//     xy: [162139.9347801624389831, 86935.6184961361577734],
//   },
//   {
//     code: '+proj=qsc +units=m +a=3396190 +b=3376200 +lat_0=0 +lon_0=90',
//     ll: [2, 1],
//     xy: [-6164327.7345527401193976, 119033.1141843862715177],
//   },
//   {
//     code: '+proj=qsc +units=m +a=3396190 +b=3376200 +lat_0=0 +lon_0=180',
//     ll: [2, 1],
//     xy: [-8327904.7183852149173617, 4465226.5862284321337938],
//   },
//   {
//     code: '+proj=qsc +units=m +a=3396190 +b=3376200 +lat_0=0 +lon_0=-90',
//     ll: [2, 1],
//     xy: [6383315.0547841880470514, 123261.7574065744993277],
//   },
//   {
//     code: '+proj=qsc +units=m +a=3396190 +b=3376200 +lat_0=90 +lon_0=0',
//     ll: [2, 1],
//     xy: [242914.9289354820502922, -6218701.0766915259882808],
//   },
//   {
//     code: '+proj=qsc +units=m +a=3396190 +b=3376200 +lat_0=-90 +lon_0=0',
//     ll: [2, 1],
//     xy: [247141.3965058987669181, 6326900.0192015860229731],
//   },
//   // Robinson
//   {
//     code: '+proj=robin +lon_0=0 +x_0=0 +y_0=0 +datum=WGS84 +units=m +no_defs',
//     ll: [-15, -35],
//     xy: [-1335949.91, -3743319.07],
//     acc: { ll: 4, xy: 0 },
//   },
//   {
//     code: '+proj=robin +lon_0=0 +x_0=0 +y_0=0 +datum=WGS84 +units=m +no_defs',
//     ll: [-10, 50],
//     xy: [-819964.6, 5326895.52],
//     acc: { ll: 4, xy: 0 },
//   },
//   {
//     code: '+proj=robin +a=6400000',
//     ll: [80, -20],
//     xy: [7449059.8, -2146370.56],
//     acc: { ll: 4, xy: 0 },
//   },
//   {
//     code: '+proj=robin +lon_0=15 +x_0=100000 +y_0=100000 +datum=WGS84',
//     ll: [-35, 40],
//     xy: [-4253493.26, 4376351.58],
//     acc: { ll: 4, xy: 0 },
//   },
//   {
//     code: 'PROJCS["World_Robinson",GEOGCS["GCS_WGS_1984",DATUM["WGS_1984",SPHEROID["WGS_1984",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Robinson"],PARAMETER["False_Easting",0],PARAMETER["False_Northing",0],PARAMETER["Central_Meridian",0],UNIT["Meter",1]]',
//     ll: [20, 40],
//     xy: [1741397.3, 4276351.58],
//     acc: { ll: 4, xy: 0 },
//   },
//   {
//     code: 'PROJCS["World_Robinson",GEOGCS["GCS_WGS_1984",DATUM["WGS_1984",SPHEROID["WGS_1984",6378137,298.257223563]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Robinson"],PARAMETER["False_Easting",100000],PARAMETER["False_Northing",100000],PARAMETER["Central_Meridian",15],UNIT["Meter",1]]',
//     ll: [-35, 40],
//     xy: [-4253493.26, 4376351.58],
//     acc: { ll: 4, xy: 0 },
//   },
//   {
//     code: '+proj=robin +lon_0=162 +x_0=0 +y_0=0 +ellps=WGS84 +datum=WGS84 +units=m +no_defs',
//     ll: [-90, 22],
//     xy: [9987057.08, 2352946.55],
//     acc: { ll: 4, xy: 0 },
//   },
//   // check that coordinates at 180 and -180 deg. longitude don't wrap around
//   {
//     code: '+title=WGS 84 / Pseudo-Mercator +proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +k=1.0 +units=m +nadgrids=@null +no_defs',
//     ll: [-180, 0],
//     xy: [-20037508.342789, 0],
//   },
//   {
//     code: '+title=WGS 84 / Pseudo-Mercator +proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +k=1.0 +units=m +nadgrids=@null +no_defs',
//     ll: [180, 0],
//     xy: [20037508.342789, 0],
//   },
//   // these test cases are taken from mapshaper-proj and the test results match
//   {
//     code: '+proj=etmerc +ellps=GRS80 +lat_1=0.5 +lat_2=2 +n=0.5',
//     ll: [2, 1],
//     xy: [222650.79679577847, 110642.2294119271],
//   },
//   {
//     code: '+proj=etmerc +approx +a=6400000 +lat_1=0.5 +lat_2=2 +n=0.5',
//     ll: [2, 1],
//     xy: [223413.46640632232, 111769.14504059685],
//   },
//   {
//     code: '+proj=etmerc +zone=30 +ellps=GRS80 +lat_1=0.5 +lat_2=2 +n=0.5',
//     ll: [2, 1],
//     xy: [222650.7967975856, 110642.2294119332],
//   },
//   {
//     code: '+proj=etmerc +k=0.998 +lon_0=-20 +datum=WGS84 +x_0=10000 +y_0=20000',
//     ll: [2, 1],
//     xy: [2516532.477709202, 139083.35793371277],
//   },
//   {
//     code: '+proj=utm +zone=30 +ellps=GRS80 +lat_1=0.5 +lat_2=2 +n=0.5',
//     ll: [2, 1],
//     xy: [1057002.405491298, 110955.14117594929],
//   },
//   {
//     code: '+proj=utm +lon_0=-3 +ellps=GRS80 +lat_1=0.5 +lat_2=2 +n=0.5',
//     ll: [2, 1],
//     xy: [1057002.4052152266, 110955.14117382761],
//   },
//   // these test cases are related to the original issue on GitHub
//   {
//     code: '+proj=utm +zone=33 +datum=WGS84 +units=m +no_defs',
//     ll: [2, 1],
//     xy: [-959006.4926646841, 113457.31956265299],
//   },
//   {
//     code: '+proj=utm +zone=33 +datum=WGS84 +units=m +no_defs',
//     ll: [31, 70],
//     xy: [1104629.4356366363, 7845845.077685604],
//   },
//   // these test cases are for Norway snow flake zones
//   {
//     code: '+proj=utm +zone=31 +datum=WGS84 +units=m +no_defs',
//     ll: [59.121778, 1.508527],
//     xy: [8089746.634775677, 301230.8618526573],
//   },
//   {
//     code: '+proj=utm +zone=32 +datum=WGS84 +units=m +no_defs',
//     ll: [59.121778, 1.508527],
//     xy: [6969865.865375574, 261237.08330733588],
//   },
//   {
//     code: '+proj=utm +zone=33 +datum=WGS84 +units=m +no_defs',
//     ll: [59.121778, 1.508527],
//     xy: [5984417.050333044, 232959.75386279594],
//   },
//   {
//     code: '+proj=utm +zone=34 +datum=WGS84 +units=m +no_defs',
//     ll: [79.070672, 20.520579],
//     xy: [7421462.108989433, 3922366.25143021],
//   },
//   {
//     code: '+proj=utm +zone=35 +datum=WGS84 +units=m +no_defs',
//     ll: [79.070672, 20.520579],
//     xy: [6548241.281523044, 3478520.1422119136],
//   },
//   // these test cases are for the margin zones 1 and 60
//   {
//     code: '+proj=utm +zone=1 +datum=WGS84 +units=m +no_defs',
//     ll: [-177, 60],
//     xy: [500000, 6651411.190362714],
//   },
//   {
//     code: '+proj=utm +zone=60 +datum=WGS84 +units=m +no_defs',
//     ll: [177, 60],
//     xy: [500000.0000000014, 6651411.190362714],
//   },
//   {
//     code: '+proj=lcc +lat_1=46.8 +lat_0=46.8 +lon_0=0 +k_0=0.99987742 +x_0=600000 +y_0=2200000 +a=6378249.2 +b=6356515 +towgs84=-168,-60,320,0,0,0,0 +pm=paris +units=m +no_defs',
//     ll: [1.4477496, 46.8692953],
//     xy: [532247.2835623875, 2208091.8723],
//   },
//   {
//     code: '+proj=utm +zone=33 +units=m +no_defs',
//     ll: [2, 1],
//     xy: [-959006.4926646841, 113457.31956265299],
//   },
//   {
//     code: '+proj=utm +zone=33 +units=m',
//     ll: [2, 1],
//     xy: [-959006.4926646841, 113457.31956265299],
//   },
//   {
//     code: '+proj=utm +zone=33',
//     ll: [2, 1],
//     xy: [-959006.4926646841, 113457.31956265299],
//   },
//   {
//     code: 'PROJCS["CUSTOM_OBLIQUE_MERCATOR", GEOGCS["WGS 84", DATUM["World Geodetic System 1984", SPHEROID["WGS 84", 6378137.0, 298.257223563]], PRIMEM["Greenwich", 0.0], UNIT["degree", 0.017453292519943295], AXIS["Geodetic latitude", NORTH], AXIS["Geodetic longitude", EAST]], PROJECTION["Hotine_Oblique_Mercator_Azimuth_Center", AUTHORITY["EPSG", "9815"]], PARAMETER["latitude_of_center", 37.50832038], PARAMETER["longitude_of_center", -122.25064809], PARAMETER["azimuth", 45.0], PARAMETER["rectified_grid_angle", -3.99], PARAMETER["scale_factor", 1.0], PARAMETER["false_easting", -361.25], PARAMETER["false_northing", 254.915], UNIT["foot", 0.3048], AXIS["Easting", EAST], AXIS["Northing", NORTH]]',
//     xy: [-361.2499999983702, 254.91500000283122],
//     ll: [-122.25064809, 37.50832038],
//     acc: {
//       ll: 3,
//       xy: 8,
//     },
//   },
//   // Omerc Type A - #273
//   {
//     code: '+proj=omerc +lat_0=4 +lonc=102.25 +alpha=323.0257964666666 +k=0.99984 +x_0=804671 +y_0=0 +no_uoff +gamma=323.1301023611111 +ellps=GRS80 +units=m +no_defs',
//     xy: [412597.532715, 338944.957259],
//     ll: [101.70979078430528, 3.06268465621428],
//     acc: {
//       ll: 2,
//       xy: -3,
//     },
//   },
//   {
//     code: 'PROJCS["GDM2000 / Peninsula RSO", GEOGCS["GDM2000", DATUM["Geodetic_Datum_of_Malaysia_2000", SPHEROID["GRS 1980",6378137,298.257222101, AUTHORITY["EPSG","7019"]], AUTHORITY["EPSG","6742"]], PRIMEM["Greenwich",0, AUTHORITY["EPSG","8901"]], UNIT["degree",0.0174532925199433, AUTHORITY["EPSG","9122"]], AUTHORITY["EPSG","4742"]], PROJECTION["Hotine_Oblique_Mercator"], PARAMETER["latitude_of_center",4], PARAMETER["longitude_of_center",102.25], PARAMETER["azimuth",323.0257964666666], PARAMETER["rectified_grid_angle",323.1301023611111], PARAMETER["scale_factor",0.99984], PARAMETER["false_easting",804671], PARAMETER["false_northing",0], UNIT["metre",1, AUTHORITY["EPSG","9001"]], AXIS["Easting",EAST], AXIS["Northing",NORTH], AUTHORITY["EPSG","3375"]]',
//     xy: [412597.532715, 338944.957259],
//     ll: [101.70979078430528, 3.06268465621428],
//     acc: {
//       ll: 7,
//       xy: 6,
//     },
//   },
//   // EPSG:3468
//   {
//     code: '+proj=omerc +lat_0=57 +lonc=-133.6666666666667 +alpha=323.1301023611111 +k=0.9999 +x_0=5000000 +y_0=-5000000 +no_uoff +gamma=323.1301023611111 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs',
//     xy: [1264314.74, -763162.04],
//     ll: [-128.115000029, 44.8150000066],
//     acc: {
//       ll: 9,
//       xy: 4,
//     },
//   },
//   {
//     code: 'PROJCS["NAD83(NSRS2007) / Alaska zone 1", GEOGCS["NAD83(NSRS2007)", DATUM["NAD83_National_Spatial_Reference_System_2007", SPHEROID["GRS 1980",6378137,298.257222101, AUTHORITY["EPSG","7019"]], TOWGS84[0,0,0,0,0,0,0], AUTHORITY["EPSG","6759"]], PRIMEM["Greenwich",0, AUTHORITY["EPSG","8901"]], UNIT["degree",0.0174532925199433, AUTHORITY["EPSG","9122"]], AUTHORITY["EPSG","4759"]], PROJECTION["Hotine_Oblique_Mercator"], PARAMETER["latitude_of_center",57], PARAMETER["longitude_of_center",-133.6666666666667], PARAMETER["azimuth",323.1301023611111], PARAMETER["rectified_grid_angle",323.1301023611111], PARAMETER["scale_factor",0.9999], PARAMETER["false_easting",5000000], PARAMETER["false_northing",-5000000], UNIT["metre",1, AUTHORITY["EPSG","9001"]], AXIS["X",EAST], AXIS["Y",NORTH], AUTHORITY["EPSG","3468"]]',
//     xy: [1264314.74, -763162.04],
//     ll: [-128.115000029, 44.8150000066],
//     acc: {
//       ll: 9,
//       xy: 4,
//     },
//   },
//   // Omerc Type B - #308
//   {
//     code: '+proj=omerc +lat_0=37.4769061 +lonc=141.0039618 +alpha=202.22 +k=1 +x_0=138 +y_0=77.65 +ellps=WGS84 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs',
//     xy: [168.2438, 64.1736],
//     ll: [141.003611, 37.476802],
//     acc: {
//       ll: 9,
//       xy: 4,
//     },
//   },
//   {
//     code: 'PROJCS["UNK / Oblique_Mercator",GEOGCS["UNK",DATUM["Unknown datum",SPHEROID["WGS 84", 6378137.0, 298.257223563]],PRIMEM["Greenwich",0],UNIT["degree",0.017453292519943295]],PROJECTION["Oblique_Mercator"],PARAMETER["latitude_of_center",37.4769061],PARAMETER["longitude_of_center",141.0039618],PARAMETER["central_meridian",141.0039618],PARAMETER["azimuth",202.22],PARAMETER["scale_factor",1],PARAMETER["false_easting",138],PARAMETER["false_northing",77.65],UNIT["Meter",1]]',
//     xy: [168.2438, 64.1736],
//     ll: [141.003611, 37.476802],
//     acc: {
//       ll: 9,
//       xy: 4,
//     },
//   },
//   // Test with Feet
//   {
//     code: 'PROJCS["UNK / Oblique_Mercator",GEOGCS["UNK",DATUM["Unknown datum",SPHEROID["WGS 84", 6378137.0, 298.257223563]],PRIMEM["Greenwich",0],UNIT["degree",0.017453292519943295]],PROJECTION["Oblique_Mercator"],PARAMETER["latitude_of_center",37.4769061],PARAMETER["longitude_of_center",141.0039618],PARAMETER["central_meridian",141.0039618],PARAMETER["azimuth",202.22],PARAMETER["scale_factor",1],PARAMETER["false_easting",138],PARAMETER["false_northing",77.65],UNIT["Foot_US",0.3048006096012192]]',
//     xy: [237.22488871325027, 33.43626458451221],
//     ll: [141.003611, 37.476802],
//   },
