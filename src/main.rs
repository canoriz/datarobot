// Read BNFs and generate text
use std::fs;
use std::io;
use std::path::Path;

mod collection;
mod parser;
mod preprocessor;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_bnfs_from_file("./bnfs") {
        let mut a = collection::Collection::new();
        lines.iter().for_each(|l| {
            if let Err(s) = a.add(l) {
                println!("[skip {}] {}", l, s);
            }
        });

        vec!["<output>"]
            .iter()
            .for_each(|bnf_expr| match a.gen(bnf_expr) {
                Ok(s) => println!("{}: {}", bnf_expr, s),
                Err(s) => println!("{}", s),
            });
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the BNFs of the file.
fn read_bnfs_from_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    Ok(read_bnfs(fs::read_to_string(filename)?))
}

fn read_bnfs(s: String) -> Vec<String> {
    s.split("\n\n").map(str::to_string).collect()
}
