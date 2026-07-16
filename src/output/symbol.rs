use crate::details::lookup;
use crate::details::element::Element;

pub fn output(atomic_number: u8) {
    let element: Option<Element> = lookup(atomic_number);

    element.unwrap().symbol;
}