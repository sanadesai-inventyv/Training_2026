mod modules; 

use modules::loops;
use modules::mutex;
use modules::ownership;
use modules::rwlock;
use modules::serde;
use modules::structures;

fn main() {
    println!("Modules are working!");
    loops::run();
    mutex::run();
    ownership::run();
    rwlock::run();
    serde::run();
    structures::run();
}
