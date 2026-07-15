use std::collections::HashMap;
use serde::Deserialize;

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
            mass: 1.008,
            boiling_point: 0.0,
            melting_point: 0.0,
            density: 0.0,
            phase: "".to_string(),
            category: "".to_string(),
            group: 0,
            period: 0,
            bohr_config: "".to_string(),
            quantum_config: "".to_string(),
            quantum_semantic_config: "".to_string(),
            block: '',
            first_ionization_energy: 0.0,
            symbol: "".to_string(),
            electron_affinity: 0.0,
        }
    );

    table
}
