use crate::output::electron;
use crate::output::electron::{number, quantum};

static BOHR_CONFIG_KEYWORDS: [&str; 3] = [
    "bc", "bohr", "bohr-config"
];

static QUANTUM_CONFIG_KEYWORDS: [&str; 3] = [
    "qc", "quantum-config", "quantum"
];

static SEMANTIC_CONFIG_KEYWORD: [&str; 7] = [
    "sc", "semantic-config", "semantic",
    "nc", "noble-config", "noble-gas-config", "noblegas-config"
];

static VALENCE_KEYWORDS: [&str; 3] = [
    "v", "valence-electron", "valency"
];

static FIRST_IONIZATION_ENERGY_KEYWORDS: [&str; 5] = [
    "first-ionization-energy", "ionization-energy", "ionization",
    "fie", "ie"
];

static ELECTRON_AFFINITY_KEYWORDS: [&str; 3] = [
    "electron-affinity", "affinity", "ea",
];

pub fn parse(number: u8, args: String) {

    if args == "-h" || args == "--help" || args == "h" || args == "help" {
        help()
    }

    else if args == "default" {
        todo!(
            "call literally everything"
        )
    }

    else if BOHR_CONFIG_KEYWORDS.contains(&&*args) {
        bohr_config(number);
    }

    else if QUANTUM_CONFIG_KEYWORDS.contains(&&*args) {
        quantum_config(number);
    }

    else if SEMANTIC_CONFIG_KEYWORD.contains(&&*args) {
        semantic_config(number);
    }

    else if args == "config" || args == "--config" {
        bohr_config(number);
        quantum_config(number);
    }

    else if VALENCE_KEYWORDS.contains(&&*args) {
        valence(number);
    }

    else if FIRST_IONIZATION_ENERGY_KEYWORDS.contains(&&*args) {
        ionization_energy(number);
    }

    else if ELECTRON_AFFINITY_KEYWORDS.contains(&&*args) {
        affinity(number);
    }
}

fn help() {
    todo!(
        "print all the config functions"
    )
}

fn incomplete() {
    todo!(
        "print a warning, then help, then everything"
    )
}

fn bohr_config(atomic_number: u8) {
    todo!(
        "call output::electron::bohr"
    )
}

fn quantum_config(atomic_number: u8) {
    todo!(
        "call output::electron::quantum"
    )
}

fn semantic_config(atomic_number: u8) {
    todo!(
        "call output::electron::semantic"
    )
}

fn valence(atomic_number: u8) {
    todo!(
        "call output::electron::valence"
    )
}

fn ionization_energy(atomic_number: u8) {
    todo!(
        "call output::electron::ionize_energy"
    )
}

fn affinity(atomic_number: u8) {
    todo!(
        "call output::electron::affinity"
    )
}
