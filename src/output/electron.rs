use crate::details::element::Element;
use crate::details::lookup;

fn number(atomic_number: u8) {
    let element: Option<Element> = lookup(atomic_number);
    
    element.unwrap().atomic_number;
}