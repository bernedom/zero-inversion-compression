use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;

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
    let mut target = Vec::new();

    for byte in data {
        target.push(byte);
    }

    let output_file = input_file.to_owned() + ".zic";
    fs::write(output_file, target).unwrap();
}
