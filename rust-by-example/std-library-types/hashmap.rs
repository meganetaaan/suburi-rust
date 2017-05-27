use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed.
        please hang up and try again.",
        "645-7639" => "Hello this is Mr. awesome's pizza. I'm Fred.
        What can I get for you today?",
        _ => "Hi! who is this again?"
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "436-8291");
    contacts.insert("Robert", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number")
    }

    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number")
    }

    contacts.remove(&("Ashley"));

    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number))
    }
}

