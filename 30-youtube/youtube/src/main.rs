extern crate reqwest;
extern crate html5ever;
extern crate kuchiki;

use std::io::{ self, Read };
use html5ever::{ parse_document, serialize };
use html5ever::rcdom::{ RcDom, Handle };
use html5ever::tendril::TendrilSink;

// fn get_script_links(handle: Handle) -> Vec<String> {
// 	let mut scripts: Vec<String> = Vec::new();
// 	let mut queue: Vec<Handle> = Vec::new();

// 	queue.push(handle);

// 	while queue.len() != 0 {
// 		let handle = queue.remove(0);
// 		let node = handle.borrow();

// 		match node.node {
// 			Element(ref name, _, ref attrs)=> {
// 				let mut is_script = false;

// 				for attr in attrs.iter() {
// 					let link = string_cache::Atom::from("script");
// 					if name.local == link {
// 						is_script = true;
// 						scripts.push(String::from(attr.value.clone()));
// 					}
// 				}
// 			},
// 			_ => {}
// 		}

// 		for child in node.children.iter() {
// 			queue.push(child.clone());
// 		}
// 	}

// 	scripts
// }

fn main() {
    let mut res = reqwest::get("https://www.youtube.com/watch?v=yfG94k41MrI").unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content);

    // let dom = parse_document(RcDom::default(), Default::default()).from_bytes().read_from(&mut content.as_bytes()).unwrap();
    // serialize(&mut io::stdout(), &dom.document, Default::default());

    // get_script_links(dom.document);
    let doc = kuchiki::parse_html().read_from(&mut content.as_bytes()).unwrap();
    println!("{:?}", doc);
}
