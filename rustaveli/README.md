# Rustaveli
Rustaveli - A library to generate random C programs.

### Usage
View example code bellow:
```rs
use rustaveli::RandomCFile;

fn main() {
    let c_file = RandomCFile::new(3, 2);
    let code = c_file.finish();

    println!("Generated {} bytes.", code.len());
}
```