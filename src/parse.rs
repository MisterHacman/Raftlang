use crate::position::Position;
use crate::token::{Token, TokenType};
use crate::error::SyntaxError;
use crate::match_either;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
	Abstraction,
	Application,
	None,
	Variable
}

#[derive(Clone)]
pub struct Node {
	pub class: NodeType,
	pub data: String,
	pub token: Token,
	pub nodes: Vec<Node>
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "({:?}: {} {:#?})", self.class, self.data, self.nodes) }
}

pub fn parse_expr(tokens: &Vec<Token>, i: &mut usize, is_arg: bool, level: &mut usize) -> Result<Node, SyntaxError> {
	if *i >= tokens.len() {
		if *level != 0 {
		return Err(SyntaxError::create(
			"expected ending parenthesis",
			None,
			None,
			true,
			None
		))
		}
		return Ok(Node { class: NodeType::None, data: "".to_owned(), token: Token {
                        class: TokenType::Keyword,
                        data: "".to_owned(),
                        pos: Position { line: 0, col: 0, start: 0, end: 0, last: true, finished: false }
		}, nodes: vec![] })
	}
	let token = &tokens[*i];
	let node = match token.class {
		TokenType::Identifier => {
			*i += 1;
			Node { class: NodeType::Variable, data: token.data.to_owned(), token: token.to_owned(), nodes: vec![] }
        	},
		TokenType::Parenthesis => match_either!(parse_par(tokens, i, level)),
		TokenType::Keyword => match_either!(parse_sym(tokens, i, level))
	};
	if !is_arg {
		parse_args(tokens, i, node, level)
	} else {
		Ok(node)
	}
}

fn parse_def(tokens: &Vec<Token>, i: &mut usize, level: &mut usize) -> Result<Node, SyntaxError> {
	if *i >= tokens.len() {
		return if *level != 0 {
		Err(SyntaxError::create(
			"no ending to parentheses",
			None,
			None,
			true,
			None
		))
		} else {
		Err(SyntaxError::create(
			"expected name in definition",
			None,
			None,
			true,
			None
		))}
	}
	let token = &tokens[*i];
	if token.class == TokenType::Identifier {
		*i += 1;
		Ok(Node { class: NodeType::Variable, data: token.data.to_owned(), token: token.to_owned(), nodes: vec![] })
	} else {
		Err(SyntaxError { error: "expexted name in definition".to_owned(), pos: token.pos })
	}
}

fn parse_args(tokens: &Vec<Token>, i: &mut usize, node: Node, level: &mut usize) -> Result<Node, SyntaxError> {
	let mut app_node;
	let mut last_node = node;
	let mut current;
	while { current = match_either!(parse_expr(tokens, i, true, level)); current.class != NodeType::None } {
		app_node = Node { class: NodeType::Application, data: "".to_owned(), token: current.token.to_owned(), nodes: vec![last_node, current] };
		last_node = app_node;
	}
	Ok(last_node)
}

fn parse_par(tokens: &Vec<Token>, i: &mut usize, level: &mut usize) -> Result<Node, SyntaxError> {
	let token = &tokens[*i];
	if token.data == "(" {
		*i += 1;
		*level += 1;
		let expr = match_either!(parse_expr(tokens, i, false, level));
		if expr.class == NodeType::None {
		        return Err(SyntaxError { error: "no expression inside paretheses".to_owned(), pos: expr.token.pos })
	        } else {
                        Ok(expr)
	        }
	} else /* token.data == ")" */ {
		if *level == 0 { return Err(SyntaxError { error: "ending parentheses before starting them".to_owned(), pos: token.pos }) }
		*i += 1;
		*level -= 1;
                Ok(Node { class: NodeType::None, data: "".to_owned(), token: token.to_owned(), nodes: vec![] })
	}
}

fn parse_sym(tokens: &Vec<Token>, i: &mut usize, level: &mut usize) -> Result<Node, SyntaxError> {
	let token = &tokens[*i];
	if token.data == "\\" || token.data == "fn" {
		*i += 1;
		let var = match_either!(parse_def(tokens, i, level));
		let body = match_either!(parse_expr(tokens, i, false, level));
		if body.class == NodeType::None {
		        return Err(SyntaxError { error: "expected body to abstraction".to_owned(), pos: body.token.pos })
		}
		Ok(Node { class: NodeType::Abstraction, data: var.data, token: token.to_owned(), nodes: vec![body] })
	} else { Err(SyntaxError { error: "invalid symbol".to_owned(), pos: token.pos }) }
}
