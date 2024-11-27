use std::{error::Error, fs::File, io::Write};

use pico_args::Arguments;
use rustaveli::RandomCFile;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Arguments::from_env();

    if args.contains("--help") || args.contains("-h") {
        println!(
            "Usage: rustaveli --output <file> --function-count <count> --struct-count <count>
Generate random C programs.

Options:
      --output            Output file
      --function-count    Number of __attribute__((constructor)) functions to generate
      --struct-count      Number of structs to generate
  -h, --help              Show this help message
"
        );
        return Ok(());
    }

    let output: String = args.value_from_str("--output")?;
    let function_count: usize = args.value_from_str("--function-count")?;
    let struct_count: usize = args.value_from_str("--struct-count")?;

    let c_file = RandomCFile::new(function_count as u8, struct_count as u8);
    let mut file = File::create(&output)?;

    file.write_all(c_file.finish().as_bytes())?;

    println!("Finished writiing to '{output}'.");

    Ok(())
}
