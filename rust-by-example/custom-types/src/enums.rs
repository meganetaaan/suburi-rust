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
    // Explicitly `use` each name so that they are available without
    // manual scoping
    use Status::{Poor, Rich};
    use Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        Rich => println!("The rich!"),
        Poor => println!("The poor!"),
    }

    match work {
        // Note again the lack of scoping
        Civilian => println!("Civilian!"),
        Soldier => println!("Soldier!"),
    }
}
