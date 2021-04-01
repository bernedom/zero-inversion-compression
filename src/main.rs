extern crate bitstream_io;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read};

use bitstream_io::{BigEndian, BitRead, BitReader, BitWrite, BitWriter};

// Compress the data in the input vector by discarding any 0 bits
fn compress(data: &Vec<u8>) -> Vec<u8> {
    let mut target = Vec::new();

    // @todo consider using LittleEndian for marketing purposes as it sounds smaller
    let mut writer = BitWriter::endian(&mut target, BigEndian);
    let mut cursor = Cursor::new(&data);
    {
        let mut reader = BitReader::endian(&mut cursor, BigEndian);
        let mut bits_written = 0;
        for _i in 0..data.len() * 8 {
            let v: bool = reader.read_bit().unwrap();
            if v {
                writer.write_bit(v).unwrap();
                bits_written += 1;
            }
        }
        // pad up to a full byte if any bits were written at all
        // @todo alternatively discading any half bytes could be an optimization
        if bits_written > 0 {
            for _i in 0..8 - bits_written % 8 {
                writer.write_bit(true).unwrap();
            }
        }
    }
    target
}

// invert the data in a vector
fn invert(data: &mut Vec<u8>) {
    for i in data {
        *i = !*i;
    }
}

// Read a file into a data vector.
// @todo optimize so zic can handle files that are larger than the available memory size
fn read_into_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect(&format!("File {} not found", filename));
    let metadata = std::fs::metadata(&filename)
        .expect(&format!("Unable to read meta data from file {}", filename));
    let mut buf = vec![0; metadata.len() as usize];
    f.read(&mut buf)
        .expect("Could not read file contents into buffer");

    buf
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: zero-inversion-compression <filename>");
        return;
    }

    let input_file = &args[1];
    let data = read_into_vec(input_file);

    // Main algorithm starts here.
    // @todo by applying a heuristic measurement to check if there are more 1 than 0 bits
    // and then inverting first could yield a steeper compression gradient
    let mut target = compress(&data);
    invert(&mut target);
    // Use some rust magic to avoid thinking of any more variable names
    let target = compress(&target);

    let output_file = input_file.to_owned() + ".zic";
    println!(
        "Compressed file '{}' from {} bytes to {} bytes in '{}'",
        input_file,
        data.len(),
        target.len(),
        output_file
    );
    fs::write(output_file, target).unwrap();
}
