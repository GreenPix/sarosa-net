mod serialize;
mod deserialize;
mod util;

pub use self::serialize::serialize;
pub use self::serialize::forge_authentication_tokens;
pub use self::serialize::send_authentication_token;
pub use self::serialize::AuthenticationToken;
pub use self::deserialize::deserialize;
pub use self::deserialize::deserialize_error_code;

#[derive(Debug,Clone,Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct TargettedOrder {
    pub target: u64,
    pub order: Order,
}

#[derive(Debug)]
pub enum Order {
    Walk(Option<Direction>),
    Say(String),
    // Attack
    // Cast spell
    // Talk
    // Exchange
    // ...
}

#[derive(Debug)]
pub enum Notification {
    Walk {
        entity: u64,
        orientation: Option<Direction>,
    },
    Say {
        entity: u64,
        message: String,
    },
    Location {
        entity: u64,
        x: f32,
        y: f32,
    },
    ThisIsYou(u64),
}

impl Notification {
    pub fn walk(id: u64, orientation: Option<Direction>) -> Notification {
        Notification::Walk {
            entity: id,
            orientation: orientation,
        }
    }

    pub fn say(id: u64, message: String) -> Notification {
        Notification::Say {
            entity: id,
            message: message,
        }
    }

    pub fn location(id: u64, x: f32, y: f32) -> Notification {
        Notification::Location {
            entity: id,
            x: x,
            y: y,
        }
    }

    pub fn this_is_you(id: u64) -> Notification {
        Notification::ThisIsYou(id)
    }
}
