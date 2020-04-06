use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut map = phf_codegen::Map::new();
    let bytes = fs::read("src/t2s.txt").expect("cannot open t2s.txt");
    let content = String::from_utf8_lossy(&bytes);
    for line in content.lines() {
        let mut iter = line.split_whitespace();
        let tr = iter.next().unwrap();
        let si = iter.next().unwrap();
        map.entry(tr, &format!("\"{}\"", si));
    }
    let mut file = BufWriter::new(File::create(&path).unwrap());
    write!(
        &mut file,
        "static T2S_MAP: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();
    write!(&mut file, "{}", map.build()).unwrap();
    write!(&mut file, ";\n").unwrap();
}
