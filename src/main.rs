use syn::{parse_file, Item, ReturnType};
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rust-autotyping <filename>");
        return;
    }
    
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Failed to read file");
    let syntax_tree = parse_file(&code).expect("Failed to parse file");

    println!("Analyzing file: {}", filename);
    
    for item in syntax_tree.items {
        if let Item::Fn(func) = item {
            match func.sig.output {
                ReturnType::Default => {
                    println!("Function '{}' is missing a return type!", func.sig.ident);
                }
                _ => {}
            }
        }
    }
}
