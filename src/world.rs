use std::collections::HashSet;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Left,
    Right,
    Forward,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InstructionSet(Vec<Instruction>);

impl InstructionSet {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self(instructions)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn turn_left(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::West,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
            Orientation::West => Orientation::South,
        }
    }

    fn turn_right(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::East,
            Orientation::South => Orientation::West,
            Orientation::East => Orientation::South,
            Orientation::West => Orientation::North,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    location: Location,
    orientation: Orientation,
}

impl Position {
    pub fn new(location: Location, orientation: Orientation) -> Self {
        Self {
            location,
            orientation,
        }
    }
}

#[derive(Debug)]
pub struct SimResult {
    final_position: Position,
    is_lost: bool,
}

impl Display for SimResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let orientation_char = match self.final_position.orientation {
            Orientation::North => 'N',
            Orientation::South => 'S',
            Orientation::East => 'E',
            Orientation::West => 'W',
        };

        if self.is_lost {
            write!(
                f,
                "{} {} {} LOST",
                self.final_position.location.x, self.final_position.location.y, orientation_char
            )
        } else {
            write!(
                f,
                "{} {} {}",
                self.final_position.location.x, self.final_position.location.y, orientation_char
            )
        }
    }
}

/**
 * Represents the grid in which the robots operate.
 * 
 * ## Notes
 * 
 * Since the robots are processed sequentially and they don't interact with each other beyond the scent trail they leave,
 * we don't actually need to save any robot state, and we don't need to represent all the individual cells of the grid.
 * We only need to know the top corner to allow us to check for out-of-bounds moves, and we need to keep track of the 
 * scented coordinates where robots have been lost.
 * 
 * ### Scented Coordinates
 * 
 * There are several ways to keep track of scented locations. I chose to use a `HashSet<Location>` because:
 * - it allows for O(1) average time complexity for insertions and lookups
 * - it's straight forward to implement and understand
 * 
 * There are other ways of implementing this, depending on the constraints of the problem.
 * 
 * For example, since we know the maximum grid size is 50x50, we could use a 2D boolean array to represent the grid 
 * and mark scented locations. This would use a fixed amount of memory and allow for guaranteed O(1) lookups, 
 * but it would become impractical if we wanted to support very large grids.
 */
pub struct World {
    top_corner: Location,
    scented_coordinates: HashSet<Location>,
}

impl World {
    pub fn new(top_corner: Location) -> Self {
        Self {
            top_corner,
            scented_coordinates: HashSet::new(),
        }
    }

    pub fn simulate(
        &mut self,
        starting_position: Position,
        instructions: InstructionSet,
    ) -> SimResult {
        let mut current_position = starting_position;
        let mut is_lost = false;

        for instruction in instructions.0 {
            match instruction {
                Instruction::Left => {
                    current_position.orientation = current_position.orientation.turn_left();
                }
                Instruction::Right => {
                    current_position.orientation = current_position.orientation.turn_right();
                }
                Instruction::Forward => {
                    if let Some(new_location) = self.neighbour(&current_position.location, current_position.orientation) {
                        current_position.location = new_location;
                    } else {
                        // If there was no scent at the current location and the robot is about to move out of bounds, 
                        // it gets lost and leaves a scent. Otherwise, it ignores the instruction and continues.
                        if !self
                            .scented_coordinates
                            .contains(&current_position.location)
                        {
                            self.scented_coordinates
                                .insert(current_position.location.clone());
                            is_lost = true;
                            break;
                        }
                    }
                }
            }
        }

        SimResult {
            final_position: current_position,
            is_lost,
        }
    }

    fn neighbour(&self, location: &Location, orientation: Orientation) -> Option<Location> {
        match orientation {
            Orientation::North if location.y < self.top_corner.y => {
                Some(Location {
                    x: location.x,
                    y: location.y + 1,
                })
            },
            Orientation::South if location.y > 0 => {
                Some(Location {
                    x: location.x,
                    y: location.y - 1,
                })
            },
            Orientation::East if location.x < self.top_corner.x => {
                Some(Location {
                    x: location.x + 1,
                    y: location.y,
                })
            },
            Orientation::West if location.x > 0 => {
                Some(Location {
                    x: location.x - 1,
                    y: location.y,
                })
            },
            _ => None,
        }
    }
}

mod test {
    #[allow(unused)]
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Orientation::North, Orientation::West, Orientation::East)]
    #[case(Orientation::South, Orientation::East, Orientation::West)]
    #[case(Orientation::East, Orientation::North, Orientation::South)]
    #[case(Orientation::West, Orientation::South, Orientation::North)]
    fn test_orientation_turning(
        #[case] start_orientation: Orientation,
        #[case] turn_left: Orientation,
        #[case] turn_right: Orientation,
    ) {
        let left = start_orientation.turn_left();
        let right = start_orientation.turn_right();
        assert_eq!(left, turn_left);
        assert_eq!(right, turn_right);
    }

    #[rstest]
    #[case(Location { x: 1, y: 1 }, Orientation::North, Some(Location { x: 1, y: 2 }))]
    #[case(Location { x: 1, y: 1 }, Orientation::South, Some(Location { x: 1, y: 0 }))]
    #[case(Location { x: 1, y: 1 }, Orientation::East, Some(Location { x: 2, y: 1 }))]
    #[case(Location { x: 1, y: 1 }, Orientation::West, Some(Location { x: 0, y: 1 }))]
    #[case(Location { x: 0, y: 0 }, Orientation::South, None)]
    #[case(Location { x: 0, y: 0 }, Orientation::West, None)]
    #[case(Location { x: 2, y: 2 }, Orientation::North, None)]
    #[case(Location { x: 2, y: 2 }, Orientation::East, None)]
    fn test_world_neighbour(
        #[case] location: Location,
        #[case] orientation: Orientation,
        #[case] expected_neighbor: Option<Location>,
    ) {
        let world = World::new(Location { x: 2, y: 2 });

        assert_eq!(world.neighbour(&location, orientation), expected_neighbor);
    }

    #[rstest]
    fn test_simulation_matches_sample() {
        let mut world = World::new(Location { x: 5, y: 3 });

        let first_result = world.simulate(
            Position::new(Location { x: 1, y: 1 }, Orientation::East),
            InstructionSet::new(vec![
                Instruction::Right,
                Instruction::Forward,
                Instruction::Right,
                Instruction::Forward,
                Instruction::Right,
                Instruction::Forward,
                Instruction::Right,
                Instruction::Forward,
            ]),
        );
        let second_result = world.simulate(
            Position::new(Location { x: 3, y: 2 }, Orientation::North),
            InstructionSet::new(vec![
                Instruction::Forward,
                Instruction::Right,
                Instruction::Right,
                Instruction::Forward,
                Instruction::Left,
                Instruction::Left,
                Instruction::Forward,
                Instruction::Forward,
                Instruction::Right,
                Instruction::Right,
                Instruction::Forward,
                Instruction::Left,
                Instruction::Left,
            ]),
        );
        let third_result = world.simulate(
            Position::new(Location { x: 0, y: 3 }, Orientation::West),
            InstructionSet::new(vec![
                Instruction::Left,
                Instruction::Left,
                Instruction::Forward,
                Instruction::Forward,
                Instruction::Forward,
                Instruction::Left,
                Instruction::Forward,
                Instruction::Left,
                Instruction::Forward,
                Instruction::Left,
            ]),
        );

        assert_eq!(first_result.to_string(), "1 1 E");
        assert_eq!(second_result.to_string(), "3 3 N LOST");
        assert_eq!(third_result.to_string(), "2 3 S");
    }

    #[test]
    fn test_scent_prevents_later_robot_from_being_lost() {
        let mut world = World::new(Location { x: 1, y: 1 });

        let first_result = world.simulate(
            Position::new(Location { x: 0, y: 0 }, Orientation::East),
            InstructionSet::new(vec![Instruction::Forward, Instruction::Forward]),
        );
        let second_result = world.simulate(
            Position::new(Location { x: 1, y: 1 }, Orientation::South),
            InstructionSet::new(vec![Instruction::Forward, Instruction::Forward]),
        );

        assert_eq!(first_result.is_lost, true);
        assert_eq!(second_result.is_lost, false);
    }

    #[test]
    fn test_scent_does_not_help_if_robot_is_lost_in_different_direction() {
        let mut world = World::new(Location { x: 1, y: 1 });

        let first_result = world.simulate(
            Position::new(Location { x: 0, y: 0 }, Orientation::East),
            InstructionSet::new(vec![Instruction::Forward, Instruction::Forward]),
        );
        let second_result = world.simulate(
            Position::new(Location { x: 0, y: 0 }, Orientation::North),
            InstructionSet::new(vec![Instruction::Forward, Instruction::Forward]),
        );

        assert_eq!(first_result.is_lost, true);
        assert_eq!(second_result.is_lost, true);
    }
}
