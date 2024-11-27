# Rustaveli
Rustaveli - generate random c programs

## Usage
```
usage: rustaveli [-h] -o OUTPUT -f FUNCTION_COUNT -s STRUCT_COUNT

Generate random c programs

options:
  -h, --help            show this help message and exit
  -o OUTPUT, --output OUTPUT
                        Output file
  -f FUNCTION_COUNT, --function-count FUNCTION_COUNT
                        Number of __attribute__((constructor)) functions to generate
  -s STRUCT_COUNT, --struct-count STRUCT_COUNT
                        Number of structs to generate
```

## Building
```
cargo build --release
```

## License
This code is released under the [MIT License](./LICENSE)