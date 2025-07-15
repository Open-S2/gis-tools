#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use gistools::parsers::{FileWriter, Writer};
    use tempfile::NamedTempFile;

    #[test]
    fn test_new() {
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let file_path = temp_file.path().to_string_lossy().into_owned();

        let mut writer = FileWriter::new(&file_path).unwrap();
        writer.append(&[0, 1, 2, 3, 4]);
        writer.append_string("TEST!");
        writer.write(&[10, 9], 1);

        let data = writer.take();

        assert_eq!(data, vec![0, 10, 9, 3, 4, 84, 69, 83, 84, 33]);
    }
}
