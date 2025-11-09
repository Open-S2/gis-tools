use clap::{Args, Parser, Subcommand, ValueEnum};
use gistools::{
    parsers::{FileReader, FileWriter},
    readers::{GISReader, ReaderType},
    writers::{ToJSONOptions, to_json, to_jsonld},
};
use s2json::Projection;

/// GIS Tools CLI
#[derive(Parser)]
#[command(
    name = "gis-tools",
    version,
    about = "CLI for GIS Tools",
    long_about = "Found an issue? Please open an issue on GitHub at:\nhttps://github.com/Open-S2/gis-tools/issues"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available subcommands
#[derive(Subcommand)]
enum Commands {
    Convert(ConvertArgs),
}

/// Arguments for `convert`
#[derive(Args)]
struct ConvertArgs {
    /// Input file path
    #[arg(short = 'i', long = "input")]
    input: Vec<String>, // <-- allow multiple input files
    /// Output file path
    #[arg(short = 'o', long = "output")]
    output: String,
    /// Input file format
    #[arg(short = 'f', long = "inputFormat", value_enum)]
    input_format: Option<ReaderType>,
    /// Output file format
    #[arg(short = 'O', long = "outputFormat", value_enum)]
    output_format: Option<OutFileType>,
    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

/// Supported output types
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutFileType {
    Json,
    Geojson,
    S2json,
    #[value(name = "json-ld")]
    JsonLd,
    #[value(name = "geojson-ld")]
    GeojsonLd,
    #[value(name = "s2json-ld")]
    S2jsonLd,
}
impl std::fmt::Display for OutFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutFileType::Json => f.write_str("json"),
            OutFileType::Geojson => f.write_str("geojson"),
            OutFileType::S2json => f.write_str("s2json"),
            OutFileType::JsonLd => f.write_str("json-ld"),
            OutFileType::GeojsonLd => f.write_str("geojson-ld"),
            OutFileType::S2jsonLd => f.write_str("s2json-ld"),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert(args) => {
            if args.verbose {
                println!("Converting inputs: {:?}", args.input);
            }

            let out_type = args.output_format.unwrap_or(OutFileType::Json);
            let readers: Vec<GISReader<FileReader>> = args
                .input
                .iter()
                .map(|path| GISReader::from_path(path.clone(), args.input_format.clone(), None))
                .collect();
            let mut writer = FileWriter::new(args.output.clone()).unwrap();

            println!(
                "Input: {:?} ({:?}), Output: {} ({})",
                args.input, args.input_format, args.output, out_type
            );

            match out_type {
                OutFileType::Json | OutFileType::Geojson | OutFileType::S2json => to_json(
                    &mut writer,
                    readers.iter().collect(),
                    Some(ToJSONOptions {
                        projection: Some(if out_type == OutFileType::S2json {
                            Projection::S2
                        } else {
                            Projection::WG
                        }),
                        geojson: Some(
                            out_type == OutFileType::Json || out_type == OutFileType::Geojson,
                        ),
                        ..Default::default()
                    }),
                ),
                OutFileType::JsonLd | OutFileType::GeojsonLd | OutFileType::S2jsonLd => to_jsonld(
                    &mut writer,
                    readers.iter().collect(),
                    Some(ToJSONOptions {
                        projection: Some(if out_type == OutFileType::S2jsonLd {
                            Projection::S2
                        } else {
                            Projection::WG
                        }),
                        geojson: Some(
                            out_type == OutFileType::JsonLd || out_type == OutFileType::GeojsonLd,
                        ),
                        ..Default::default()
                    }),
                ),
            }

            if args.verbose {
                println!("Output written to {}", args.output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::OutFileType;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn test_cli_out_file_type() {
        let test = OutFileType::Json;
        assert_eq!(test.to_string(), "json");

        let test = OutFileType::JsonLd;
        assert_eq!(test.to_string(), "json-ld");

        let test = OutFileType::Geojson;
        assert_eq!(test.to_string(), "geojson");

        let test = OutFileType::GeojsonLd;
        assert_eq!(test.to_string(), "geojson-ld");

        let test = OutFileType::S2json;
        assert_eq!(test.to_string(), "s2json");

        let test = OutFileType::S2jsonLd;
        assert_eq!(test.to_string(), "s2json-ld");
    }

    #[test]
    #[allow(deprecated)]
    fn test_convert_geojson_to_json() {
        // Path to input fixture
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("tests/readers/json/fixtures/points.geojson");

        // Create temporary output file
        let output = NamedTempFile::new().expect("Failed to create temporary file");
        let output_path = output.path();

        // Run CLI: gis-tools convert
        let mut cmd = Command::cargo_bin("gis-tools").unwrap();
        cmd.args([
            "convert",
            "-i",
            input.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "-f",
            "geojson",
            "-O",
            "json",
            "-v",
        ]);

        cmd.assert().success().stdout(predicate::str::contains("Converting inputs"));

        // Check output file was written
        let contents = std::fs::read_to_string(output_path).expect("Output not written");
        assert!(contents.contains("{"));
        assert!(contents.contains("features"));
    }

    #[test]
    #[allow(deprecated)]
    fn test_convert_geojson_to_geojsonld() {
        // Path to input fixture
        let mut input = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input.push("tests/readers/json/fixtures/points.geojson");

        // Create temporary output file
        let output = NamedTempFile::new().expect("Failed to create temporary file");
        let output_path = output.path();

        // Run CLI: gis-tools convert
        let mut cmd = Command::cargo_bin("gis-tools").unwrap();
        cmd.args([
            "convert",
            "-i",
            input.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "-f",
            "geojson",
            "-O",
            "geojson-ld",
            "-v",
        ]);

        cmd.assert().success().stdout(predicate::str::contains("Converting inputs"));

        // Check output file was written
        let contents = std::fs::read_to_string(output_path).expect("Output not written");
        assert!(contents.contains("Feature"), "Expected JSON-LD context in output");
        // assert!(contents.contains("FeatureCollection"));
    }
}
