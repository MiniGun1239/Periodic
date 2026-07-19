use crate::details::lookup;
use crate::details::element::Element;

pub fn output(number: u8) {
    let element: Option<Element> = lookup(number);

    println!("  Symbol: {}", element.unwrap().symbol);
    println!("---");
}
