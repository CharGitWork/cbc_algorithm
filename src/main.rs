// use symetric_cryptography::source_lib::storage::{Cli, CliTrait, RootTx, RootTrait};

// use std::io;

// use std::env;
// use std::io::Write;

fn main() {
    // let cli = Cli::create(env::args());
    // let mut root = CliTrait::calling_root(cli);
    // <RootTx as RootTrait>::string_to_bytes(&mut root);

    // let bytes = root.bytes_vers;

    // io::stdout().write_all(&bytes).unwrap();

    let a = 2312;
    let b = 128;

    println!("a / b == {}", a/b);
    println!("a % b == {}", a%b);
}