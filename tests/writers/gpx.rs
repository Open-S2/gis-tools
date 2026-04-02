#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        parsers::{BufferWriter, Writer},
        readers::JSONCollectionReader,
        writers::to_gpx,
    };
    use s2json::{MValue, Properties, VectorFeature};
    use std::{fs, path::PathBuf};

    // TODO: Cleanup output, its probably the readers fault?
    #[test]
    fn gpx_writer_basic() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gpx/fixtures/gpx-test-short.json");
        let data = fs::read_to_string(path).unwrap();

        let mut features: Vec<VectorFeature<MValue, Properties, MValue>> =
            serde_json::from_str(&data).unwrap();

        // remove the metadata from each feature
        features.iter_mut().for_each(|feature| {
            feature.metadata = None;
        });

        let json_reader: JSONCollectionReader<MValue, Properties, MValue> =
            JSONCollectionReader::from(features);

        // write to gpx
        let mut buf_writer = BufferWriter::new(Vec::new());

        to_gpx(&mut buf_writer, vec![&json_reader], None);

        let actual = String::from_utf8(buf_writer.take()).unwrap();
        // Uncomment to write to "tests/writers/fixtures/gpx-test-short-write.gpx"
        // fs::write("tests/writers/fixtures/gpx-test-short-write-rust.gpx", actual.clone()).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/writers/fixtures/gpx-test-short-write-rust.gpx");
        let expected = fs::read_to_string(path).unwrap();
        assert_eq!(expected, actual);
    }
}
