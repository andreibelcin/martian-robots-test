use std::io::BufRead;

use clap::Parser;
use thiserror::Error;

use input_parse::Args;

mod input_parse;
mod world;

#[derive(Debug, Error)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] input_parse::ParseError),
    #[error("Unexpected end of input")]
    UnexpectedEof,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let file = std::fs::File::open(&args.input_path)?;
    let reader = std::io::BufReader::new(file);
    let mut line_reader = reader.lines();

    let top_corner = line_reader
        .next()
        .ok_or(Error::UnexpectedEof)??
        .parse::<world::Location>()?;

    let mut world = world::World::new(top_corner);

    while let Some(position_line) = line_reader.next() {
        let position_line = position_line?;
        let position = position_line.parse::<world::Position>()?;

        let commands_line = line_reader
            .next()
            .ok_or(Error::UnexpectedEof)??
            .parse::<world::InstructionSet>()?;

        let sim_result = world.simulate(position, commands_line);
        println!("{sim_result}");
    }

    Ok(())
}
