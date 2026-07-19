use crate::details::element::Element;
use crate::details::lookup;
use crate::electron;

pub fn number(atomic_number: u8) {
    let element: Option<Element> = lookup(atomic_number);

    element.unwrap().atomic_number;
}

pub fn quantum(number: u8) {
    let element: Option<Element> = lookup(number);

    out_quantum_config(number, element.unwrap().quantum_config)
}

pub fn bohr(number: u8) {
    let element: Option<Element> = lookup(number);

    out_bohr_config(number, element.unwrap().bohr_config)
}

pub fn semantic(number: u8) {
    let element: Option<Element> = lookup(number);

    out_semantic_config(number, element.unwrap().quantum_semantic_config)
}

pub fn valence(number: u8) {
    let element: Option<Element> = lookup(number);

    out_valence(number, element);
}

pub fn ionization_energy(number: u8) {
    let element: Option<Element> = lookup(number);

    out_ionization_energy(number, element.unwrap().first_ionization_energy);
}

pub fn affinity(number: u8) {
    let element: Option<Element> = lookup(number);

    out_affinity(number, element.unwrap().electron_affinity);
}

// Special Functions

pub fn all(number: u8) {
    quantum(number);
    bohr(number);
    semantic(number);

    ionization_energy(number);
    affinity(number);

    valence(number);
}

// Outputs

fn out_quantum_config(_number: u8, output: String) {
    println!("  Quantum Configuration:");
    println!("  {}", output);
    println!("---");
}

fn out_bohr_config(_number: u8, output: String) {
    println!("  Bohr configuration:");
    println!("  {}", output);
    println!("---");
}

fn out_semantic_config(_number: u8, output: String) {
    println!("  Semantic configuration:");
    println!("  {}", output);
    println!("---");
}

fn out_valence(number: u8, element: Option<Element>) {
    let element_unwrap_bohr_config = element.unwrap().bohr_config;
    let bohr_electrons: Vec<&str> = element_unwrap_bohr_config.split(',').collect();

    println!("  Valence Information:");
    println!("  Number of electrons in last shell filled: {:?}", electron::valence_electrons::get(number));
    println!("  Electrons in the last {} shell: {}", bohr_electrons.len(), bohr_electrons.last().unwrap());
    println!("---");
}

fn out_ionization_energy(_number: u8, output: f64) {
    println!("  First Ionization Energy:");
    println!("  {} Joules", output);
    println!("---");
}

fn out_affinity(_number: u8, output: f64) {
    println!("  Electron Affinity:");
    println!("  {} Joules", output);
    println!("---");
}
