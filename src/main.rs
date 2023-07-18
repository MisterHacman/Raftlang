mod parse;
mod token;
mod error;
mod position;
mod utils;

use parse::parse_expr;
use token::tokenize;
use error::{FatalError, ReadError, Error};

use std::{fs, io::Read, env};

use crate::error::SyntaxError;

fn read_file(path: String) -> Result<String, ReadError> {
	let mut file = match_either!(fs::File::open(&path), ReadError { error: format!("could not open file `{}`", path) });
    	let mut buf = "".to_owned();
	match_either!(file.read_to_string(&mut buf), ReadError { error: format!("could not read file `{}`", path) });
    	Ok(buf)
}

fn run(args: &mut env::Args) -> Result<(), String> {
	if args.len() <= 1 {
		return Err(FatalError { error: "no input files" }.as_str(None))
	}
	args.next();
	let path = match_maybe!(args.next(), FatalError { error: "could not read input path" }.as_str(None));
	let buf = match_either!(read_file(path), None, ReadError);
	let tokens = match_either!(tokenize(buf.as_bytes()), Some(buf.as_bytes()), SyntaxError);
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
