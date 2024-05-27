pub mod file_handler {
    use std::{
        fs::{self, OpenOptions},
        io::{BufRead, BufReader, Write},
        path::PathBuf,
    };

    pub fn append_line(file: &PathBuf, line: String) -> Result<(), String> {
        let mut oo_file = match OpenOptions::new()
            .read(false)
            .write(false)
            .append(true)
            .create(false)
            .open(file)
        {
            Ok(file) => file,
            Err(e) => panic!("failed to open file! {}", e),
        };

        if let Err(e) = writeln!(oo_file, "{}", line) {
            eprintln!("Coldn't write to file: {}", e);
        }
        Ok(())
    }

    // I got this staight from GPT4
    // maby i dont know how it works but it  kinda right
    pub fn remove_line(file: &PathBuf, index: usize) -> Result<(), String> {
        // Read lines into a buffer
        let input = fs::File::open(file).map_err(|e| format!("Failed to open the file: {}", e))?;
        let buffered = BufReader::new(input);
        let mut lines: Vec<String> = buffered
            .lines()
            .collect::<Result<_, _>>()
            .map_err(|e| format!("Failed to read lines: {}", e))?;

        // Remove the line if index is within bounds
        if index < lines.len() {
            lines.remove(index);
        } else {
            return Err("Index out of bounds".to_string());
        }

        // Write back to the file
        let mut output = fs::File::create(file)
            .map_err(|e| format!("Failed to open the file for writing: {}", e))?;
        for line in lines {
            writeln!(output, "{}", line).map_err(|e| format!("Failed to write to file: {}", e))?;
        }

        Ok(())
    }
}
