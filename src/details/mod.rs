use std::collections::HashMap;
use serde_json;

pub mod element;

use element::Element;

static RAW_JSON: &'static str = include_str!("element_info.json");

pub fn lookup(atomic_number: u8) -> Option<Element> {
    let db = get_database();
    db.get(&atomic_number).cloned()
}


pub fn get_database() -> HashMap<u8, Element> {
    let table: HashMap<u8, Element> = serde_json::from_str(RAW_JSON).unwrap();

    table
}
