extern crate reqwest;
extern crate select;
extern crate regex;
extern crate serde_json;

use std::io::{ Read };
use std::collections::HashMap;
use select::document::Document;
use select::predicate::{ Name };
use regex::Regex;
use serde_json::{ Value, Error };

fn parse_json(text: &str, start: usize) -> Result<Value, Error> {
    let end_re = Regex::new(r";ytplayer.load =").unwrap();
    // let mat = re.find(text).unwrap();
    let end_pos = end_re.find(text).unwrap();

    let start = start + 18;
    let end = end_pos.end() - 16;

    let slice = &text[start..end];

    let parsed: Value = serde_json::from_str(slice)?;

    Ok(parsed)
}

fn main() {
    let mut res = reqwest::get("https://www.youtube.com/watch?v=yfG94k41MrI").unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content);

    // let dom = parse_document(RcDom::default(), Default::default()).from_bytes().read_from(&mut content.as_bytes()).unwrap();
    // serialize(&mut io::stdout(), &dom.document, Default::default());

    let re = Regex::new(r"ytplayer.config").unwrap();
    // let end_re = Regex::new(r";ytplayer.load =").unwrap();

    let doc = Document::from(content.as_str());
    for node in doc.find(Name("script")) {
        // get text
        // search text for ytplayer.config
        // use that node
        // get json data from node
        if re.is_match(&node.text().to_string()) {
            let text = &node.text();
            let mat = re.find(text).unwrap();

            match parse_json(&node.text(), mat.start()) {
                Ok(parsed) => {
                    let stream_map: String = parsed["args"]["url_encoded_fmt_stream_map"].to_string();
                    let mut videos: Vec<HashMap<&str, &str>> = vec![];
                    let urls: Vec<&str> = stream_map.split(",").collect();


                    for url in urls {
                        let mut params: HashMap<&str, &str> = HashMap::new();

                        let stuff = url.split("&");
                        for s in stuff {
                            let param: Vec<&str> = s.split("=").collect();
                            // println!("{:?}", param);
                            params.insert(param[0], param[1]);
                        }

                        videos.push(params);
                    }

                    println!("{:?}", videos);
                    break;
                },
                Err(e) => panic!("Error parsing json: {}", e)
            }

        }
    }
}
