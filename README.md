# tagbam
Tag reads in BAM file

# Usage
```
Usage: tagbam [OPTIONS] --input <INPUT> --tag <TAG> --value <VALUE> --output-file <OUTPUT_FILE>

Options:
  -i, --input <INPUT>              Input BAM file
  -t, --threads <THREADS>          Number of parallel decompression & writer threads to use [default: 4]
      --tag <TAG>                  Tag to add (must be 1-2 characters)
  -v, --value <VALUE>              Value to add
  -o, --output-file <OUTPUT_FILE>  Output file
  -c, --compression <COMPRESSION>  BAM output compression level [default: 6]
  -h, --help                       Print help
  -V, --version                    Print version
```
# Example
```
tagbam --tag HP --value 1 --input in.bam --output-file out.bam
```
