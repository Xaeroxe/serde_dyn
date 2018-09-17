extern crate uuid;
use std::mem::transmute;
use uuid::Uuid;

fn main() {
    println!("{}", unsafe {transmute::<[u8; 16], u128>(*Uuid::new_v4().as_bytes())});
}
