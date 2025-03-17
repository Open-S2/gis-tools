use super::SortChunk;
use crate::{geometry::S2CellId, util::Buffer};
use std::{
    format, fs::OpenOptions, io::Write, os::unix::fs::FileExt, string::String, vec, vec::Vec,
};

/// A Key is a 16 byte buffer that contains a uint64 id split into lo and hi.
/// The last 8 bytes contains the u32 offset and u32 length
pub struct Key {
    pub id: S2CellId,
    offset: u32,
    length: u32,
}

/// Given chunk information needed to start sorting, sort the chunk and return the intermediate
/// output file created
pub fn sort_chunk(chunk: SortChunk) -> String {
    let SortChunk { name, input, out_dir, start, end, value_offset } = chunk;
    // let out_file = `${outDir}/es_${name}_${start}_${end}.tmp`;
    let out_file = format!("{}/es_{}_{}_{}.tmp", out_dir, name, start, end);
    _sort_chunk(&input, &out_file, start, end, value_offset);
    out_file
}

/// sourts at an actual value offset if merging values of multiple files
fn _sort_chunk(input: &str, output: &str, start: u64, end: u64, value_offset: u64) {
    let input_handle = OpenOptions::new().read(true).open(input).unwrap();
    let mut output_handle =
        OpenOptions::new().read(true).create(true).append(true).open(output).unwrap();
    // read in the chunk
    let mut input_bytes = vec![0; (end - start) as usize];
    let _ = input_handle.read_at(&mut input_bytes, start);
    let mut input_buffer = Buffer::new(input_bytes);
    // sort the chunk
    let mut keys = buffer_to_keys(&mut input_buffer);
    keys.sort_by(|a, b| a.id.cmp(&b.id));
    // update keys to correct offset
    for key in keys.iter_mut() {
        key.offset += value_offset as u32;
    }
    // write out the sorted chunk
    let sorted_buffer = keys_to_buffer(keys);
    let _ = output_handle.write_all(&sorted_buffer);
    let _ = output_handle.flush();
}

/// Convert a buffer to a list of keys
pub fn buffer_to_keys(buffer: &mut Buffer) -> Vec<Key> {
    // for each 16 bytes in the buffer, create a key
    let mut keys: Vec<Key> = vec![];
    let mut i = 0;
    while i < buffer.len() {
        keys.push(Key {
            // id: buffer.readBigUInt64LE(i),
            id: buffer.get_u64_at(i).into(),
            // offset: buffer.readUInt32LE(i + 8),
            offset: buffer.get_u32_at(i + 8),
            // length: buffer.readUInt32LE(i + 12),
            length: buffer.get_u32_at(i + 12),
        });
        i += 16
    }

    keys
}

/// Convert a list of keys to a buffer
pub fn keys_to_buffer(keys: Vec<Key>) -> Vec<u8> {
    let res: Vec<u8> = vec![0; keys.len() * 16];
    let mut buffer = Buffer::new(res);
    let mut i = 0;
    while i < keys.len() {
        let key = &keys[i];
        buffer.set_u64_at(i * 16, key.id.id);
        buffer.set_u32_at(i * 16 + 8, key.offset);
        buffer.set_u32_at(i * 16 + 12, key.length);
        i += 1
    }

    buffer.take()
}
