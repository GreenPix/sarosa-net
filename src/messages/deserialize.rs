use std::io::Read;

use capnp::serialize;
use capnp::Error;
use capnp::message::{ReaderOptions,Reader};

use super::Notification;
use notifications_capnp;
use authentication_capnp::error_code;

pub fn deserialize<T: Read>(reader: &mut T) -> Result<Notification,Error> {
    let options = ReaderOptions::new();
    let message_reader = try!(serialize::read_message(reader, options));
    let root = try!(message_reader.get_root::<notifications_capnp::notification::Reader>());
    match try!(root.which()) {
        notifications_capnp::notification::Which::EntityWalk(walk) => {
            let walk_notif = try!(deserialize_walk(walk));
            Ok(walk_notif)
        }
        _ => unimplemented!(),
    }
}

fn deserialize_walk(reader: notifications_capnp::notification::entity_walk::Reader) -> Result<Notification,Error> {
    let id = reader.get_id();
    let orientation = try!(reader.get_orientation());
    let notif = Notification::walk(id, orientation.into());
    Ok(notif)
}

pub fn deserialize_error_code<T: Read>(reader: &mut T) -> Result<i64,Error> {
    let options = ReaderOptions::new();
    let message_reader = try!(serialize::read_message(reader, options));
    let root = try!(message_reader.get_root::<error_code::Reader>());
    Ok(root.get_code())
}
