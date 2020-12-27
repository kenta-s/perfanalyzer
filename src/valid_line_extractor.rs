use std::fs::File;
use std::io::{Write, BufRead, BufReader, BufWriter};
use regex::Regex;

pub fn extract_usable_lines(filename: String) -> std::io::Result<()> {
    let f = File::create("./ready.log")?;
    let mut stream = BufWriter::new(f);

    for result in BufReader::new(File::open(filename)?).lines() {
        let row = result?;

        let path_regex = Regex::new(r"path=(\S+)").unwrap();
        if !path_regex.is_match(&row) { continue };

        stream.write(&row.as_bytes())?;
        stream.write(b"\n")?;
    }

    Ok(())
}
