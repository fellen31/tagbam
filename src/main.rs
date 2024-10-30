mod bam_processor;
mod utils;

use clap::Parser;
use bam_processor::process_bam_file;
use utils::validate_tag;

/// The arguments end up in the Cli struct
#[derive(Parser, Debug)]
#[clap(author, version, about = "Tag reads in BAM file", long_about = None)]
pub struct Args {
    /// Input BAM file
    #[clap(short, long, value_parser)]
    pub input: String,

    /// Number of parallel decompression & writer threads to use
    #[clap(short, long, value_parser, default_value_t = 4)]
    pub threads: u32,

    /// Tag to add (must be 1-2 characters)
    #[clap(long, value_parser)]
    pub tag: String,
    
    /// Value to add
    #[clap(short, long, value_parser)]
    pub value: i8,
    
    /// Output file
    #[clap(short, long, value_parser)]
    pub output_file: String,

    /// BAM output compression level
    #[clap(short, long, value_parser, default_value_t = 6)]
    pub compression: u32,
}

fn main() {
    // Parse arguments
    let args = Args::parse();
    
    // Validate the tag length
    if let Err(e) = validate_tag(&args.tag) {
        eprintln!("Validation error: {}", e);
        std::process::exit(1);
    }

    // Create a thread pool for reading and writing records
    let thread_pool = bam_processor::create_thread_pool(args.threads);

    // Process BAM file
    if let Err(e) = process_bam_file(
        &args.input,
        &args.output_file,
        &args.tag,
        args.value,
        args.compression,
        &thread_pool,
    ) {
        eprintln!("Error processing BAM file: {}", e);
        std::process::exit(1);
    }
}
