use crate::details::element::Element;
use crate::details::lookup;

pub fn output(number: u8) {
    let element: Option<Element> = lookup(number);

    println!("  Average Mass: {}u", element.unwrap().mass);
    println!("---");
}
