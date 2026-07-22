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

pub fn number(reverse: bool) {
    out_number(reverse);
}

pub fn mass(reverse: bool) {
    out_mass(reverse);
}


// output functions

fn out_name(reverse: bool) {
    let elements: Vec<Element> = serde_json::from_str("../../details/element_info.json").unwrap();

    let mut names: Vec<(u8, String)> = elements
        .iter()
        .enumerate()
        .map(|(index, element)| ((index + 1) as u8, element.name.clone()))
        .collect();

    names.sort_by(|a, b| a.1.cmp(&b.1));

    if reverse {
        names.reverse();
    }

    for (num, name) in names {
        println!("{}: {}", name, num);
    }
}

fn out_symbol(reverse: bool) {
    let elements: Vec<Element> = serde_json::from_str("../../details/element_info.json").unwrap();

    let mut symbols: Vec<(u8, String)> = elements
        .iter()
        .enumerate()
        .map(|(index, element)| ((index + 1) as u8, element.symbol.clone()))
        .collect();

    symbols.sort_by(|a, b| a.1.cmp(&b.1));

    if reverse {
        symbols.reverse();
    }

    for (num, symbol) in symbols {
        println!("{}: {}", num, symbol);
    }
}

fn out_number(reverse: bool) {
    let elements: Vec<Element> = serde_json::from_str("../../details/element_info.json").unwrap();

    let mut numbers: Vec<(u8, String)> = elements
        .iter()
        .enumerate()
        .map(|(index, element)| ((index + 1) as u8, element.name.clone()))
        .collect();

    if reverse {
        numbers.reverse();
    }

    for (num, name) in numbers {
        println!("{}: {}", num, name);
    }
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
