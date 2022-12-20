use tree_sitter_tags::{TagsContext, TagsConfiguration};
use std::fs::File;
use std::io::Write;
use std::env;
use std::fs;
use std::path::Path;

mod ruby;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut tags_file = File::create("tags_output").unwrap();
    let mut context = TagsContext::new();
    let ruby_config = TagsConfiguration::new(
        tree_sitter_ruby::language(),
        include_str!("../ruby/tags.scm"),
        tree_sitter_ruby::LOCALS_QUERY,
    ).unwrap();

    args[1..].iter().flat_map(|filename| {
        let contents = fs::read(&filename).unwrap();
        let path = Path::new(filename);

        match path.extension() {
            None => vec![],
            Some(os_str) => {
                match os_str.to_str() {
                    Some("rb") => ruby::generate_tags(&mut context, &ruby_config, filename, &contents),
                    _ => vec![]
                }
            }
        }
    }).for_each(|line| tags_file.write_all(line.as_bytes()).unwrap());
}
