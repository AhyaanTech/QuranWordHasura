use std::fs;

use serde_json::{Result, Value};

fn main() -> Result<()> {
    let contents =
        fs::read_to_string("first.json").expect("Should have been able to read the file");
    let v: Value = serde_json::from_str(&contents)?;
    let ayahs = v["ayahs"][0]["text"].to_owned().to_string().chars();
    println!("With text:\n{:?}", ayahs);
    Ok(())
}
