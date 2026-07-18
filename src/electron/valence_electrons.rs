use crate::electron::orbitals::ORBITALS;

pub fn get(mut electrons:u8) -> u8 {
    if electrons == 0 {
        return 0;
    }

    for &(_orbital, amount) in &ORBITALS {
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
