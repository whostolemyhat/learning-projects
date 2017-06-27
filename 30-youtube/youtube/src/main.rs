extern crate reqwest;
extern crate select;
extern crate regex;

use std::io::{ self, Read };
use select::document::Document;
use select::predicate::{ Name };
use regex::Regex;

fn main() {
    let mut res = reqwest::get("https://www.youtube.com/watch?v=yfG94k41MrI").unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content);

    // let dom = parse_document(RcDom::default(), Default::default()).from_bytes().read_from(&mut content.as_bytes()).unwrap();
    // serialize(&mut io::stdout(), &dom.document, Default::default());

    let re = Regex::new(r"ytplayer.config").unwrap();

    let doc = Document::from(content.as_str());
    for node in doc.find(Name("script")) {
    	// get text
    	// search text for ytplayer.config
    	// use that node
    	// get json data from node
    	if re.is_match(&node.text().to_string()) {
    		println!("{:?}", node.text());
    		break;
    	}
    }
}
