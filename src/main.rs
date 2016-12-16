extern crate pnet;
extern crate pnet_macros_support;

use pnet::packet::{Packet, MutablePacket};

mod packet;
use packet::my_protocol::{MyProtocolPacket, MutableMyProtocolPacket};

fn main() {
}
