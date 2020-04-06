use std::env;
use std::fs::File;
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut map = phf_codegen::Map::new();
    let t2s_file = File::open("src/t2s.txt").expect("cannot open t2s.txt");
    let reader = BufReader::new(t2s_file);
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let tr = iter.next().unwrap();
        let si = iter.next().unwrap();
        map.entry(tr.to_string(), &format!("\"{}\"", si));
    });
    let mut file = BufWriter::new(File::create(&path).unwrap());
    write!(&mut file, "static T2S_MAP: phf::Map<&'static str, &'static str> = ").unwrap();
    map.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}
