# rustcfuzz

use `cargo build` to build <br/> 
use `cargo run -- --help` for help

Options:

`-i, --input-dir` <INPUT_DIR> locate input directory path <br/> 
`-o, --output-dir` <OUTPUT_DIR> locate output directory path <br/> 
`-m, --mode` <MODE> 0: deletion only, 1: self splice mutation, 2: all file splice mutation <br/> 
`-f, --file-count` <FILE_COUNT> count of mutation for each seed file. if 0, then generate all possible mutation files <br/> 
`-h, --help` Print help

# example usage

for splicing 30 mutations for each seed <br/> 
`cargo run -- --input-dir example_data --output-dir ./out --mode 2 --file-count 30`

for splicing to different type with 10 mutations for each seed  <br/> 
`cargo run -- --input-dir example_data --output-dir ./out --mode 3 --file-count 10`

in case you want to splice code from itself <br/> 
`cargo run -- -i tests -o ./out -m 1 -f 20`

if you want to create all deletions from your seeds <br/> 
`cargo run -- -i example_data -o ./out -m 0 -f 0`
