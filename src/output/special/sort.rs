pub fn name() {
    todo!(
        "call out_name or out_name_reversed"
    )
}

pub fn symbol() {
    todo!(
        "call out_symbol or out_symbol_reversed"
    )
}

pub fn number() {
    todo!(
        "call out_number or out_number_reversed"
    )
}

pub fn mass() {
    todo!(
        "call out_mass or out_mass_reversed"
    )
}


// output functions

fn out_name(reverse: bool) {
    todo!(
        "call output::special::sort::name or name_reversed"
    )
}

fn out_symbol(reverse: bool) {
    todo!(
        "call output::special::sort::symbol or symbol_reversed"
    )
}

fn out_number(reverse: bool) {
    todo!(
        "call output::special::sort::number or number_reversed"
    )
}

fn out_mass(reverse: bool) {
    let elements: Vec<Element> = serde_json::from_str("../../details/element_info.json").unwrap();

    let mut mass: Vec<(String, f64)> = elements
        .iter()
        .map(|element| (element.name.clone(), element.mass))
        .collect();

    mass.sort_by(|b, a| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    
    if reverse {
        mass.reverse();
    }

    for (name, mass) in mass {
        println!("{}: {}", name, mass);
    }
}
