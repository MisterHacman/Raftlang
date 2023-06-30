mod parse;
mod token;
mod error;
mod position;
mod utils;

use parse::parse_expr;
use token::tokenize;
use error::{FatalError, ReadError, Error};

use std::{fs, io::Read, env};

fn read_file(path: String) -> Result<String, ReadError> {
    let mut file = match fs::File::open(&path) {
        Ok(ok) => ok,
        Err(_) => { return Err(ReadError { error: format!("could not open file `{}`", path) }) }
    };
    let mut buf = "".to_owned();
    match file.read_to_string(&mut buf) {
        Ok(_) => {},
        Err(_) => { return Err(ReadError { error:  format!("could not read file `{}`", path) }) }
    }
    Ok(buf)
}

fn run(args: &mut env::Args) -> Result<(), String> {
    if args.len() <= 1 {
        return Err(FatalError { error: "no input files".to_owned() }.as_str(None).to_owned())
    }
    args.next();
    let path;
    match args.next() {
        Some(some) => path = some,
        None => { return Err(FatalError { error: "could not read input path".to_owned() }.as_str(None).to_owned()) }
    }
    let buf;
    match read_file(path) {
        Ok(ok) => buf = ok,
        Err(err) => { return Err(err.as_str(None).to_owned()) }
    }
    let tokens;
    match tokenize(buf.as_bytes()) {
        Ok(ok) => tokens = ok,
        Err(err) => { return Err(err.as_str(Some(buf.as_bytes())).to_owned()) }
    }
    let ast;
    match parse_expr(&tokens, &mut 0, false, &mut 0) {
        Ok(ok) => ast = ok,
        Err(err) => { return Err(err.as_str(Some(buf.as_bytes())).to_owned()) }
    }
    println!("{:?}", tokens);
    println!("{:?}", ast);
    Ok(())
}

fn main() -> () {
    match run(&mut env::args()) {
        Ok(_) => {},
        Err(err) => eprintln!("{}", err)
    }
}
