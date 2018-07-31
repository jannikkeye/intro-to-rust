extern crate example_crate;
extern crate rand;

use rand::prelude::*;

fn main() {
    let example_crate = example_crate::Struct::new();
    println!("{} from example_crate\n", example_crate);

    let random_int: u8 = random();

    println!("{}", random_int)
}
