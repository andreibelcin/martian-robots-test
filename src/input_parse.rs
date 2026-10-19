use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

use crate::world::{Instruction, InstructionSet, Location, Orientation, Position};

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Invalid character '{0}' in instruction set")]
    BadInstruction(char),
    #[error("Failed to parse integer: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Invalid orientation: {0}")]
    InvalidOrientation(String),
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub input_path: PathBuf,
}

impl TryFrom<char> for Instruction {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Instruction, ParseError> {
        match c {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            'F' => Ok(Instruction::Forward),
            _ => Err(ParseError::BadInstruction(c)),
        }
    }
}

impl FromStr for InstructionSet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<InstructionSet, ParseError> {
        Ok(InstructionSet::new(
            s.chars()
                .map(Instruction::try_from)
                .collect::<Result<Vec<Instruction>, ParseError>>()?,
        ))
    }
}

impl FromStr for Location {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Location, ParseError> {
        let coords = s
            .split_whitespace()
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;

        if coords.len() != 2 {
            return Err(ParseError::InvalidFormat(s.to_string()));
        }

        let x = coords[0];
        let y = coords[1];

        Ok(Location { x, y })
    }
}

impl FromStr for Position {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Position, ParseError> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(ParseError::InvalidFormat(s.to_string()));
        }

        let location = Location {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
        };

        let orientation = match parts[2] {
            "N" => Orientation::North,
            "S" => Orientation::South,
            "E" => Orientation::East,
            "W" => Orientation::West,
            _ => return Err(ParseError::InvalidOrientation(s.to_string())),
        };

        Ok(Position::new(location, orientation))
    }
}

mod test {
    #[allow(unused)]
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1 2", Ok(Location { x: 1, y: 2 }))]
    #[case("55 78", Ok(Location { x: 55, y: 78 }))]
    #[case("1", Err(ParseError::InvalidFormat("1".to_string())))]
    #[case("1 a", Err(ParseError::ParseIntError("a".parse::<i32>().unwrap_err())))]
    #[case("1 2 3", Err(ParseError::InvalidFormat("1 2 3".to_string())))]
    #[case("", Err(ParseError::InvalidFormat("".to_string())))]
    #[case("1,2", Err(ParseError::ParseIntError("1,2".parse::<i32>().unwrap_err())))]
    fn test_location_parsing(
        #[case] input: &'static str,
        #[case] expected: Result<Location, ParseError>,
    ) {
        let location = Location::from_str(input);
        assert_eq!(location, expected);
    }

    #[rstest]
    #[case("1 2 N", Ok(Position::new(Location { x: 1, y: 2 }, Orientation::North)))]
    #[case("1 2 S", Ok(Position::new(Location { x: 1, y: 2 }, Orientation::South)))]
    #[case("1 2 E", Ok(Position::new(Location { x: 1, y: 2 }, Orientation::East)))]
    #[case("1 2 W", Ok(Position::new(Location { x: 1, y: 2 }, Orientation::West)))]
    #[case("N 1 2", Err(ParseError::ParseIntError("N".parse::<i32>().unwrap_err())))]
    #[case("1 2 X", Err(ParseError::InvalidOrientation("1 2 X".to_string())))]
    #[case("1 2 N E", Err(ParseError::InvalidFormat("1 2 N E".to_string())))]
    #[case("1 2", Err(ParseError::InvalidFormat("1 2".to_string())))]
    #[case("1,2,N", Err(ParseError::InvalidFormat("1,2,N".to_string())))]
    fn test_position_parsing(
        #[case] input: &'static str,
        #[case] expected: Result<Position, ParseError>,
    ) {
        let position = Position::from_str(input);
        assert_eq!(position, expected);
    }

    #[rstest]
    #[case('L', Ok(Instruction::Left))]
    #[case('R', Ok(Instruction::Right))]
    #[case('F', Ok(Instruction::Forward))]
    #[case('X', Err(ParseError::BadInstruction('X')))]
    fn test_instruction_parsing(
        #[case] input: char,
        #[case] expected: Result<Instruction, ParseError>,
    ) {
        let instruction_set = Instruction::try_from(input);
        assert_eq!(instruction_set, expected);
    }

    #[rstest]
    #[case("L", Ok(InstructionSet::new(vec![Instruction::Left])))]
    #[case("LFR", Ok(InstructionSet::new(vec![Instruction::Left, Instruction::Forward, Instruction::Right])))]
    #[case("L R", Err(ParseError::BadInstruction(' ')))]
    #[case("LFX", Err(ParseError::BadInstruction('X')))]
    #[case("", Ok(InstructionSet::new(vec![])))]
    fn test_instruction_set_parsing(
        #[case] input: &'static str,
        #[case] expected: Result<InstructionSet, ParseError>,
    ) {
        let instruction_set = InstructionSet::from_str(input);
        assert_eq!(instruction_set, expected);
    }
}
