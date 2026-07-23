use std::collections::HashMap;
use crate::details::element::Element;

pub fn name(reverse: bool) {
    out_name(reverse);
}

pub fn symbol(reverse: bool) {
    out_symbol(reverse);
}

pub fn number(reverse: bool) {
    out_number(reverse);
}

pub fn mass(reverse: bool) {
    out_mass(reverse);
}


// output functions

fn out_name(reverse: bool) {
    let elements: Vec<Element> = get_elements();

    let mut names: Vec<(u8, String)> = get_names_with_index(elements);

    names.sort_by(|a, b| {
        if !reverse {
            a.1.cmp(&b.1)
        } else {
            b.1.cmp(&a.1)
        }
    });

    for (num, name) in names {
        println!("{}. {}", num, name);
    }
}

fn out_symbol(reverse: bool) {
    let elements: Vec<Element> = get_elements();

    let mut symbols: Vec<(u8, String)> = get_symbols_with_index(elements);

    symbols.sort_by(|a, b| {
        if !reverse {
            a.1.cmp(&b.1)
        } else {
            b.1.cmp(&a.1)
        }
    });

    for (num, symbol) in symbols {
        println!("{}: {}", num, symbol);
    }
}

fn out_number(reverse: bool) {
    let elements: Vec<Element> = get_elements();

    let mut numbers: Vec<(u8, String)> = get_names_with_index(elements);

    numbers.sort_by(|a, b| {
        if !reverse {
            a.0.cmp(&b.0)
        } else {
            b.0.cmp(&a.0)
        }
    });

    for (num, name) in numbers {
        println!("{}: {}", num, name);
    }
}

fn out_mass(reverse: bool) {
    let elements: Vec<Element> = get_elements();

    let mut mass: Vec<(String, f64)> = get_mass_with_name(elements);

    mass.sort_by(|b, a| {
        if reverse {
            a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        }
    });

    for (name, mass) in mass {
        println!("{}: {}", name, mass);
    }
}


// helper

fn get_names_with_index(elements: Vec<Element>) -> Vec<(u8, String)> {
    elements
        .iter()
        .map(|element| (element.atomic_number, element.name.clone()))
        .collect()
}

fn get_symbols_with_index(elements: Vec<Element>) -> Vec<(u8, String)> {
    elements
        .iter()
        .map(|element| (element.atomic_number, element.symbol.clone()))
        .collect()
}

fn get_mass_with_name(elements: Vec<Element>) -> Vec<(String, f64)> {
    elements
        .iter()
        .map(|element| (element.name.clone(), element.mass))
        .collect()
}

fn get_elements() -> Vec<Element> {
    let json_data = include_str!("../../details/element_info.json");

    let elements_map: HashMap<u8, Element> = serde_json::from_str(json_data).unwrap();

    let elements: Vec<Element> = elements_map.into_values().collect();
    elements
}
