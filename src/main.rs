mod universe;

use std::{
    thread::sleep,
    time::Duration
};

use universe::Universe;

fn main() {
    let mut my_universe = Universe::new(
        32,
        32,
        0.5,
        3,
        4,
        vec![2],
    );

    loop {
        println!("{}", my_universe);
        sleep(Duration::from_millis(200));
        my_universe.tick();
    }
}