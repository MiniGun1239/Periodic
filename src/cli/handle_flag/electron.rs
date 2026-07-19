use crate::output;


// keywords

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
    "ea", "electron-affinity", "affinity",
];


// parser

pub fn parse(number: u8, args: String) {

    if args == "-h" || args == "--help" || args == "h" || args == "help" {
        help()
    }

    else if args == "." || args == "a" || args == "all" {
        all(number);
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

    else {
        incomplete()
    }
}


// priv funcs

fn bohr_config(number: u8) {
    output::electron::bohr(number);
}

fn quantum_config(number: u8) {
    output::electron::quantum(number);
}

fn semantic_config(number: u8) {
    output::electron::semantic(number);
}

fn valence(number: u8) {
    output::electron::valence(number);
}

fn ionization_energy(number: u8) {
    output::electron::ionization_energy(number);
}

fn affinity(number: u8) {
    output::electron::affinity(number);
}

// Special functions

fn help() {
    println!("List of Available Options:");
    println!("Usage: periodic [ATOMIC_NUMBER] --electron [OPTIONS]");
    println!(" ---");

    print!  ("  h , help                 Prints help information\n\n");

    println!("  bc, bohr-config          Prints the Bohr Configuration");
    println!("  qc, quantum-config       Prints the Quantum Configuration");
    print!  ("  sc, semantic-config      Prints the Semantic Configuration\n\n");

    println!("  v , valence              Prints the Valence Information");
    println!("  ie, ionization-energy    Prints the ionization Energy");
    print!  ("  ea, electron-affinity    Prints the Electron Affinity\n\n");

    println!("  a , all                  Prints all of the above");
}

fn incomplete() {
    println!("  Incorrect or no arguments provided");
    println!("    -- -- -- -- -- - -- -- -- -- -- ");
    println!();

    help();
}

fn all(number: u8) {
    output::electron::all(number)
}
