# rustcfuzz

use `cargo build` to build
use `cargo run -- --help` for help

Options:
-i, --input-dir <INPUT_DIR> locate input directory path
-o, --output-dir <OUTPUT_DIR> locate output directory path
-m, --mode <MODE> 0: deletion only, 1: self splice mutation, 2: all file splice mutation
-f, --file-count <FILE_COUNT> count of mutation for each seed file. if 0, then generate all possible mutation files
-h, --help Print help

# example usage

for splicing 30 mutations for each seed
`cargo run -- --input-dir example_data --output-dir ./out --mode 2 --file-count 30`

in case you want to splice code from itself
`cargo run -- -i example_data -o ./out -m 1 -f 20`

if you want to create all deletions from your seeds
`cargo run -- -i example_data -o ./out -m 0 -f 0`
