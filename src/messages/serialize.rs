use std::io::{Write,Error};
use capnp::serialize;
use capnp::message::{Allocator,Builder};

use byteorder::{LittleEndian, WriteBytesExt};

use messages::{Order,TargettedOrder,Direction};

use commands_capnp::command::Builder as CommandBuilder;
use authentication_capnp::authentication_token::Builder as AuthBuilder;

pub fn serialize<T: Write>(writer: &mut T, order: &TargettedOrder) -> Result<(),Error> {
    let id = order.target;
    match order.order {
        Order::Walk(ref direction) => serialize_walk(writer, id, direction),
        _ => unimplemented!(),
    }
}

fn serialize_capnp<A,T>(writer: &mut T, message: &mut Builder<A>) -> Result<(),Error>
where A: Allocator,
      T: Write {
    let size = serialize::compute_serialized_size_in_words(message) * 8;
    try!(writer.write_u64::<LittleEndian>(size as u64));

    serialize::write_message(writer, message)
}

fn serialize_walk<T: Write>(writer: &mut T, id: u64, walk: &Option<Direction>) -> Result<(),Error> {
    let mut message_builder = Builder::new_default();
    {
        let mut message = message_builder.init_root::<CommandBuilder>().init_entity_order();
        message.set_origin(id);
        message.set_walk(walk.clone().into());
    }
    serialize_capnp(writer, &mut message_builder)
}

// TODO: Make it 256 bits
pub struct AuthenticationToken {
    data0: u64,
}

pub fn forge_authentication_tokens() -> Vec<AuthenticationToken> {
    (0..30).map(|i| {
        AuthenticationToken {
            data0: i,
        }
    }).collect()
}

pub fn send_authentication_token<T: Write>(writer: &mut T, token: &AuthenticationToken) -> Result<(),Error> {
    let mut message_builder = Builder::new_default();
    {
        let mut message = message_builder.init_root::<AuthBuilder>();
        message.set_data0(token.data0);
    }
    serialize_capnp(writer, &mut message_builder)
}
