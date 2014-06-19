extern crate grunejacobs;

use grunejacobs::chomsky;

pub fn demo() {
    let tdh = chomsky::tdh();
    println!("tdh: {}", tdh);
}

pub fn main() {
    demo()
}
