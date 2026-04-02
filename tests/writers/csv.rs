#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::{string::String, vec};
    use gistools::{
        parsers::{BufferWriter, FileReader, Writer},
        readers::json::JSONReader,
        writers::{ToCSVOptions, to_csv},
    };
    use s2json::MValueCompatible;
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[test]
    fn test_to_csv() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path = path.join("tests/writers/fixtures/points.geojson");

        let reader: JSONReader<FileReader, (), Props, ()> = JSONReader::new(FileReader::from(path));
        let mut writer = BufferWriter::default();

        // write
        to_csv(
            &mut writer,
            vec![&reader],
            Some(ToCSVOptions { properties: Some(vec!["name".into()]), ..Default::default() }),
        );

        // validate
        let writer_str: String = String::from_utf8_lossy(&writer.take()).into();
        let expected = "lon,lat,name\n144.9584,-37.8173,Melbourne\n149.1009,-35.3039,Canberra\n151.2144,-33.8766,Sydney\n";
        assert_eq!(&writer_str, expected);
    }
}
