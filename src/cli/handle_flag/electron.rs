use std::arch::x86_64::_mm_add_sd;
use std::ptr::addr_eq;
use clap::ArgAction::SetTrue;

pub fn parse(args: String) {

    let bohr_config = [
        "b", "bohr", "bohr-config",
        "-b", "--bohr", "--bohr-config",
    ];

    let quantum_config = [
        "q", "quantum-config", "quantum",
        "-q", "--quantum-config", "--quantum"
    ];

    let valence = [
        "v", "valence-electron", "valency",
        "-v", "--valence-electron", "--valency"
    ];

    if args == "-h" || args == "--help" {
        todo!()
    }

    else if args == "default" {
        todo!()
    }

    else if bohr_config.contains(&&*args) {
        todo!()
    }

    else if quantum_config.contains(&&*args) {
        todo!()
    }

    else if args == "config" || args == "--config" {
        todo!(
            "add both the bohr and quantum and semantic"
        )
    }

    else if valence.contains(&&*args) {
        todo!()
    }
}

fn help() {
    todo!("add help things")
}

fn incomplete() {
    todo!("FAHHH")
}

fn bohr_config(atomic_number: u8) {
    todo!()
}

fn quantum_config(atomic_number: u8) {
    todo!()
}
