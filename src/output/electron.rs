use crate::details::element::Element;
use crate::details::lookup;

pub fn number(atomic_number: u8) {
    let element: Option<Element> = lookup(atomic_number);

    element.unwrap().atomic_number;
}

pub fn quantum(number: u8) {
    let element: Option<Element> = lookup(number);

    element.unwrap().quantum_config;
}

pub fn bohr(number: u8) {
    let element: Option<Element> = lookup(number);

    element.unwrap().bohr_config;
}

pub fn semantic(number: u8) {
    let element: Option<Element> = lookup(number);

    element.unwrap().quantum_semantic_config;
}

pub fn valence(number: u8) {
    let element: Option<Element> = lookup(number);

    drop(element);
}

pub fn ionization_energy(number: u8) {
    let element: Option<Element> = lookup(number);

    element.unwrap().first_ionization_energy;
}

pub fn affinity(number: u8) {
    let element: Option<Element> = lookup(number);

    element.unwrap().electron_affinity;
}
