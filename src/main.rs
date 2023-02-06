use std::fs;
use csv::{ReaderBuilder, WriterBuilder};

/// The main function that reads in three tab-delimited text files and verifies the number of columns for each record.
/// If a record has more than 4 columns, it will be written to a corresponding verified .csv file.
/// The verified .csv file will be tab-delimited and use '|' as the quote character.
fn main() {
    // List of input files to be processed
    let input_files = vec!["ol_dump_authors_latest.txt", "ol_dump_works_latest.txt", "ol_dump_editions_latest.txt"];
    for input_file in input_files {
        let input = match fs::File::open(input_file) {
            Ok(file) => { file }
            Err(e) => {
                eprintln!("{} {}", e, input_file);
                continue;
            }
        };
        // Create a CSV reader with no headers and tab delimiter
        let mut csv_in = ReaderBuilder::new().has_headers(false)
            .delimiter(b'\t')
            .from_reader(input);

        let output_file_path = format!("{}_verified.csv", input_file.replace(".txt", ""));
        // Create a CSV writer with tab delimiter and '|' as the quote character
        let mut csv_out = match WriterBuilder::new().delimiter(b'\t').quote(b'|').from_path(output_file_path.clone()) {
            Ok(writer) => { writer }
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        for record in csv_in.records() {
            let record = match record {
                Ok(record) => { record }
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };
            // Only write the record to the output file if it has more than 4 columns
            if record.len() > 4 {
                match csv_out.write_record(&record) {
                    Ok(_) => {}
                    Err(e) => { eprintln!("{}", e) }
                };
            }
        }
        println!("File {} was successfully written", output_file_path)
    }
    println!("finish")
}
