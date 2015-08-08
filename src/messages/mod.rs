mod serialize;
mod deserialize;
mod util;

pub use self::serialize::serialize;
pub use self::serialize::fake_authentication_token;
pub use self::deserialize::deserialize;

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
    Walk(WalkNotification),
    Say(SayNotification),
}

#[derive(Debug)]
pub struct WalkNotification {
    origin: u64,
    orientation: Option<Direction>,
}

#[derive(Debug)]
pub struct SayNotification {
    origin: u64,
    message: String,
}

impl Notification {
    pub fn walk(id: u64, orientation: Option<Direction>) -> Notification {
        Notification::Walk(WalkNotification {
            origin: id,
            orientation: orientation,
        })
    }

    pub fn say(id: u64, message: String) -> Notification {
        Notification::Say(SayNotification {
            origin: id,
            message: message,
        })
    }
}
