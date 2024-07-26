use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli{
  //The pattern to look for
  pattern: String,
  //The path to the file to read
  path: String //path: std::path::PathBuf
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args:Cli = Cli::parse();

   if let Ok(lines) = read_lines(&args.path) {
    for line in lines.flatten() {
      if line.contains(&args.pattern){
        println!("{}",line);
      }
    }
}

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
