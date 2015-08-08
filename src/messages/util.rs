use super::Direction;
use common_capnp;

impl Into<common_capnp::Direction> for Option<Direction> {
    fn into(self) -> common_capnp::Direction {
        match self {
            None => common_capnp::Direction::None,
            Some(Direction::West) => common_capnp::Direction::West,
            Some(Direction::South) => common_capnp::Direction::South,
            Some(Direction::East) => common_capnp::Direction::East,
            Some(Direction::North) => common_capnp::Direction::North,
        }
    }
}

impl From<common_capnp::Direction> for Option<Direction> {
    fn from(direction: common_capnp::Direction) -> Option<Direction> {
        match direction {
            common_capnp::Direction::None => None,
            common_capnp::Direction::West => Some(Direction::West),
            common_capnp::Direction::South => Some(Direction::South),
            common_capnp::Direction::East => Some(Direction::East),
            common_capnp::Direction::North => Some(Direction::North),
        }
    }
}

