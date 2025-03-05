use unicode_segmentation::UnicodeSegmentation;

pub fn separate(s: &str) -> (String, String) {
    let graphemes = s.graphemes(true);
    match graphemes.count() {
        0 => ('\0'.to_string(), String::new()),
        1 => (s.graphemes(true).next().unwrap().to_string(), String::new()),
        _ => {
            let mut graphemes = s.graphemes(true);
            let head = graphemes.next().unwrap().to_string();
            let tail = graphemes.collect::<String>();
            (head, tail)
        }
    }
}
