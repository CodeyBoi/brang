use std::collections::VecDeque;

use crate::parse::num;
use regex::Regex;
use Token::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Print,
    // Signatures
    FuncSig,
    VarSig,
    // Conditionals
    Branch,
    If,
    Else,
    ElseIf,
    // Loops
    While,
    For,
    // Literals
    NumLit(num),
    StrLit(String),
    // Identifiers
    Ident(String),
    // Expressions
    Expr,
    // Unary operators
    Not,
    // Binary operators
    Assign,
    BinOp(BiOp),
    // Syntax
    LBracket,
    RBracket,
    LParent,
    RParent,
    Semicolon,
    Comment(String),
    Whitespace,
    EOF,
    Err(String),
    Root,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BiOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    // Comparators
    Equal,
    NotEqual,
    LessOrEqual,
    GreaterOrEqual,
    Less,
    Greater,
    Invalid,
}

impl BiOp {
    fn from(s: &str) -> Self {
        use BiOp::*;
        match s {
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            "^" => Pow,
            "==" => Equal,
            "!=" => NotEqual,
            "<=" => LessOrEqual,
            ">=" => GreaterOrEqual,
            "<" => Less,
            ">" => Greater,
            _ => Invalid,
        }
    }

    fn prio(op: BiOp) -> u8 {
        use BiOp::*;
        match op {
            Pow => 4,
            Mul => 3,
            Div => 3,
            Add => 2,
            Sub => 2,
            Equal => 1,
            NotEqual => 1,
            LessOrEqual => 1,
            GreaterOrEqual => 1,
            Less => 1,
            Greater => 1,
            Invalid => panic!("invalid operator"),
        }
    }

    fn is_assoc(op: BiOp) -> bool {
        use BiOp::*;
        match op {
            Mul => true,
            Add => true,
            Equal => true,
            NotEqual => true,
            LessOrEqual => true,
            GreaterOrEqual => true,
            Less => true,
            Greater => true,
            Sub => false,
            Div => false,
            Pow => false,
            Invalid => panic!("invalid operator"),
        }
    }
}

pub fn tokenize(program: String) -> VecDeque<Token> {
    let patterns: [(Token, &'static str); 21] = [
        // Keywords
        (Print,     r"print\s"),
        (FuncSig, r"fun\s"),
        (VarSig, r"var\s"),
        (ElseIf, r"else if\s"),
        (If, r"if\s"),
        (Else, r"else\s"),
        (While, r"while\s"),
        (For, r"for\s"),
        // Literals
        (NumLit(0), r"\d+"),
        (StrLit("".to_string()),    r#"("[^"]*"|'[^']*')"#),
        // Identifiers
        (Ident("".to_string()),     r"[\pL][\pL\d]*"),
        // Unary operators
        (Not, r"!"),
        // Binary operators
        (BinOp(BiOp::Invalid), r"(==|!=|<=|>=|<|>|\^|\+|-|\*|/)"),
        (Assign, r"="),
        // Syntax tokens
        (LBracket, r"\{"),
        (RBracket, r"\}"),
        (LParent, r"\("),
        (RParent, r"\)"),
        (Semicolon, r";"),
        (Comment("".to_string()), r"#.*"),
        (Whitespace, r"\s+"),
    ];

    let mut tokens = VecDeque::new();
    let patterns: Vec<(Token, Regex)> = patterns
        .iter()
        .map(|s| (s.0.clone(), Regex::new(&(r"^".to_owned() + s.1)).unwrap()))
        .collect();
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
                    NumLit(_) => tokens.push_back(NumLit(
                        m.as_str()
                            .parse()
                            .expect("Error while parsing numeric literal."),
                    )),
                    StrLit(_) => {
                        let mut strlit = m.as_str().chars();
                        strlit.next();
                        strlit.next_back();
                        tokens.push_back(StrLit(strlit.as_str().to_string()));
                    },
                    Ident(_) => tokens.push_back(Ident(m.as_str().to_string())),
                    BinOp(_) => tokens.push_back(BinOp(BiOp::from(m.as_str()))),
                    Comment(_) => tokens.push_back(Comment(m.as_str().to_string())),
                    Whitespace => (),
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
pub struct Node {
    pub token: Token,
    pub children: Vec<Node>, // 0: lhs, 1: rhs, ...
}

impl Node {
    fn new(token: Token, children: Vec<Node>) -> Self {
        Node { token, children }
    }

    fn leaf(token: Token) -> Self {
        Node::new(token, Vec::new())
    }
}

pub fn print_tree(node: &Node) {
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
    for (i, child) in node.children.iter().enumerate() {
        print_tree_recur(child, "", i == node.children.len() - 1);
    }
}

pub fn parse(mut tokens: VecDeque<Token>) -> Node {
    let mut statements = Vec::new();
    while let Some(node) = parse_next(&mut tokens) {
        statements.push(node);
    }
    Node::new(Root, statements)
}

fn parse_next(tokens: &mut VecDeque<Token>) -> Option<Node> {
    println!("--- TOKENS ---\n{:?}\n", tokens);
    while let Some(token) = tokens.front() {
        match token {
            If => return Some(parse_if(tokens)),
            VarSig | Ident { .. } => return Some(parse_assign(tokens)),
            Print => {
                let token = tokens.pop_front().unwrap();
                if let Some(StrLit(_)) = tokens.front() {
                    // TODO: Substitute with `parse_string_expr`
                    let strlit = Node::leaf(tokens.pop_front().unwrap());
                    tokens.pop_front();
                    return Some(Node::new(token, vec![strlit]));
                } else {
                    let expr = parse_numeric_expr(tokens);
                    return Some(Node::new(token, vec![expr]));
                }
            }
            Else => {
                eprintln!("else-statement must come after an if-statement");
                return None;
            },
            ElseIf => {
                eprintln!("else if-statement must come after an if-statement");
                return None;
            }
            RBracket => {
                tokens.pop_front();
                return None;
            }
            Comment(_) => {
                tokens.pop_front();
            }
            EOF => return None,
            _ => unimplemented!("node start at {:?}", token),
        }
    }
    None
}

fn parse_if(tokens: &mut VecDeque<Token>) -> Node {
    let mut children = Vec::new();
    children.push(parse_numeric_expr(tokens));
    if let Some(n) = parse_next(tokens) {
        children.push(n);
    } else {
        children.push(Node::leaf(Whitespace));
    }
    match tokens.front() {
        Some(Else) | Some(ElseIf) => children.push(parse_else(tokens)),
        _ => (),
    }
    Node::new(Branch, children)
}

fn parse_else(tokens: &mut VecDeque<Token>) -> Node {
    let token = tokens.pop_front().unwrap();
    match token {
        Else => {
            todo!("else");
        },
        ElseIf => {
            todo!("else if")
        },
        _ => panic!("expected else- or else if-statement"),
    }
}

fn parse_assign(tokens: &mut VecDeque<Token>) -> Node {
    let variable = if &VarSig == tokens.front().unwrap() {
        tokens.pop_front();
        Node::leaf(tokens.pop_front().unwrap())
    } else {
        Node::leaf(tokens.pop_front().unwrap())
    };
    if tokens.pop_front() != Some(Assign) {
        panic!("expected assign operator");
    }
    let expr = parse_numeric_expr(tokens);
    Node::new(Assign, vec![variable, expr])
}

fn parse_numeric_expr(tokens: &mut VecDeque<Token>) -> Node {
    let mut rpn_expr = Vec::new();
    let mut ops = Vec::new();
    while let Some(token) = tokens.pop_front() {
        match token {
            NumLit(_) => rpn_expr.push(Node::leaf(token)),
            Ident { .. } => rpn_expr.push(Node::leaf(token)),
            BinOp(o) => {
                while let Some(BinOp(other_op)) = ops.last() {
                    if (BiOp::is_assoc(o) && BiOp::prio(o) <= BiOp::prio(*other_op))
                        || (!BiOp::is_assoc(o) && BiOp::prio(o) < BiOp::prio(*other_op))
                    {
                        let op = ops.pop().unwrap();
                        rpn_expr.push(Node::leaf(op));
                    } else {
                        break;
                    }
                }
                ops.push(token);
            }
            LParent => ops.push(token),
            RParent => {
                while let Some(op) = ops.pop() {
                    if op == LParent {
                        break;
                    } else {
                        rpn_expr.push(Node::leaf(op));
                    }
                }
            }
            Semicolon | LBracket | RBracket => break,
            _ => (),
        }
    }

    while let Some(op) = ops.pop() {
        rpn_expr.push(Node::leaf(op));
    }
    Node::new(Expr, rpn_expr)
}

// fn parse_numeric_expr_old(tokens: &mut VecDeque<Token>) -> Node {
//     let mut nodes = Vec::new();
//     let mut ops = Vec::new();
//     while let Some(token) = tokens.pop_front() {
//         match token {
//             NumLit(_) => nodes.push(Node::leaf(token)),
//             Ident { .. } => nodes.push(Node::leaf(token)),
//             BinOp(op) => {
//                 while let Some(BinOp(other_op)) = ops.last() {
//                     if (BiOp::is_assoc(op) && BiOp::prio(op) <= BiOp::prio(*other_op))
//                         || (!BiOp::is_assoc(op) && BiOp::prio(op) < BiOp::prio(*other_op))
//                     {
//                         let op = ops.pop().unwrap();
//                         let rhs = nodes.pop()
//                             .expect("operator has no right hand operand");
//                         let lhs = nodes.pop()
//                             .expect("operator has no left hand operand");
//                         nodes.push(Node::new(op, vec![lhs, rhs]));
//                     } else {
//                         break;
//                     }
//                 }
//                 ops.push(token);
//             }
//             LParent => ops.push(token),
//             RParent => {
//                 while let Some(op) = ops.pop() {
//                     if op == LParent {
//                         break;
//                     } else {
//                         let rhs = nodes.pop().expect("operator has no right hand operand");
//                         let lhs = nodes.pop().expect("operator has no left hand operand");
//                         nodes.push(Node::new(op, vec![lhs, rhs]));
//                     }
//                 }
//             }
//             Semicolon | LBracket | RBracket => break,
//             _ => (),
//         }
//     }

//     while let Some(op) = ops.pop() {
//         let rhs = nodes.pop().expect("operator has no right hand operand");
//         let lhs = nodes.pop().expect("operator has no left hand operand");
//         nodes.push(Node::new(op, vec![lhs, rhs]));
//     }

//     if nodes.len() == 1 {
//         nodes.pop().unwrap()
//     } else {
//         panic!("invalid expression");
//     }
// }
