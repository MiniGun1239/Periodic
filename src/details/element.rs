#[derive(Default, Clone, Debug, PartialEq)]
pub struct Element {
    pub atomic_number: u8,
    pub name: String,
    pub symbol: String,
    pub mass: f64,

    pub boiling_point: f64,
    pub melting_point: f64,
    pub density: f64,
    pub phase: String,

    pub category: String,
    pub group: u8,
    pub period: u8,

    pub bohr_config: String,
    pub quantum_config: String,
    pub quantum_semantic_config: String,
    pub block: char,

    pub first_ionization_energy: f64,
    pub electron_affinity: f64,
}
