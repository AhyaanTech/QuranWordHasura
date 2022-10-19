use std::fs;

use serde_json::{Result, Value};
use unicode_segmentation::UnicodeSegmentation;

fn main() -> Result<()> {
    let contents =
        fs::read_to_string("first.json").expect("Should have been able to read the file");
    let v: Value = serde_json::from_str(&contents)?;
    let binding = &v["ayahs"][0]["text"].to_string();
    let ayahs = binding
        .unicode_words()
        // .map(|f| f.escape_unicode().to_string())
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|x| x.to_owned().ne("\"") && x.to_owned().ne("\u{feff}"))
        .collect::<Vec<&str>>();
    println!("With text:\n{:#?}", ayahs);
    Ok(())
}
