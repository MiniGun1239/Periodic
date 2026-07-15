pub mod bohr_atomic_model;
pub mod quantum_atomic_model;

pub struct Electron {
    principal: u8,
    azimuthal: u8,
    magnetic: u8,
    spin: bool // True = +1/2, False = -1/2
}
