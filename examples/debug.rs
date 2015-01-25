extern crate nibble_vec;

use nibble_vec::NibbleVec;

fn main() {
    let mut v = NibbleVec::from_byte_vec(vec![1 << 4 | 11, 2 << 4 | 12, 3 << 4 | 13]);
    v.push(4);
    println!("{:?}", v);
}
