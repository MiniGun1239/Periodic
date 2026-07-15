use crate::electron::quantum_model::orbitals::ORBITALS;

pub fn get(mut electrons:u8) -> u8 {
    if electrons == 0 {
        return 0;
    }

    for &(orbital, amount) in &ORBITALS {
        if electrons == 0 {
            return 0;
        }

        if electrons >= amount {
            electrons -= amount;
        } else {
            return electrons;
        }
    }

    0
}
