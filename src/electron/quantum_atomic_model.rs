static ORBITALS: [(&str, u8); 29] = [
    ("1s", 2),
    ("2s", 2), ("2p", 6),
    ("3s", 2), ("3p", 6),
    ("4s", 2), ("3d", 10), ("4p", 6),
    ("5s", 2), ("4d", 10), ("5p", 6),
    ("6s", 2), ("4f", 14), ("5d", 10), ("6p", 6),
    ("7s", 2), ("5f", 14), ("6d", 10), ("7p", 6),
    ("8s", 2), ("5g", 18), ("6f", 14), ("7d", 10), ("8p", 6),
    ("9s", 2), ("6g", 18), ("7f", 14), ("8d", 10), ("9p", 6),
];

static NOBLE_GASES: [(&str, u8); 7] = [
    ("He", 2), ("Ne", 10), ("Ar", 18), ("Kr", 36),
    ("Xe", 54), ("Rn", 86), ("Og", 118)
];

pub fn get_full_config(mut electrons:u8) -> String {
    if electrons == 0 {
        return "No electrons".to_string();
    }

    let mut parts: Vec<String> = Vec::new();

    for &(orbital, amount) in &ORBITALS {
        if electrons == 0 {
            break
        }

        let give = std::cmp::min(electrons, amount);

        parts.push(format!("{}{}", orbital, give));

        electrons -= give;
    }

    parts.join(", ")
}

pub fn get_shorthand_config(mut electrons:u8) -> String {
    if electrons == 0 {
        return "No electrons".to_string();
    }

    let electrons_initial: u8 = electrons;

    let mut parts: Vec<String> = Vec::new();
    let preceding_noble_gas: &str;
    let preceding_noble_gas_number: u8;

    for (name, i) in NOBLE_GASES.iter().rev() {
        if *i == 1 {
            preceding_noble_gas_number = 0;
            break
        }

        if i < &electrons {
            preceding_noble_gas = name;
            preceding_noble_gas_number = *i;


            parts.push(format!("[{}]", preceding_noble_gas));
            break
        }
    }

    for &(orbital, amount) in &ORBITALS {
        if electrons == 0 {
            break
        }

        if electrons > electrons_initial - preceding_noble_gas_number {
            electrons -= amount;

            continue
        }

        let give = std::cmp::min(electrons, amount);

        parts.push(format!("{}{}", orbital, give));

        electrons -= give;
    }

    parts.join(", ")
}