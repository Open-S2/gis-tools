use super::SortChunk;
use crate::parsers::Buffer;
use std::{
    format, fs::OpenOptions, io::Write, os::unix::fs::FileExt, string::String, vec, vec::Vec,
};

const KEY_STORE_LENGTH: usize = crate::data_store::file::KEY_STORE_LENGTH as usize;

/// A Key is a KEY_STORE_LENGTH byte buffer that contains a uint64 id split into lo and hi.
/// The last 8 bytes contains the u64 offset and u64 length
pub struct Key {
    pub id: u64,
    offset: u64,
    length: u32,
}

/// Given chunk information needed to start sorting, sort the chunk and return the intermediate
/// output file created
pub fn sort_chunk(chunk: SortChunk) -> String {
    let SortChunk { name, input, out_dir, start, end, value_offset } = chunk;
    let out_file = format!("{out_dir}/es_{name}_{start}_{end}.tmp");
    _sort_chunk(&input, &out_file, start, end, value_offset);
    out_file
}

/// sorts at an actual value offset if merging values of multiple files
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
    keys.sort_by_key(|a| a.id);
    // update keys to correct offset
    for key in keys.iter_mut() {
        key.offset += value_offset;
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
            id: buffer.get_u64_at(i),
            offset: buffer.get_u64_at(i + 8),
            length: buffer.get_u32_at(i + 16),
        });
        i += KEY_STORE_LENGTH
    }

    keys
}

/// Convert a list of keys to a buffer
pub fn keys_to_buffer(keys: Vec<Key>) -> Vec<u8> {
    let res: Vec<u8> = vec![0; keys.len() * KEY_STORE_LENGTH];
    let mut buffer = Buffer::new(res);
    let mut i = 0;
    while i < keys.len() {
        let key = &keys[i];
        buffer.set_u64_at(i * KEY_STORE_LENGTH, key.id);
        buffer.set_u64_at(i * KEY_STORE_LENGTH + 8, key.offset);
        buffer.set_u32_at(i * KEY_STORE_LENGTH + 16, key.length);
        i += 1
    }

    buffer.take()
}
