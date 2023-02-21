use args::ProgArgs;
use anyhow::Result;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> Result<()> {
    let args = ProgArgs::parse();
    match args.sub_command {
        args::PngArgs::Encode(args) => commands::encode(&args.path, args.chunk_type, &args.data),
        args::PngArgs::Decode(args) => commands::decode(&args.path, &args.chunk_type),
        args::PngArgs::Remove(args) => commands::remove(&args.path, &args.chunk_type),
        args::PngArgs::Print(args) => commands::print(&args.path),
    }
}
