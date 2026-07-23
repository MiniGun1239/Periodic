use crate::output::regular::name::output;

pub fn parse(atomic_number:u8) {
    output(atomic_number);
}

pub fn usage() {
    println!("Usage: periodic <atomic_number> --name")
}