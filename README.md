<img align="right" src="zic-logo.png">

# zero-inversion-compression

Zero-Inversion-Compression (zic) is a non-lossless file compression algorithm with compression rate up to 100% of the original file size. 

```
cargo build
cargo run README.md

>    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
>     Running `target/debug/zero-inversion-compression README.md`

Compressed file 'README.md' from 1390 bytes to 'README.md.zic' (0 bytes) 
(100% compression rate)

```

# Algorithm

zic reduces the file size by discarding any bits of a file that do not contain valuable information, thus reducing the hamming weight of a file.  
Files are represented in binary form as a series of 0 and 1. As 0 has no value, these bits can be discarded, leaving only the valuable bits in a file. While this alone might yield good compression results further optimization is achieved by inverting the compressed file and running the algorithm twice. 

## Options for further improvement

* Running the two compression runs in parallel
* Evening out the hamming weight of the file before running the algorithm
* Compress the two loops into one by inverting the bits in place
* Apply the [NGGYU-Algorithm](https://www.youtube.com/watch?v=dQw4w9WgXcQ) for sanitizing the input.

## Contributing

Feel free to fork and put up a pull request for any improvements. A good starting point for various improvements are the `@todos` in the code. 
