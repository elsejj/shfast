extern crate chrono;
extern crate encoding_rs;
extern crate protobuf;

use std::fs::File;
use std::io::{Result, Read};
use std::path::Path;
use mktdt::{MktdtParser};

mod mktdt;

mod ftcodec;

fn read_file<P: AsRef<Path>>(file_name: P, buff: &mut Vec<u8>) -> Result<usize> {
    let mut file = File::open(file_name)?;
    let meta = file.metadata()?;
    let file_size = meta.len() as usize;


    if buff.len() < file_size {
        buff.resize(file_size, 0);
    }
    let (left, _) = buff.split_at_mut(file_size);
    file.read_exact(left)?;
    Ok(file_size)
}

fn main() -> Result<()>{

    let file_names = [
        "data/mktdt00.txt",
        "data/mktdt00.txt",
    ];

    let mut buff: Vec<u8> = Vec::new();
    let mut parser = MktdtParser::default();


    for file_name in file_names.iter() {
        let t1 = chrono::Local::now();
        let size = read_file(file_name, &mut buff)?;
        let (data, _) = buff.split_at(size);
        parser.parse_file(data).unwrap();
        let t2 = chrono::Local::now();

        let span = t2 - t1;
        println!("parse used: {:?} ms", span.num_milliseconds());
    }
    Ok(())
}
