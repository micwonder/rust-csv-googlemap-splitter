use std::error::Error;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use indicatif::{ProgressBar, ProgressStyle};

const ROWS_LINE: usize = 3600;
const RECT_SIZE: usize = 20;
const ROWS_PER_FILE: usize = ROWS_LINE * RECT_SIZE;         // 72000 rows at once
// const BUFFER_SIZE: usize = 10 * 1024 * 1024;    // 10MB buffer size

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // Start the timer
    let start_time = Instant::now();

    // Open the input CSV file
    let input_file = File::open("F:\\Super\\CSV\\Dataset\\N039E125.csv").await?;
    let reader = BufReader::new(input_file);
    // let mut reader = BufReader::with_capacity(BUFFER_SIZE, input_file);
    
    let mut counter = 0;
    let mut file_counter = 1;
    
    // Create a vector to hold tasks
    let mut tasks = Vec::new();
    
    // Create a buffer to store lines of each batch
    let mut batch_lines = Vec::with_capacity(ROWS_PER_FILE);
    let mut rows_lines = Vec::with_capacity(ROWS_LINE);
    // let mut output_file = AsyncFile::create(format!("output{}.csv", file_counter)).await?;
    // Initialize the progress bar
    // let pb = ProgressBar::new_spinner();
    // pb.set_style(ProgressStyle::default_spinner()
    //     .template("{spinner:.green} [{elapsed}] {wide_msg}")
    //     .unwrap()
    // );
    // pb.set_message("Processing lines...");
    
    // Read input CSV file line by line and split into multiple files

    // while let Some(line) = reader.next_line().await? {
    // while reader.read_line(&mut line).await? > 0 {    
    // Read lines asynchronously
    let mut line_stream = reader.lines();
    // while let Some(line) = line_stream.next_line().await? {
    println!("Get started ...");
    loop {
        // Read ROWS_LINE lines
        for _ in 0..ROWS_LINE {
            if let Some(line) = line_stream.next_line().await? {
                rows_lines.push(line);
            } else {
                break;
            }
        }

        // If file reading reaches to EOF, break the loop
        if rows_lines.len() < ROWS_LINE {
            let len = rows_lines.len();
            if len > 0 {
                for _ in rows_lines.len()..ROWS_LINE {
                    rows_lines.push(rows_lines[len - 1].clone());
                }
            }
            break;
        }

        batch_lines.extend(rows_lines.clone());
        // If the batch size is reached, save the batch and reset for the next batch
        if counter % RECT_SIZE == 0 && counter != 0 {
            println!("{}th length: {}", file_counter, batch_lines.len());
            // Process batch of lines asynchronously
            let task = save_batch_to_file(batch_lines.clone());
            file_counter += 1;
            tasks.push(task);

            batch_lines.clear();
            batch_lines.extend(rows_lines.clone());
        }
        
        rows_lines.clear();
        counter += 1;
    }

    // Process remaining lines
    if !batch_lines.is_empty() {
        let mut last_rows_lines = rows_lines.clone();
        if last_rows_lines.len() == 0 {
            last_rows_lines = if batch_lines.len() >= ROWS_LINE {
                batch_lines[batch_lines.len() - ROWS_LINE..].to_vec()
            } else {
                batch_lines.clone() // This might occur an issue when batch_line length is smaller than ROWS_LINES
            };
        }

        for _ in batch_lines.len() / ROWS_LINE..RECT_SIZE + 1 {
            batch_lines.extend(last_rows_lines.clone());
        }
        println!("{}th length: {}", file_counter, batch_lines.len());

        let task = save_batch_to_file(batch_lines);
        tasks.push(task);
    }

    // Wait for all tasks to complete
    let mut completion_counter = 0;
    for task in tasks {
        task.await?;
        println!("{}th task was completed...", completion_counter);
        completion_counter += 1;
    }

    println!("Counter: {}, File_Counter: {}", counter, file_counter);

    // Print the elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:?}", elapsed_time);

    Ok(())
}

// Function to convert decimal degress to degrees, minutes, seconds format
fn convert_to_dms(decimal_degrees_str: &str) -> Result<String, Box<dyn Error>>{
    // Attempt to parse the input string into a f64
    let decimal_degrees: f64 = decimal_degrees_str.parse::<f64>()?;
    
    // Convert the parsed decimal degrees to degrees, minutes, seconds format
    let degrees = decimal_degrees.trunc() as i32;
    let minutes_raw = (decimal_degrees.abs() - degrees as f64) * 60.0;
    let minutes = minutes_raw.trunc() as i32;
    let seconds = (minutes_raw - minutes as f64) * 60.0;

    // Format the result string
    let result = format!("{}°{}'{}", degrees, minutes, seconds);

    Ok(result)
}

async fn save_batch_to_file(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let rect_number = lines.len() / (RECT_SIZE + 1) / RECT_SIZE;

    if lines.len() > 0 {
        let mut processing_start_time = Instant::now();

        let mut rect_batches: Vec<Vec<String>> = Vec::with_capacity(rect_number);
        for _ in 0..rect_number {
            rect_batches.push(Vec::new());
        }

        println!("Time initialization: {:?}", processing_start_time.elapsed());
        processing_start_time = Instant::now();

        for col in 0..rect_number {
            for row in 0..RECT_SIZE + 1 {
                let start_idx = col * 20;
                let end_idx = (col + 1) * 20;
                if col == rect_number - 1 {
                    rect_batches[col].extend(lines[start_idx + row * ROWS_LINE..end_idx + row * ROWS_LINE].to_vec());
                } else {
                    rect_batches[col].extend(lines[start_idx + row * ROWS_LINE..end_idx + 1 + row * ROWS_LINE].to_vec());
                }
            }
        }

        println!("Time extend: {:?}", processing_start_time.elapsed());
        
        for batch in &rect_batches {
            processing_start_time = Instant::now();

            let first_line: Vec<&str> = batch[0].split_whitespace().collect::<Vec<&str>>();
            let last_line: Vec<&str> = batch.last().unwrap().split_whitespace().collect::<Vec<&str>>();
            let file_name = format!(
                "dataset\\{}N_{}E_{}N_{}E.txt",
                convert_to_dms(first_line[1])?,
                convert_to_dms(first_line[0])?,
                convert_to_dms(last_line[1])?,
                convert_to_dms(last_line[0])?
            );

            // Generate content to save to file
            let mut content = String::new();

            
            for line in batch {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    // Reorder the parts of longitude, latitude, altitude format
                    let reordered_line = format!(
                        "{} {} {}\n",
                        parts[1],
                        parts[0],
                        parts[2],
                    );
                    content.push_str(&reordered_line);
                }
            }
            println!("Time push str: {:?}", processing_start_time.elapsed());
            processing_start_time = Instant::now();

            let mut output_file = File::create(file_name).await?;
            output_file.write_all(content.as_bytes()).await?;
            output_file.flush().await?;

            println!("Time writing: {:?}", processing_start_time.elapsed());
        }
    }

    Ok(())
}