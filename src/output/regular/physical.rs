use crate::details::element::Element;
use crate::details::lookup;

pub fn boil(number: u8) {
    let element:Option<Element> = lookup(number);

    out_boil(number, element);
}

pub fn melt(number: u8) {
    let element:Option<Element> = lookup(number);

    out_melt(number, element);
}

pub fn density(number: u8) {
    let element:Option<Element> = lookup(number);

    out_density(number, element);
}

pub fn phase(number: u8) {
    let element:Option<Element> = lookup(number);

    out_phase(number, element);
}


// special functions

pub fn all(number: u8) {
    boil(number);
    melt(number);

    density(number);

    phase(number);
}


// output functions

fn out_boil(_number: u8, element: Option<Element>) {
    let output = element.unwrap().boiling_point;

    println!("Boiling point: {} Kelvin", output);
    println!("---");
}

fn out_melt(_number: u8, element: Option<Element>) {
    let output = element.unwrap().melting_point;

    println!("Melting point: {} Kelvin", output);
    println!("---");
}

fn out_density(_number: u8, element: Option<Element>) {
    let output = element.unwrap().density;

    println!("Density at STP: {} g/cm³", output);
    println!("---");
}

fn out_phase(_number: u8, element: Option<Element>) {
    let output = element.unwrap().phase;

    println!("Phase at STP: {}", output);
    println!("---");
}
