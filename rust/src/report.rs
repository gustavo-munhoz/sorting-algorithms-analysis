use std::fs::File;
use std::io::Write;

use crate::runner::Record;

pub fn to_json(path: &str, data: &[Record]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let json = serde_json::to_string_pretty(data)?;

    file.write_all(json.as_bytes())
}