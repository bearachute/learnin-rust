#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 'use' each name so they are available
    // without manual scoping
    use Status::{Poor, Rich};
    // Automatically 'use' each name inside work!
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        // lack of scoping because of use above
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),

    }

    match work {
        // note again lack of scope
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldier, fight!"),
    }
}