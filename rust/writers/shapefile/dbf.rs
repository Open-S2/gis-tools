use crate::{
    parsers::{FeatureReader, Writer},
    readers::DBFRow,
    util::Date,
};
use alloc::collections::BTreeMap;
use s2json::{MValue, MValueCompatible, PrimitiveValue};

/// DBF File Version
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DBFFileVersion {
    /// dBase III without memo file
    #[default]
    DBase3WithoutMemo = 0x03,
    /// dBase III with memo file
    DBase3WithMemo = 0x83,
    /// dBase IV with memo file
    DBase4WithMemo = 0x8b,
    /// Visual FoxPro 9 (may have memo file)
    VisualFoxPro9 = 0x30,
    /// FoxPro 2.x (may have memo file)
    FoxPro2WithMemo = 0xf5,
}

/// Sweeps over all feature property collections to assemble a unified DBF column schema definition.
///
/// ## Parameters
/// - `iterators`: Multiple array of records containing the GeoJSON feature property values
///
/// ## Returns
/// An array of field schema objects tailored to conform with DBF specifications
pub fn to_dbf_meta<
    M: Clone,
    P: MValueCompatible,
    D: MValueCompatible,
    I: FeatureReader<M, P, D>,
>(
    iterators: &[&I],
) -> (Vec<DBFRow>, usize) {
    // Use a map to accumulate the most permissive size constraints across all rows
    let mut schema_map = BTreeMap::<String, DBFRow>::new();
    let mut feature_count = 0;

    for iterator in iterators {
        for feature in iterator.iter() {
            feature_count += 1;
            let props: MValue = feature.properties.clone().into();
            for (key, value) in props.iter() {
                if let Some(value) = value.to_prim() {
                    if value.is_null() {
                        continue;
                    }
                    let normalized_key =
                        if key.len() <= 10 { key.clone() } else { key[..10].to_string() };
                    let data_type = get_type(value);
                    // Initialize configuration parameters for this column if missing
                    if !schema_map.contains_key(&normalized_key) {
                        schema_map.insert(
                            normalized_key.clone(),
                            DBFRow { name: normalized_key.clone(), data_type, len: 0, decimal: 0 },
                        );
                    }

                    let current_meta = schema_map.get_mut(&normalized_key).unwrap();
                    // Handle type widening if a column shifts from Boolean/Numeric up to String
                    if current_meta.data_type != data_type && current_meta.data_type != 'C' {
                        current_meta.data_type = 'C';
                    }
                    // Calculate formatting geometry constraints for the current specific value
                    if current_meta.data_type == 'N' {
                        let (total_length, decimal_places) = get_numeric_constraints(value);
                        current_meta.decimal = u64::max(current_meta.decimal, decimal_places);
                        current_meta.len = u64::max(current_meta.len, total_length);
                    } else {
                        // Handle length constraints for strings or fallback characters
                        let string_len = value.to_string().unwrap_or_default().len() as u64;
                        current_meta.len = u64::max(current_meta.len, string_len);
                    }
                }
            }
        }
    }

    // Cap fallback sizes and enforce maximum specification boundaries
    let schema_list: Vec<DBFRow> = schema_map
        .into_values()
        .map(|mut row| {
            if row.data_type == 'N' {
                // dBase III bounds: 1 to 18 characters
                row.len = u64::min(u64::max(row.len, 1), 18);
            } else {
                row.len = u64::min(u64::max(row.len, get_size(row.data_type) as u64), 254);
            }
            row
        })
        .collect();

    (schema_list, feature_count)
}

/// # DBF Writer
///
/// ## Description
///
/// Given a writer and an array of iterators, write the input features property data into a DBF file
///
/// ## Usage
/// ```ts
/// import { to_dbf, JSONReader } from 'gis-tools-ts';
/// import { FileReader, FileWriter } from 'gis-tools-ts/file';
/// // or use mmap reader if using bun
/// // import { MMapReader } from 'gis-tools-ts/mmap';
/// // or use a BufferWriter if you are using a browser
/// // import { BufferWriter } from 'gis-tools-ts';
///
/// const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
/// const jsonReader = new JSONReader(fileReader);
/// const bufWriter = new FileWriter(`${__dirname}/fixtures/points.dbf`);
///
/// // store to singular output
/// await to_dbf(bufWriter, [jsonReader]);
/// ```
///
/// ## Parameters
/// - `writer`: the writer to append strings to
/// - `iterators`: the collection of iterators to write
pub fn to_dbf<
    T: Writer,
    M: Clone,
    P: MValueCompatible,
    D: MValueCompatible,
    I: FeatureReader<M, P, D>,
>(
    writer: &mut T,
    iterators: Vec<&I>,
) {
    let (meta, feature_count) = to_dbf_meta(&iterators);

    // 1. Compute foundational structural offsets
    let field_desc_length = 32 * meta.len() + 1; // 32 bytes per descriptor + 0x0D terminator
    let header_length = 32 + field_desc_length;
    // Calculate raw bytes per row payload: 1 byte for the deletion flag + sum of field widths
    let mut bytes_per_record = 1;
    for row in &meta {
        bytes_per_record += row.len as usize;
    }

    // 2. Build and emit the primary 32-byte DBF File Header
    let mut header_buffer = [0u8; 32];
    let now = Date::now();

    header_buffer[0] = 0x03; // dBase III signature version code
    header_buffer[1] = (now.year - 1900) as u8; // Year offset tracking
    header_buffer[2] = now.month + 1; // Month index (1-based)
    header_buffer[3] = now.day; // Day of the month
    // Total record inventory size (Little Endian)
    header_buffer[4..8].copy_from_slice(&(feature_count as u32).to_le_bytes());
    // Total header block footprint
    header_buffer[8..10].copy_from_slice(&(header_length as u16).to_le_bytes());
    // Total record byte sequence distance
    header_buffer[10..12].copy_from_slice(&(bytes_per_record as u16).to_le_bytes());
    writer.append(&header_buffer);

    // 3. Build and emit the Field Descriptor Array (32 bytes per field)
    for field in &meta {
        let mut desc_buffer = [0u8; 32];
        // Encode field name directly up to 10 characters max, padded with nulls
        let name_bytes = field.name.as_bytes();
        let name_len = usize::min(name_bytes.len(), 10);
        desc_buffer[..name_len].copy_from_slice(&name_bytes[..name_len]);
        // Set field DataType, Field Width length, and decimal sub-precision parameters
        desc_buffer[11] = field.data_type as u8;
        desc_buffer[16] = field.len as u8;
        desc_buffer[17] = field.decimal as u8;

        writer.append(&desc_buffer);
    }
    // Write out the required field description array terminator block
    writer.append(&[0x0d]);

    // 4. Transform and stream specific entity attribute rows
    for iterator in iterators {
        for feature in iterator.iter() {
            let mut record_buffer = vec![0x20; bytes_per_record as usize];
            let props: MValue = feature.properties.clone().into();
            let mut offset = 0;

            // Write deletion indicator token: 0x20 represents an active valid entity row
            record_buffer[offset] = 0x20;
            offset += 1;

            //       for (const field of meta) {
            for field in &meta {
                //         const rawValue = row[field.name];
                //         let stringPayload = rawValue === null || rawValue === undefined ? '' : String(rawValue);
                //         if (field.data_type === 'N') {
                //           // Numbers must align exactly with your custom precision metrics
                //           const numValue = Number(rawValue);
                //           if (!isNaN(numValue)) {
                //             stringPayload = numValue.toFixed(field.decimal);
                //           }
                //           // Left-pad with empty space characters matching dBase standard formatting expectations
                //           stringPayload = stringPayload.padStart(field.len, ' ').slice(0, field.len);
                //         } else if (field.data_type === 'L') {
                //           // Enforce valid Boolean indicators: 'T' or 'F'
                //           const lower = stringPayload.toLowerCase();
                //           const isTrue = rawValue === true || ['true', 't', 'y'].includes(lower);
                //           stringPayload = isTrue ? 'T' : 'F';
                //         } else if (field.data_type === 'D') {
                //           // Dates require strict YYYYMMDD string format representations
                //           if (rawValue instanceof Date) {
                //             const y = rawValue.getFullYear();
                //             const m = String(rawValue.getMonth() + 1).padStart(2, '0');
                //             const d = String(rawValue.getDate()).padStart(2, '0');
                //             stringPayload = `${y}${m}${d}`;
                //           }
                //           stringPayload = stringPayload.padStart(field.len, ' ').slice(0, field.len);
                //         } else {
                //           // Standard alphanumeric string processing (Right-padded with spacing characters)
                //           stringPayload = stringPayload.padEnd(field.len, ' ').slice(0, field.len);
                //         }

                //         // Safe continuous allocation block layout injection via TextEncoder
                //         const encodedFieldBytes = textEncoder.encode(stringPayload);
                //         for (let b = 0; b < field.len; b++) {
                //           recordBuffer[offset++] = b < encodedFieldBytes.length ? encodedFieldBytes[b] : 0x20;
                //         }
                let mut string_payload = String::new();

                if let Some(value) = props.get(&field.name).and_then(|v| v.to_prim()) {
                    if !value.is_null() {
                        if field.data_type == 'N' {
                            if let Some(num_value) = value.to_f64() {
                                // Formatting with custom precision
                                string_payload =
                                    format!("{:.1$}", num_value, field.decimal as usize);
                            }
                            // Left-pad with spaces
                            let width = field.len as usize;
                            if string_payload.len() < width {
                                let padding = " ".repeat(width - string_payload.len());
                                string_payload = format!("{}{}", padding, string_payload);
                            }
                            string_payload.truncate(width);
                        } else if field.data_type == 'L' {
                            let val_str = value.to_string().unwrap_or_default().to_lowercase();
                            let is_true = val_str == "true" || val_str == "t" || val_str == "y";
                            string_payload =
                                if is_true { "T".to_string() } else { "F".to_string() };
                        } else if field.data_type == 'D' {
                            // Convert the property into your custom Date type
                            if let Some(val_str) = value.to_string() {
                                let date = Date::from(val_str.as_str());
                                string_payload = format!("{}", date); // Uses your Display implementation
                            }

                            // Field width tracking and slicing
                            let width = field.len as usize;
                            if string_payload.len() < width {
                                let padding = " ".repeat(width - string_payload.len());
                                string_payload = format!("{}{}", padding, string_payload);
                            }
                            string_payload.truncate(width);
                        } else {
                            // Standard alphanumeric string processing
                            string_payload = value.to_string().unwrap_or_default();
                            let width = field.len as usize;
                            if string_payload.len() < width {
                                let padding = " ".repeat(width - string_payload.len());
                                string_payload = format!("{}{}", string_payload, padding); // Right padded
                            }
                            string_payload.truncate(width);
                        }
                    }
                }

                // If nothing was generated (null/missing value), pad entirely with spaces
                if string_payload.is_empty() {
                    string_payload = " ".repeat(field.len as usize);
                }

                // Inject encoded field bytes into allocation block
                let encoded_bytes = string_payload.as_bytes();
                let field_width = field.len as usize;
                for b in 0..field_width {
                    record_buffer[offset] =
                        if b < encoded_bytes.len() { encoded_bytes[b] } else { 0x20 };
                    offset += 1;
                }
            }
            // Stream out the record buffer sequence directly to disk or memory stream instantly
            writer.append(&record_buffer);
        }
    }

    // 5. Emit structural standard EOF byte sequence completion flag
    writer.append(&[0x1a]);
}

/// Parses numeric properties safely to establish full string geometry and decimal requirements.
///
/// ## Parameters
/// - `value`: the value to parse
///
/// ## Returns
/// Numeric constraints, (total_length, decimal_places)
fn get_numeric_constraints(value: &PrimitiveValue) -> (u64, u64) {
    if !value.is_number() {
        return (18, 0);
    }
    let num = value.to_f64().unwrap();

    // Convert to fixed notation to sidestep scientific notation snags (like 1e-7)
    // 15 is the IEEE-754 double precision ceiling for precise decimal representations
    let mut s = format!("{:.15}", num);

    // Strip floating point binary trailing zeros
    if s.contains('.') {
        s = s.trim_end_matches('0').to_string();
        if let Some(stripped) = s.strip_suffix('.') {
            s = stripped.to_string();
        }
    }

    // Assuming `s` is a String from the previous step and `num` is your f64
    let parts = s.split_once('.');
    let (integer_part, decimal_part) = match parts {
        Some((int, dec)) => (int, dec),
        None => (s.as_str(), ""),
    };
    // Exclude negative sign from index tracking
    let integer_part_clean = integer_part.strip_prefix('-').unwrap_or(integer_part);
    let decimal_places = usize::min(decimal_part.len(), 15);
    // DBF Numeric width tracking
    let is_negative = if num < 0.0 { 1 } else { 0 };
    let has_dot = if decimal_places > 0 { 1 } else { 0 };
    let total_length = integer_part_clean.len() + is_negative + has_dot + decimal_places;

    (total_length as u64, decimal_places as u64)
}

fn get_type(value: &PrimitiveValue) -> char {
    match value {
        PrimitiveValue::F32(_)
        | PrimitiveValue::F64(_)
        | PrimitiveValue::I64(_)
        | PrimitiveValue::U64(_) => 'N',
        PrimitiveValue::String(s) => {
            let lower = s.to_lowercase();
            if lower == "true"
                || lower == "false"
                || lower == "t"
                || lower == "f"
                || lower == "y"
                || lower == "n"
            {
                'L'
            } else {
                'C'
            }
        }
        PrimitiveValue::Bool(_) => 'L',
        _ => 'C',
    }
}

fn get_size(value: char) -> u8 {
    match value {
        'C' => 254,
        'L' => 1,
        'D' => 8,
        'B' => 8,
        _ => 18,
    }
}
