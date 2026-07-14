use crate::details::element::Element;
use crate::details::lookup;

pub fn output(atomic_number: u8) -> f64 {
    let element: Option<Element> = lookup(atomic_number);

    element.unwrap().mass_number
}
