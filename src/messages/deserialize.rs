use std::io::{self,ErrorKind,Read};

use capnp::serialize;
use capnp::Error;
use capnp::message::{ReaderOptions,Reader};
use byteorder::{self,LittleEndian, ReadBytesExt};

use super::Notification;
use notifications_capnp::notification;
use authentication_capnp::error_code;

pub fn deserialize<T: Read>(reader: &mut T) -> Result<Notification,Error> {
    let _size_notification = match reader.read_u64::<LittleEndian>() {
        Err(byteorder::Error::Io(e)) => return Err(e.into()),
        Err(byteorder::Error::UnexpectedEOF) => return Err(io::Error::new(ErrorKind::ConnectionAborted, "").into()),
        Ok(size) => size,
    };
    let options = ReaderOptions::new();
    let message_reader = try!(serialize::read_message(reader, options));
    let root = try!(message_reader.get_root::<notification::Reader>());
    match try!(root.which()) {
        notification::Which::EntityWalk(walk) => {
            deserialize_walk(walk)
        }
        notification::Which::EntityLocation(location) => {
            deserialize_location(location)
        }
        _ => unimplemented!(),
    }
}

fn deserialize_walk(reader: notification::entity_walk::Reader) -> Result<Notification,Error> {
    let id = reader.get_id();
    let orientation = try!(reader.get_orientation());
    Ok(Notification::walk(id, orientation.into()))
}

fn deserialize_location(reader: notification::entity_location::Reader) -> Result<Notification,Error> {
    let id = reader.get_id();
    let location = try!(reader.get_location());
    let x = location.get_x();
    let y = location.get_y();
    Ok(Notification::location(id, x, y))
}

pub fn deserialize_error_code<T: Read>(reader: &mut T) -> Result<i64,Error> {
    let _size_notification = match reader.read_u64::<LittleEndian>() {
        Err(byteorder::Error::Io(e)) => return Err(e.into()),
        Err(byteorder::Error::UnexpectedEOF) => return Err(io::Error::new(ErrorKind::ConnectionAborted, "").into()),
        Ok(size) => size,
    };
    let options = ReaderOptions::new();
    let message_reader = try!(serialize::read_message(reader, options));
    let root = try!(message_reader.get_root::<error_code::Reader>());
    Ok(root.get_code())
}
