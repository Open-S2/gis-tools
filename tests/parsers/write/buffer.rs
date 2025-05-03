#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use gistools::parsers::{BufferWriter, Writer};

    #[test]
    fn test_new() {
        let mut writer = BufferWriter::new(vec![]);
        writer.append(&[0, 1, 2, 3, 4]);
        writer.append_string("TEST!");
        writer.write(&[10, 9], 1);

        let data = writer.take();

        assert_eq!(data, vec![0, 10, 9, 3, 4, 84, 69, 83, 84, 33]);
    }
}
