use crate::details::element::Element;
use crate::details::lookup;

pub fn output(number: u8) {
    let element: Option<Element> = lookup(number);
    
    println!("  Name: {}", element.unwrap().name);
    println!("---");
}
