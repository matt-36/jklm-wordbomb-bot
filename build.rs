use serde::Deserialize;
use std::{borrow::Cow, io::Write, path::Path};

const DICTS_JSON: &[u8] = std::include_bytes!("./dicts.json");

#[derive(Deserialize)]
struct Stuff {
    word: Cow<'static, str>,
}
#[allow(clippy::from_over_into)]
impl<'a> Into<Cow<'a, str>> for Stuff {
    fn into(self) -> Cow<'a, str> {
        self.word
    }
}

fn main() {
    if Path::new("./target/words").exists() {
        return;
    }

    let thing: Vec<Stuff> = serde_json::from_slice(DICTS_JSON).unwrap();
    let mut out = std::io::BufWriter::new(std::fs::File::create("./target/words").unwrap());

    for line in thing.into_iter().map(<Stuff as Into<Cow<str>>>::into) {
        out.write_all(line.as_bytes()).unwrap();
        out.write_all(b"\n").unwrap();
    }
    out.flush().unwrap();
}
