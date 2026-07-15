use crate::electron::quantum_model::noble_gases::NOBLE_GASES;
use crate::electron::quantum_model::orbitals::ORBITALS;

pub fn get_full(mut electrons:u8) -> String {
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

pub fn get_semantic(mut electrons:u8) -> String {
    if electrons == 0 {
        return "No electrons".to_string();
    }

    let electrons_initial: u8 = electrons;

    let mut parts: Vec<String> = Vec::new();
    let preceding_noble_gas: &str;
    let mut preceding_noble_gas_number: u8 = 0;

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
