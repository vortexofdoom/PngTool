use crate::chunk_type::ChunkType;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct ProgArgs {
    #[clap(subcommand)]
    pub sub_command: PngArgs,
}

#[derive(Debug, Subcommand)]
pub enum PngArgs {
    /// Encode data into a .png image, giving its path, the chunk type, and the data to be encoded.
    Encode(EncodeArgs),
    /// Decode data from a .png image, giving its path and the chunk type, returns the data encoded, if found.
    Decode(DecodeArgs),
    /// Remove encoded data from a .png image, if present, given the path and chunk type.
    Remove(RemoveArgs),
    /// Prints all chunks in the .png file
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: ChunkType,
    pub data: String,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub path: PathBuf,
}
