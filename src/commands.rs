use anyhow::{anyhow, Result};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode(path: &PathBuf, chunk_type: ChunkType, data: &str) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(Path::new(path))
        .expect("Could not open .png file");
    let reader = BufReader::new(&file);
    let mut buf = Vec::new();
    for b in reader.bytes() {
        buf.push(b.unwrap())
    }
    let mut png = Png::try_from(buf.as_slice())?;

    for chunk in png.chunks().iter() {
        println!("{chunk:?}");
    }

    let chunk = Chunk::new(chunk_type, data.as_bytes().to_vec());
    png.append_chunk(chunk);

    for chunk in png.chunks().iter() {
        println!("{chunk:?}");
    }

    let mut file = File::create(path).expect("could not create file");
    file.write_all(&png.as_bytes())
        .expect("could not write to .png file");
    file.flush().unwrap();
    Ok(())
}

pub fn decode(path: &PathBuf, chunk_type: &str) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(Path::new(path))
        .expect("Could not open .png file");
    let reader = BufReader::new(&file);
    let mut buf = Vec::new();
    for b in reader.bytes() {
        buf.push(b.unwrap())
    }
    let png = Png::try_from(buf.as_slice())?;

    let data = match png.chunk_by_type(chunk_type) {
        Some(chunk) => chunk.data().to_vec(),
        None => return Err(anyhow!("No chunk of that type found in .png file")),
    };

    println!("{}", String::from_utf8(data).unwrap());
    Ok(())
}

pub fn remove(path: &PathBuf, chunk_type: &str) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(Path::new(path))
        .expect("Could not open .png file");
    let reader = BufReader::new(&file);
    let mut buf = Vec::new();
    for b in reader.bytes() {
        buf.push(b.unwrap())
    }
    let mut png = Png::try_from(buf.as_slice())?;

    png.remove_chunk(chunk_type)?;
    let mut file = File::create(path).expect("could not create file");
    file.write_all(&png.as_bytes())
        .expect("could not write to .png file");
    file.flush().unwrap();
    Ok(())
}

pub fn print(path: &PathBuf) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(Path::new(path))
        .expect("Could not open .png file");
    let reader = BufReader::new(&file);
    let mut buf = Vec::new();
    for b in reader.bytes() {
        buf.push(b.unwrap())
    }
    let png = Png::try_from(buf.as_slice())?;

    println!("{png:?}");
    Ok(())
}
