use regex::Regex;

// main.rs has declared this mod already
// ie imagine there's a pub mod html {} here
// pub so we can import into test mod
pub fn strip_tags(html: String) -> String {
 // regex: (</?[^>]+>) - matches + captures any tag
 let re = Regex::new(r"(</?[^>]+>)").unwrap();
 let new_lines = Regex::new(r"(\n\s+)").unwrap();

 // &html[..] - convert String to &str (slice of entire string)
 let modified = re.replace_all(&html[..], "");

 new_lines.replace_all(&modified[..], "\n")
}


#[cfg(test)]
mod test {
  use super::*;

    #[test]
    fn strip_html() {
      assert_eq!(strip_tags("<strong>Hello</strong> <em>world</em>".to_string()), "Hello world");
    }
}