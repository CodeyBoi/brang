use std::{io::{Read, BufRead}, fmt::{self, format, write}, collections::VecDeque};
use regex::Regex;

use Token::*;

fn main() {
    let mut program = String::with_capacity(8192);
    std::io::stdin().read_to_string(&mut program).unwrap();
    let tokens = tokenize(program);
    for token in &tokens {
        println!("{:?}", token);
    }

    let root = parse(tokens);
    print_tree(&root);
}



#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    // Signatures
    FuncSig,
    VarSig,
    // Conditionals
    If,
    Else,
    ElseIf,
    // Loops
    While,
    For,
    // Literals
    NumLit(u8),
    StrLit(String),
    // Identifiers
    Ident(String),
    // Unary operators
    Not,
    // Binary operators
    Comp(String),
    Assign,
    BinOp(String),
    // Syntax
    LBrack,
    RBrack,
    Semic,
    Comment(String),
    Whitespace,
    EOF,
    Err(String),
    Root,
    Unknown,
}

fn tokenize(program: String) -> VecDeque<Token> {
    let patterns: [(Token, &'static str); 19] = [
        // Keywords
        (FuncSig,                    r"fun\s"),
        (VarSig,                     r"var\s"),
        (If,                         r"if\s"),
        (Else,                       r"else\s"),
        (ElseIf,                     r"else if\s"),
        (While,                      r"while\s"),
        (For,                        r"for\s"),
        // Literals 
        (NumLit(0),                  r"\d+"),
        (StrLit("".to_string()),     r#"("[^"]*"|'[^']*')"#),
        // Identifiers
        (Ident("".to_string()),      r"[\pL][\pL\d]*"),
        // Unary operators
        (Not,                        r"!"),
        // Binary operators
        (Comp("".to_string()),       r"(==|!=|<=|>=)"),
        (BinOp("".to_string()),      r"[\^<>+\-*/]"),
        (Assign,                     r"="),
        // Syntax tokens
        (LBrack,                     r"\{"),
        (RBrack,                     r"\}"),
        (Semic,                      r";"),
        (Comment("".to_string()),    r"#.*"),
        (Whitespace,                 r"\s+"),
    ];

    let mut tokens = VecDeque::new();
    let patterns: Vec<(Token, Regex)> = patterns.iter().map(|s|
        (s.0.clone(), Regex::new(&(r"^".to_owned() + s.1)).unwrap())
    ).collect();
    let mut ptr = 0;
    // While not at EOF, find next token
    'main: while ptr < program.len() {
        // Get a &str to code after current position
        let buf = program.split_at(ptr).1;
        for (kind, re) in &patterns {
            if let Some(m) = re.find(buf) {
                // Match found! Move pointer forward for next token.
                ptr += m.end();
                match *kind {
                    NumLit(_)   => tokens.push_back(NumLit(m.as_str().parse()
                                        .expect("Error while parsing numeric literal."))),
                    StrLit(_)   => tokens.push_back(StrLit(m.as_str().to_string())),
                    Ident(_)    => tokens.push_back(Ident(m.as_str().to_string())),
                    BinOp(_)    => tokens.push_back(BinOp(m.as_str().to_string())),
                    Comment(_)  => tokens.push_back(Comment(m.as_str().to_string())),
                    Comp(_)     => tokens.push_back(Comp(m.as_str().to_string())),
                    Whitespace  => {},
                    _ => tokens.push_back(kind.clone()),
                }
                // Find next token.
                continue 'main;
            }
        }
        // If no match is found, add the first char in the buffer as an `Err`.
        tokens.push_back(Err(buf.chars().next().unwrap_or(' ').to_string()));
        ptr += 1;
    }
    // Reached end of program, so push an `EOF` token and quit.
    tokens.push_back(EOF);
    tokens
}

#[derive(Debug)]
struct Node {
    token:      Token,
    children:   Vec<Node>,      // 0: lhs, 1: rhs, ...
}

impl Node {
    fn new(token: Token, children: Vec<Node>) -> Self {
        Node { token, children }
    }

    fn leaf(token: Token) -> Self {
        Node::new(token, Vec::new())
    }
}

fn print_tree(node: &Node) {
    fn print_tree_recur(node: &Node, indent: &str, last: bool) {
        let indent = if last {
            println!("{}└╴{:?}", indent, node.token);
            format!("{}  ", indent)
        } else {
            println!("{}├╴{:?}", indent, node.token);
            format!("{}│ ", indent)
        };
        for (i, child) in node.children.iter().enumerate() {
            print_tree_recur(child, &indent, i == node.children.len() - 1);
        }
    }
    println!("{:?}", node.token);
    for child in &node.children {
        print_tree_recur(child, "", true);
    }
}

fn parse(mut tokens: VecDeque<Token>) -> Node {
    let mut statements = Vec::new();
    while let Some(token) = &mut tokens.front() {
        match token {
            FuncSig => todo!(),
            VarSig => {
                statements.push(parse_assign(&mut tokens));
            },
            LBrack => todo!(),
            Semic => todo!(),
            Whitespace => todo!(),
            Unknown => todo!(),
            EOF => break,
            _ => todo!(),
        }
    }
    Node::new(Root, statements)
}

fn parse_assign(tokens: &mut VecDeque<Token>) -> Node {
    if tokens.pop_front() != Some(VarSig) {
        panic!("Expected variable signature 'var'.");
    }
    let variable = if let Some(var) = tokens.pop_front() {
        Node::leaf(var)
    } else {
        panic!("Expected identifier.")
    };
    if tokens.pop_front() != Some(Assign) {
        panic!("Expected assign operator '='.")
    }
    let expr = parse_expr(tokens);
    Node::new(Assign, vec![variable, expr])
}

fn parse_expr(tokens: &mut VecDeque<Token>) -> Node {
    let token = tokens.front().expect("Unexpected EOF while parsing.");
    match token {
        NumLit(_) => {
            parse_numeric_expr(tokens)
        },
        _ => panic!("Unexpected or unimplemented token. Expected expression.")
    }
}

fn parse_numeric_expr(tokens: &mut VecDeque<Token>) -> Node {

    fn prio(op: &str) -> u8 {
        match op {
            "^" => 4,
            "*" => 3,
            "/" => 3,
            "+" => 2,
            "-" => 2,
            _=> panic!("Found undefined operator."),
        }
    }

    fn is_assoc(op: &str) -> bool {
        match op {
            "*" | "+" => true,
            "^" | "/" | "-" => false,
            _ => panic!("Found undefined operator.")
        }
    }

    let mut nodes = Vec::new();
    let mut ops = Vec::new();
    while let Some(token) = tokens.pop_front() {
        match token {
            NumLit(_) => nodes.push(Node::leaf(token)),
            BinOp(ref op) => {
                while let Some(BinOp(other_op)) = ops.last() {
                    if (is_assoc(op) && prio(op) <= prio(other_op)) 
                        || (!is_assoc(op) && prio(op) < prio(other_op))
                    {
                        let op = ops.pop().unwrap();
                        let rhs = nodes.pop()
                            .expect("Operator has no right hand operand.");
                        let lhs = nodes.pop()
                            .expect("Operator has no left hand operand.");
                        nodes.push(Node::new(op, vec![lhs, rhs]));
                    } else {
                        break;
                    }
                }
                ops.push(token);
            },
            Semic => break,
            _ => {}
        }
    }

    while let Some(op) = ops.pop() {
        let rhs = nodes.pop()
            .expect("Operator has no right hand operand.");
        let lhs = nodes.pop()
            .expect("Operator has no left hand operand.");
        nodes.push(Node::new(op, vec![lhs, rhs]));
    }

    if nodes.len() == 1 {
        nodes.pop().unwrap()
    } else {
        panic!("Invalid expression.");
    }
}

fn eval_numeric_expr(expr: Expr) -> u8 {
    match expr {
        Expr::NumLiteral(n) => n,
        Expr::UnaOp(_) => todo!(),
        Expr::BinOp(_) => todo!(),
    }
}

enum Expr {
    NumLiteral(u8),
    UnaOp(Box<UnaExpr>),
    BinOp(Box<BinExpr>),
}

enum UnaOp {
    Not,
    Neg,
}

struct UnaExpr {
    rhs: Expr,
    op: UnaOp,
}

enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
}

struct BinExpr {
    lhs: Expr, rhs: Expr,
    op: BinOp,
}

