extern crate grunejacobs;

use grunejacobs::chomsky;

pub fn demo() {
    let tdh = chomsky::tdh_0();
    println!("tdh: {}", tdh);
}

pub fn main() {
    demo()
}
