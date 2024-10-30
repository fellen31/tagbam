use rust_htslib::bam::{self, Read};
use rust_htslib::tpool::ThreadPool;
use std::env;
use std::path::Path;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Create a thread pool for BAM processing
pub fn create_thread_pool(threads: u32) -> ThreadPool {
    rust_htslib::tpool::ThreadPool::new(threads).unwrap()
}

/// Process the BAM file
pub fn process_bam_file(
    input_path: &str,
    output_file: &str,
    tag: &str,
    value: i8,
    compression_level: u32,
    thread_pool: &ThreadPool,
) -> Result<(), String> {
    let path = Path::new(input_path);

    // Read BAM file
    let mut bam = bam::Reader::from_path(path)
        .map_err(|e| format!("Failed to read BAM file: {}", e))?;
    bam.set_thread_pool(thread_pool)
        .map_err(|e| format!("Failed to set thread pool: {}", e))?;

    // Create new header from BAM
    let mut header = bam::Header::from_template(bam.header());
    
    // Create command-line command
    let command_line = env::args().collect::<Vec<String>>().join(" ");
    add_program_group_record(&mut header, &command_line);

    // Build writer
    let mut writer = bam::Writer::from_path(output_file, &header, bam::Format::Bam)
        .map_err(|e| format!("Failed to create BAM writer: {}", e))?;
    writer.set_thread_pool(thread_pool)
        .map_err(|e| format!("Failed to set thread pool: {}", e))?;
    writer.set_compression_level(bam::CompressionLevel::Level(compression_level))
        .map_err(|e| format!("Failed to set compression level: {}", e))?;

    // Push tags to record and write
    for record_result in bam.records() {
        let mut record = record_result
            .map_err(|e| format!("Error reading BAM record: {}", e))?;
        record.push_aux(tag.as_bytes(), bam::record::Aux::I8(value))
            .map_err(|e| format!("Error writing tag to BAM record: {}", e))?;
        writer.write(&record)
            .map_err(|e| format!("Error writing BAM record: {}", e))?;
    }
    
    Ok(())
}

/// Add a program group record to the BAM header
fn add_program_group_record(header: &mut bam::Header, command_line: &str) {
    let id = PKG_NAME;
    let pn = PKG_NAME;
    let vn = PKG_VERSION;
    let cl = command_line;

    let mut pg_record = bam::header::HeaderRecord::new(b"PG");
    pg_record.push_tag(b"ID", id);
    pg_record.push_tag(b"PN", pn);
    pg_record.push_tag(b"VN", vn);
    pg_record.push_tag(b"CL", cl);

    header.push_record(&pg_record);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use bam::Header; // Import the Header type for testing
    
    #[test]
    fn test_process_bam_file() {
        let bam_file = "data/test.bam";

        let output_bam = NamedTempFile::new().expect("Failed to create output temp BAM file");
        let tag = "HP";
        let value = 42;
        let compression_level = 5;
        let thread_pool = ThreadPool::new(1).unwrap(); // Adjust as necessary
        
        // Call the function to process the BAM file
        let result = process_bam_file(
            bam_file,
            output_bam.path().to_str().unwrap(),
            tag,
            value,
            compression_level,
            &thread_pool,
        );
        
        assert!(result.is_ok(), "Failed to process BAM file: {:?}", result);
        let mut bam_reader = bam::Reader::from_path(output_bam.path().to_str().unwrap()).expect("Failed to read temp BAM file");
        
        // Check that the records contain the new tag and value
        for record in bam_reader.records() {
            let record = record.expect("Failed to read record");
            let aux_iter = record.aux_iter();
            for result in aux_iter {
                match result {
                    Ok((temp_tag, temp_value)) => {
                        if std::str::from_utf8(temp_tag).unwrap() == tag {
                            assert_eq!(std::str::from_utf8(temp_tag).unwrap(), tag);
                            assert_eq!(temp_value, rust_htslib::bam::record::Aux::I8(value));
                        }
                    }
                    Err(e) => {
                        // Handle the error case
                        eprintln!("Error while parsing aux: {:?}", e);
                        // You may want to break or return depending on your logic
                    }
                }
            }
        }
    }
    
    #[test]
    fn test_process_bam_file_existing_tag() {
        let bam_file = "data/test.bam";

        let output_bam = NamedTempFile::new().expect("Failed to create output temp BAM file");
        let tag = "NM";
        let value = 42;
        let compression_level = 5;
        let thread_pool = ThreadPool::new(1).unwrap(); // Adjust as necessary
        
        // Call the function to process the BAM file
        let result = process_bam_file(
            bam_file,
            output_bam.path().to_str().unwrap(),
            tag,
            value,
            compression_level,
            &thread_pool,
        );
        
        assert!(result.is_err(), "Failed to process BAM file: {:?}", result);
    }

    #[test]
    fn test_add_program_group_record() {
        let mut header = Header::new(); // Create a new BAM header
        let command_line = "some command line options";

        // Call the function to add the program group record
        add_program_group_record(&mut header, command_line);

        let binding = header.to_hashmap();
        // Access the PG records directly
        let pg_records = binding.get("PG").expect("No PG records found");

        // Since we expect only one record, check the first one
        let record = &pg_records[0];
        assert_eq!(record["ID"], PKG_NAME);
        assert_eq!(record["PN"], PKG_NAME);
        assert_eq!(record["VN"], PKG_VERSION);
        assert_eq!(record["CL"], "some command line options");
    }
}
