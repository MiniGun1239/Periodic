use std::collections::HashMap;

pub mod element;

use element::Element;

pub fn lookup(atomic_number: u8) -> Option<Element> {
    let db = get_database();
    db.get(&atomic_number).cloned()
}


pub fn get_database() -> HashMap<u8, Element> {
    let mut table: HashMap<u8, Element> = HashMap::new();

    table.insert(
        1,
        Element {
            name: String::from("Hydrogen"),
            atomic_number: 1,
            mass_number: 1.008
        }
    );

    table
}
