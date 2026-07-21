use crate::details::element::Element;
use crate::details::lookup;

pub fn category(number: u8) {
    let element: Option<Element> = lookup(number);

    out_category(number, element);
}

pub fn group(number: u8) {
    let element: Option<Element> = lookup(number);

    out_group(number, element);
}

pub fn period(number: u8) {
    let element: Option<Element> = lookup(number);

    out_period(number, element);
}

pub fn block(number: u8) {
    let element: Option<Element> = lookup(number);

    out_block(number, element);
}


// Special functions

pub fn all(number: u8) {
    category(number);
    group(number);
    period(number);
    block(number);
}


// Output functions

fn out_category(_number: u8, element: Option<Element>) {
    let output = element.unwrap().category;

    println!("  Category: {}", output);
    println!("---");
}

fn out_group(_number: u8, element: Option<Element>) {
    let output = element.unwrap().group;

    println!("  Group: {}", output);
    println!("---");
}

fn out_period(_number: u8, element: Option<Element>) {
    let output = element.unwrap().period;

    println!("  Period: {}", output);
    println!("---");
}

fn out_block(_number: u8, element: Option<Element>) {
    let output = element.unwrap().block;

    println!("  Block: {}", output);
    println!("---");
}
