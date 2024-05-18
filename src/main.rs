use std::error::Error;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use indicatif::{ProgressBar, ProgressStyle};

const ROWS_PER_FILE: usize = 3600 * 20;         // 72000 rows at once
// const BUFFER_SIZE: usize = 10 * 1024 * 1024;    // 10MB buffer size

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    //! Open the input CSV file
    
    let input_file = File::open("F:\\Super\\CSV\\Dataset\\N039E125.csv").await?;
    let reader = BufReader::new(input_file);
    // let mut reader = BufReader::with_capacity(BUFFER_SIZE, input_file);
    
    let mut counter = 1;
    let mut file_counter = 1;
    
    // Create a vector to hold tasks
    let mut tasks = Vec::new();
    
    // Create a buffer to store lines of each batch
    let mut batch_lines = Vec::with_capacity(ROWS_PER_FILE);
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
    while let Some(line) = line_stream.next_line().await? {
        batch_lines.push(line.clone());
        
        // If the batch size is reached, save the batch and reset for the next batch
        if counter % ROWS_PER_FILE == 0 {
            // Process batch of lines asynchronously
            let task = save_batch_to_file(batch_lines.clone());
            file_counter += 1;
            tasks.push(task);

            batch_lines.clear();
        }
        
        counter += 1;
    }

    // Process remaining lines
    if !batch_lines.is_empty() {
        println!("Remaining: {}", batch_lines.len());
        let task = save_batch_to_file(batch_lines);
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await?;
        println!("Finished..")
    }

    // let mut contents = vec![];

    println!("Counter: {}, File_Counter: {}", counter, file_counter);

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
    let rect_number = lines.len() / 20 / 20;

    if lines.len() > 0 {
        let smaller_batches: Vec<Vec<String>> = lines.chunks(20).map(|chunk| chunk.to_vec()).collect();
        
        let mut temp_rect_batches: Vec<Vec<&String>> = Vec::with_capacity(rect_number);
        for _ in 0..rect_number {
            temp_rect_batches.push(Vec::new());
        }

        for (i, batch) in smaller_batches.iter().enumerate() {
            temp_rect_batches[i % rect_number].extend(batch);
        }

        for temp_batch in &temp_rect_batches {
            let first_line = temp_batch[0].split_whitespace().collect::<Vec<&str>>();
            let last_line = temp_batch.last().unwrap().split_whitespace().collect::<Vec<&str>>();
            let file_name = format!(
                "dataset\\{}N_{}E_{}N_{}E.txt",
                convert_to_dms(first_line[1])?,
                convert_to_dms(first_line[0])?,
                convert_to_dms(last_line[1])?,
                convert_to_dms(last_line[0])?
            );

            let mut output_file = File::create(file_name).await?;

            for line in temp_batch {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    // Reorder the parts of longitude, latitude, altitude format
                    let reordered_line = format!(
                        "{} {} {}\n",
                        parts[1],
                        parts[0],
                        parts[2],
                    );
                    output_file.write_all(reordered_line.as_bytes()).await?;
                }
            }
            output_file.flush().await?;
        }
    }

    Ok(())
}