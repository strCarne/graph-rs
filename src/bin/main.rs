use std::fs;

use clap::{Arg, Command};
use graph_rs::{graph::Graph, tgf::TgfConvertible};

const INPUT_FILE_NAME_ARG: &str = "file_name";

fn main() -> std::io::Result<()> {
    let matches = Command::new("graph-rs")
        .version("1.0")
        .author("Semion Voevoda semionvoevoda@gmail.com")
        .about(concat!(
            "Graph-rs is a CLI tool for reading from tgf files and then ",
            "outputing vertecies IDs with its values and adjacent vertecies IDs"
        ))
        .arg(
            Arg::new(INPUT_FILE_NAME_ARG)
                .required(true)
                .help("tgf file name"),
        )
        .get_matches();

    // unsafe block is okay, because if this arg is not specified, it will panic before this block
    let file_name: &String = unsafe { matches.get_one(INPUT_FILE_NAME_ARG).unwrap_unchecked() };

    let tgf_raw = fs::read_to_string(file_name)?;

    let graph: Graph<i32, String> = match Graph::from_tgf(tgf_raw.into()) {
        Ok(graph) => graph,
        Err(err) => panic!("error: {}", err),
    };

    println!("{}", graph);

    Ok(())
}
