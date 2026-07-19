use crate::details::element::Element;
use crate::details::lookup;

pub fn category(number: u8) {
    let element: Option<Element> = lookup(number);

    out_category(element, number);
}

pub fn group(number: u8) {
    let element: Option<Element> = lookup(number);

    out_group(element, number);
}

pub fn period(number: u8) {
    let element: Option<Element> = lookup(number);

    out_period(element, number);
}

pub fn block(number: u8) {
    let element: Option<Element> = lookup(number);

    out_block(element, number);
}


// Special functions

pub fn all(number: u8) {
    category(number);
    group(number);
    period(number);
    block(number);
}


// Output functions

fn out_category(element: Option<Element>, number: u8) {
    let output = element.unwrap().category;

    println!("Category of element {}: {}", number, output);
    println!("  ---");
}

fn out_group(element: Option<Element>, number: u8) {
    let output = element.unwrap().group;

    println!("Group of element {}: {}", number, output);
    println!("  ---");
}

fn out_period(element: Option<Element>, number: u8) {
    let output = element.unwrap().period;

    println!("Period of element {}: {}", number, output);
    println!("  ---");
}

fn out_block(element: Option<Element>, number: u8) {
    let output = element.unwrap().block;

    println!("Block of element {}: {}", number, output);
    println!("  ---");
}
