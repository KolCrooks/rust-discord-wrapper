use super::gateway_payload::PayloadOpcode;

pub mod core;
mod identify_payload;
pub use identify_payload::*;
pub mod dispatch_payloads;

pub trait PayloadData {
    fn get_opcode(&self) -> PayloadOpcode;
}
