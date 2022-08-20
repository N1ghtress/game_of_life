mod universe;

use std::{
    thread::sleep,
    time::Duration
};

use universe::Universe;

fn main() {
    let mut my_universe = Universe::new(16, 16);
    loop {
        println!("{}", my_universe);
        sleep(Duration::from_millis(1000));
        my_universe.tick();
    }
}
