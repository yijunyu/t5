use rand::Rng;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use tree_sitter::{Point, Parser, Query, QueryCursor};
use walkdir::{DirEntry, WalkDir};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Extractor {
    query: Query,
    captures: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtractedMatch<'query> {
    kind: &'static str,
    pub name: &'query String,
    pub text: String,
    pub start: Point,
    pub end: Point,
}

impl Extractor {
    pub fn new(lang: tree_sitter::Language, query: &str) -> Extractor {
        let q = Query::new(lang, query).unwrap();
        let captures = q.capture_names().to_vec();
        Extractor {
            query: q,
            captures,
        }
    }

    pub fn extract(&self, tree: &tree_sitter::Tree, source: &[u8]) -> Result<Vec<ExtractedMatch>, std::str::Utf8Error> {
        let mut cursor = QueryCursor::new();
        cursor.captures(&self.query, tree.root_node(), source).map(|(mat, capture_index)| {
            let name = &self.captures[capture_index];
            let mut node = mat.captures[mat.pattern_index].node;
            for cap in mat.captures.iter().filter(|x| x.index as usize == capture_index) {
                node = cap.node;
            }
            let text = match node.utf8_text(source).map(|unowned| unowned.to_string())
            {
                Ok(text) => text,
                Err(problem) => return Err(problem),
            };
            Ok(ExtractedMatch {
                kind: node.kind(),
                name,
                text,
                start: node.start_position(),
                end: node.end_position(),
            })
        }).collect::<Result<Vec<ExtractedMatch>, std::str::Utf8Error>>()
    }
}

fn c_files(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".c"))
        .unwrap_or(false)
}

fn split_name() -> &'static str {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..10);
    match r {
        8 => "test",
        9 => "valid",
        _ => "train",
    }
}

fn get_methods(source: &String, lang: &str, query: &str) -> HashMap::<String, String> {
        let mut map = HashMap::<String, String>::new();
        let source_bytes: &[u8] = &source.as_bytes();
        let language =  match lang {
            "c" => tree_sitter_c::language(),
            "rust" => tree_sitter_rust::language(),
            _ => tree_sitter_c::language(),
        };
        let mut parser = Parser::new();
        parser.set_language(language).unwrap();
        let tree = parser.parse(&source_bytes, None).unwrap();
        let e = Extractor::new(language, query);
        let m = e.extract(&tree, source_bytes).unwrap();
        let mut key = "".to_string(); 
        for v in m {
            if v.name == "i" {
                map.insert(v.text.to_string(), "".to_string());
                key = v.text.to_string();
            } else {
                map.insert(key.to_string(), v.text);
            }
        };
        map
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let walker = WalkDir::new(&args[1])
        .into_iter()
        .filter_map(|entry: Result<DirEntry, walkdir::Error>| entry.ok());
    fs::create_dir_all("data/translate")?;
    ["train", "test", "valid"].map(|name| {
        let t5_c = format!("data/translate/{}.c-rust.txt.c", &name);
        let t5_rust = format!("data/translate/{}.c-rust.txt.rs", &name);
        if std::path::Path::new(&t5_c).exists() {
            std::fs::remove_file(t5_c).unwrap();
        }
        if  std::path::Path::new(&t5_rust).exists() {
            std::fs::remove_file(t5_rust).unwrap();
        }
    });
    for entry in walker.filter(|e| c_files(e)) {
        let source = fs::read_to_string(entry.path())?;
        let c_digest = md5::compute(source.as_bytes());
        let path = &entry.path().clone();
        let basename = &path.file_name().unwrap();
        let rust_file = format!(
            "{}",
            &path
                .parent()
                .unwrap()
                .join("src/")
                .join(basename.to_string_lossy().replace(".c", ".rs"))
                .to_string_lossy()
        );
        if std::path::Path::new(&rust_file).exists() {
                let m1 = get_methods(&source, "c", include_str!("c.scm"));
                let rust_source = fs::read_to_string(&rust_file)?;
                let m2 = get_methods(&rust_source, "rust", include_str!("rust.scm"));
                let name = split_name();
                let t5_c = format!("data/translate/{}.c-rust.txt.c", &name);
                let mut t5_c_file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .create(true)
                                .open(&t5_c)
                                .unwrap();
                let t5_rust = format!("data/translate/{}.c-rust.txt.rs", &name);
                let mut t5_rust_file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .create(true)
                                .open(&t5_rust)
                                .unwrap();

                for (key, value) in &m1 {
                    // Special case: main is mapped to main_0 by c2rust
                    if m2.contains_key(key) && key != "main" { 
                        writeln!(t5_c_file, "{}", &value.replace("\n"," ").to_string())?;
                        writeln!(t5_rust_file, "{}", &m2[key].replace("\n"," ").to_string())?;
                    } else if m2.contains_key("main_0") && key == "main" { 
                        writeln!(t5_c_file, "{}", &value.replace("\n"," ").to_string())?;
                        writeln!(t5_rust_file, "{}", &m2["main_0"].replace("\n"," ").to_string())?;
                    }
                }
        }
    }
    Ok(())
}
