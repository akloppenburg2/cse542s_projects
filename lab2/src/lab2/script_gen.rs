use {
    super::declarations::GEN_SCRIPT_ERR,
    std::fs::File,
    std::io::{BufReader, BufRead},
};

// Function to grab and trim lines from a file
pub fn grab_trimmed_file_lines(file_name: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    // Try to open the file, and if it fails, return an error
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Could not open file '{}'", file_name);
            return Err(GEN_SCRIPT_ERR);
        }
    };

    // Initialize a BufReader and a String to hold each line
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    // Loop to read each line from the file
    loop {
        line.clear();  // Clear the buffer before reading the next line

        match reader.read_line(&mut line) {
            Ok(0) => return Ok(()),  // End of file reached, return success
            Ok(_) => {
                // Trim the line and push it into the vector if it's not empty
                let trimmed_line = line.trim().to_string();
                if !trimmed_line.is_empty() {
                    lines.push(trimmed_line);
                }
            }
            Err(_) => {
                eprintln!("Error: Could not read line from file '{}'", file_name);
                return Err(GEN_SCRIPT_ERR);  // Return error if reading fails
            }
        }
    }
}
