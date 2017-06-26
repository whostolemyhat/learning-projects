extern crate reqwest;
extern crate html5ever;

use std::io::{ self, Read };
use html5ever::{ parse_document, serialize };
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;

fn main() {
    let mut res = reqwest::get("https://www.youtube.com/watch?v=yfG94k41MrI").unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content);

    let dom = parse_document(RcDom::default(), Default::default()).from_utf8().read_from(&mut content.as_bytes()).unwrap();

    serialize(&mut io::stdout(), &dom.document, Default::default());
}
