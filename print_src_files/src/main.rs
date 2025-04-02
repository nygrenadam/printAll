// src/main.rs
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // Define the target directory relative to where the program is run.
    // Ensure you run this program from your game's root directory (the one containing 'src').
    let src_dir = Path::new("./src");

    println!("--- Reading files from: {:?} ---", src_dir.canonicalize()?); // Print absolute path for clarity
    println!("--------------------------------------\n");

    // Read the directory entries
    match fs::read_dir(src_dir) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let path = entry.path();

                        // Process only if it's a file
                        if path.is_file() {
                            // Get the filename
                            let file_name = path.file_name()
                                .map(|name| name.to_string_lossy()) // Convert OsStr to String (lossy)
                                .unwrap_or_else(|| path.to_string_lossy()); // Fallback to full path if no filename

                            println!("--- File: {} ---", file_name);

                            // Read the file content to a string
                            match fs::read_to_string(&path) {
                                Ok(contents) => {
                                    println!("{}", contents); // Print the file contents
                                }
                                Err(e) => {
                                    eprintln!("   [Error] Could not read file '{}': {}", file_name, e);
                                }
                            }
                            println!("\n--- End of File: {} ---\n", file_name); // Add separator
                        }
                        // Ignore directories, symlinks, etc.
                    }
                    Err(e) => {
                        eprintln!("[Error] Failed to process a directory entry: {}", e);
                        // Decide if you want to stop or continue on entry errors
                        // continue; // Uncomment to skip problematic entries
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[Error] Failed to read directory '{}': {}", src_dir.display(), e);
            eprintln!("Ensure you are running this program from the root directory of your game project (the parent directory of 'src').");
            // Return the error to stop the program
            return Err(e);
        }
    }

    println!("--------------------------------------");
    println!("--- Finished processing all files. ---");
    Ok(())
}