use capnp::serialize;
use capnp::message::{Allocator,Builder};

use byteorder::{LittleEndian, WriteBytesExt};

use messages::{Order,TargettedOrder,Direction};

use commands_capnp::command::Builder as CommandBuilder;
use authentication_capnp::authentication_token::Builder as AuthBuilder;

pub fn serialize(order: &TargettedOrder) -> Vec<u8> {
    let id = order.target;
    match order.order {
        Order::Walk(ref direction) => serialize_walk(id, direction),
        _ => unimplemented!(),
    }
}

fn serialize_capnp<A: Allocator>(message: &mut Builder<A>) -> Vec<u8> {
    let size = serialize::compute_serialized_size_in_words(message) * 8;
    let mut res = Vec::with_capacity(size + 8);
    res.write_u64::<LittleEndian>(size as u64).unwrap();

    // unwrap safe because Err is never returned by Write for Vec<u8>
    serialize::write_message(&mut res, message).unwrap();
    debug_assert_eq!(res.len(), size + 8);
    res
}

fn serialize_walk(id: u64, walk: &Option<Direction>) -> Vec<u8> {
    let mut message_builder = Builder::new_default();
    {
        let mut message = message_builder.init_root::<CommandBuilder>().init_entity_order();
        message.set_origin(id);
        message.set_walk(walk.clone().into());
    }
    serialize_capnp(&mut message_builder)
}

pub fn fake_authentication_token(number: u64) -> Vec<u8> {
    let mut message_builder = Builder::new_default();
    {
        let mut message = message_builder.init_root::<AuthBuilder>();
        message.set_data0(number);
    }
    serialize_capnp(&mut message_builder)
}
