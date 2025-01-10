use clap::{App, Arg};
use colored::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{self, copy, BufReader, Read, Write};
use std::path::Path;
use std::time::Instant;

#[derive(Debug)]
enum CompressionError {
    IoError(io::Error),
    InvalidInput(String),
}

impl From<io::Error> for CompressionError {
    fn from(error: io::Error) -> Self {
        CompressionError::IoError(error)
    }
}

struct CompressionStats {
    source_size: u64,
    target_size: u64,
    elapsed: std::time::Duration,
    compression_ratio: f64,
}

fn get_compression_level(level: &str) -> Compression {
    match level {
        "fast" => Compression::fast(),
        "best" => Compression::best(),
        _ => Compression::default(),
    }
}

fn compress_file(
    source: &str,
    target: &str,
    compression_level: Compression,
    show_progress: bool,
) -> Result<CompressionStats, CompressionError> {
    let source_path = Path::new(source);
    if !source_path.exists() {
        return Err(CompressionError::InvalidInput(format!(
            "Source file '{}' does not exist",
            source
        )));
    }

    let input_file = File::open(source_path)?;
    let file_size = input_file.metadata()?.len();
    let mut input = BufReader::new(input_file);

    let output = File::create(target)?;
    let mut encoder = GzEncoder::new(output, compression_level);

    let progress_bar = if show_progress {
        let pb = ProgressBar::new(file_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    let start = Instant::now();

    if let Some(pb) = &progress_bar {
        let mut buffer = vec![0; 1024 * 1024]; // 1MB buffer
        let mut total_read = 0u64;

        loop {
            let len = input.read(&mut buffer)?;
            if len == 0 {
                break;
            }
            encoder.write_all(&buffer[..len])?;
            total_read += len as u64;
            pb.set_position(total_read);
        }
    } else {
        copy(&mut input, &mut encoder)?;
    }

    if let Some(pb) = progress_bar {
        pb.finish_with_message("Compression complete");
    }

    let output = encoder.finish()?;
    let target_size = output.metadata()?.len();
    let source_size = input.get_ref().metadata()?.len();
    let compression_ratio = 1.0 - (target_size as f64 / source_size as f64);

    Ok(CompressionStats {
        source_size,
        target_size,
        elapsed: start.elapsed(),
        compression_ratio,
    })
}

fn main() {
    let matches = App::new("File Compressor")
        .version("2.0")
        .author("kushwahramkumar2003@gmail.com")
        .about("Compresses files using GZIP compression")
        .arg(
            Arg::new("source")
                .help("Source file to compress")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("target")
                .help("Target compressed file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("compression")
                .short('c')
                .long("compression")
                .help("Compression level (fast, default, best)")
                .default_value("default"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Disable progress bar")
                .takes_value(false),
        )
        .get_matches();

    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();
    let compression_level = get_compression_level(matches.value_of("compression").unwrap());
    let show_progress = !matches.is_present("quiet");

    println!("{}", "\nFile Compression Utility".bright_green().bold());
    println!("{}", "=======================".bright_green());

    match compress_file(source, target, compression_level, show_progress) {
        Ok(stats) => {
            println!("\n{}", "Compression Summary:".bright_blue().bold());
            println!(
                "{}: {:.2} MB",
                "Source file size".bright_yellow(),
                stats.source_size as f64 / 1_048_576.0
            );
            println!(
                "{}: {}",
                "Compressed size".bright_yellow(),
                stats.target_size as f64 / 1_048_576.0
            );
            println!(
                "{}: {:.1}%",
                "Compression ratio".bright_yellow(),
                stats.compression_ratio * 100.0
            );
            println!("{}: {:.2?}", "Time elapsed".bright_yellow(), stats.elapsed);
            println!(
                "\n{}\n",
                "Compression completed successfully!".bright_green().bold()
            );
        }
        Err(error) => {
            eprintln!(
                "\n{} {}",
                "Error:".bright_red().bold(),
                match error {
                    CompressionError::IoError(e) => format!("IO error: {}", e),
                    CompressionError::InvalidInput(e) => e,
                }
            );
            std::process::exit(1);
        }
    }
}
