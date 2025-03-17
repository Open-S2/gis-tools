mod merge_sorted_chunks;
mod sort_chunk;

use merge_sorted_chunks::*;
use sort_chunk::*;
use std::{
    env, format,
    fs::{self, File},
    io::{copy, BufReader, BufWriter, Write},
    path::Path,
    string::{String, ToString},
    sync::{Arc, Mutex},
    thread::{self, available_parallelism},
    vec,
    vec::Vec,
};

/// Sorts an array using external-sorting.
pub fn external_sort(
    inputs: &[&str], /* a list of input files without their extensions. e.g. './file1', './file2', './file3' */
    output: &str,    // output folder to place the sorted keys
    max_heap: Option<u64>, // max instance of the parsed entity in memory
    thread_count: Option<u64>, // number of workers
    tmp_dir: Option<&str>, // temporary directory to store intermediate sorted files
) {
    let max_heap = max_heap.unwrap_or(100_000);
    let thread_count = thread_count.unwrap_or(1) as usize;
    let binding = tmpdir();
    let tmp_dir = tmp_dir.unwrap_or(&binding);
    // 1) Get the size of the input
    let sizes = get_sizes(inputs);
    // 2) Build chunk list
    let chunks = build_chunks(&sizes, tmp_dir, max_heap);
    // 3) Sort chunks - using either workers or single threaded
    let mut sorted_files: Vec<String> = vec![];
    if thread_count == 1 || chunks.len() <= 10 {
        for chunk in chunks {
            sorted_files.push(sort_chunk(chunk));
        }
    } else {
        sorted_files = sort_chunks_with_threads(chunks, thread_count);
    }
    // 4) Merge chunks
    merge_sorted_chunks(&sorted_files, output);
    merge_values(output, sizes);
    // 5) Cleanup
    for file in sorted_files {
        fs::remove_file(file).unwrap();
    }
}

/// A File name and it's size
#[derive(Clone)]
pub struct FileSize {
    /// Name of the folder
    name: String,
    /// Name of the input (there could be multiple input files to sort)
    input: String,
    /// Total size of the key store
    key_size: u64,
    /// Total size of the item store
    value_size: u64,
    /// Offset for values
    value_offset: u64,
}

/// Get the sizes of the inputs
fn get_sizes(inputs: &[&str]) -> Vec<FileSize> {
    let mut sizes: Vec<FileSize> = vec![];
    let mut value_offset = 0;

    for input in inputs {
        let key_size = file_size(&format!("{}.keys", input));
        let value_size = file_size(&format!("{}.values", input));
        let name = Path::new(input)
            .file_name() // Get file name as OsStr
            .and_then(|name| name.to_str()) // Convert OsStr to &str
            .unwrap_or("Unknown")
            .to_string();
        sizes.push(FileSize { name, input: input.to_string(), key_size, value_size, value_offset });
        value_offset += value_size;
    }

    sizes
}

fn file_size(file: &str) -> u64 {
    if let Ok(metadata) = fs::metadata(file) {
        metadata.len() // File size in bytes
    } else {
        0
    }
}

/// A chunk of a file to be sorted
pub struct SortChunk {
    name: String, // name of the input (there could be multiple input files to sort)
    input: String,
    out_dir: String,
    start: u64,
    end: u64,
    value_offset: u64,
}

/// Build chunks from all files. Chunks help describe a portion of work to complete
fn build_chunks(file_sizes: &[FileSize], out_dir: &str, max_heap: u64) -> Vec<SortChunk> {
    let mut chunks: Vec<SortChunk> = vec![];

    for file_size in file_sizes {
        let FileSize { name, input, key_size, value_offset, .. } = file_size;
        let mut start = 0;
        while start < *key_size {
            let end = (start + max_heap * 16).min(*key_size);
            chunks.push(SortChunk {
                name: name.clone(),
                input: format!("{}.keys", input),
                out_dir: out_dir.to_string(),
                start,
                end,
                value_offset: *value_offset,
            });
            start += max_heap * 16;
        }
    }

    chunks
}

/// Sorts a list of chunks using threads
fn sort_chunks_with_threads(chunks: Vec<SortChunk>, tc: usize) -> Vec<String> {
    let parallelism = available_parallelism().map(|n| n.get()).unwrap_or(1);
    let thread_count = usize::min(tc, parallelism);

    // Create threads that take from the `chunks` vector until its empty and call sort_chunk.
    // Store the resulant string from sort_chunk in the `sorted_files` vector.
    let chunks = Arc::new(Mutex::new(chunks));
    let sorted_files = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::with_capacity(thread_count);
    for _ in 0..thread_count {
        let chunks = Arc::clone(&chunks);
        let sorted_files = Arc::clone(&sorted_files);

        let handle = thread::spawn(move || {
            while let Some(chunk) = chunks.lock().unwrap().pop() {
                let result = sort_chunk(chunk);
                sorted_files.lock().unwrap().push(result);
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }

    Arc::try_unwrap(sorted_files).unwrap().into_inner().unwrap()
}

/// merge the values files since the sorted key indexes have been merged as well.
fn merge_values(output: &str, sizes: Vec<FileSize>) {
    if sizes.len() <= 1 {
        return;
    }
    let mut sorted_sizes = sizes.clone(); // Clone if you need to keep original order
    sorted_sizes.sort_by(|a, b| a.value_offset.cmp(&b.value_offset));

    let values: Vec<String> = sorted_sizes
        .into_iter() // Now consume the sorted vector
        .filter(|c| c.input != output && c.value_size > 0)
        .map(|c| c.input)
        .collect();

    if values.is_empty() {
        return;
    }

    let output = File::create(format!("{}.values", output)).unwrap();
    let mut writer = BufWriter::new(output);

    for value in values {
        let input = File::open(format!("{}.values", value)).unwrap();
        let mut reader = BufReader::new(input);

        // write into output
        copy(&mut reader, &mut writer).unwrap();
    }

    writer.flush().unwrap();
}

fn tmpdir() -> String {
    env::temp_dir().to_str().unwrap().to_string()
}
