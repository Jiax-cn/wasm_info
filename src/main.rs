use std::{
	fs::read,
    path::PathBuf,
};

use wasm_info::{
    custom::parse_name_sec,
};

use clap::{Parser,Subcommand,CommandFactory};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// parse wasm functions' names.
    Parse{
        /// wasm file with name section.
        input: PathBuf,
    },
}

fn main(){
    let args = Args::parse();

    match args.command {
        Some(Commands::Parse{input}) => {
            let bytes = read(input).unwrap();

            if let Some(name_map) = parse_name_sec(&bytes){
                for naming in name_map.into_iter() {
                    let naming = naming.unwrap();
                    println!("{} {}", naming.index, naming.name);
                }
            }
        },
        None => {
            let _ = Args::command().print_long_help();
        },
    }

    return;
}