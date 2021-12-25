use std::{
    collections::HashMap, 
    io::Write, 
    fs::OpenOptions,
};

pub(crate) use u8 as num;

use crate::token::Node;
use crate::token::Token;

pub fn to_brainfuck(input: Node, output: &str) -> Result<(), std::io::Error> {
    let mut c = Parser::new();
    
    for n in &input.children {
        c.process_node(n);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;
    
    write!(file, "{}", c.out)?;
    Ok(())
}

struct Parser {
    ptr:    usize,
    env:    HashMap<String, usize>,
    allocd: HashMap<usize, usize>,
    out:    String,
}

/**
 * Preferred instruction set
 * set dest const   // sets the value at `dest` to `const`
 * cpy src dest     // copies memory from `src` to `dest`
 * 
 * add lhs rhs      // adds `lhs` to `rhs` and writes to `rhs`
 * sub lhs rhs      // same, but subtraction
 * mul lhs rhs      // same, but multiplication
 * 
 * cmp lhs rhs
 * jmp label        // jumps to label (prob have to implement by inlining)
 */

impl Parser {

    const ARRAY_SIZE: usize = 30_000;

    // Compile functions

    fn process_node(&mut self, node: &Node) {
        use Token::*;
        match node {
            Node { token: If | Else | ElseIf | While | For, .. } => self.process_block(node),
            Node { token: Assign, .. }                           => self.process_assign(node),
            Node { token: Print, ..}                             => self.process_print(node),
            _ => unimplemented!("{:?}", node.token),
        }
    }

    fn process_assign(&mut self, node: &Node) {
        let adr = if let Token::Ident(name) = &node.children[0].token {
            self.assign(name)
        } else { panic!("no identifier in assign block") };
        self.process_expr_node(&node.children[1], adr);
    }

    /// Parses and evaluates an expression to compute a value.
    /// Then writes that value to `result`, which is assumed to be zeroed.
    fn process_expr_node(&mut self, node: &Node, result: usize) {
        use Token::*;
        match node.token {
            Expr => (),
            _ => panic!("expression node does not have the type Expr"),
        }

        let stack = self.malloc(node.children.len() / 2 + 1);
        let mut index = 0;

        for n in &node.children {
            // println!("stack at {}, ptr at {}", stack, self.ptr);
            // println!("env: {:?}, index: {}", self.env, index);
            match &n.token {
                NumLit(val) => {
                    self.set(stack + index, *val);
                    index += 1;
                }
                Ident(name) => {
                    let adr = *self.env.get(name).expect("identifier in expression was not defined");
                    self.cpy(adr, stack + index);
                    index += 1;
                },
                BinOp(op) => {
                    use crate::token::BiOp::*;

                    if index < 2 {
                        eprintln!("Error: too many operators in expression node");
                    }

                    let a = stack + index - 2;
                    let b = stack + index - 1;
                    match op {
                        Add             => self.consuming_add(b, a),
                        Sub             => self.consuming_sub(b, a),
                        Mul             => self.consuming_mul(b, a),
                        Div             => todo!("division"),
                        Pow             => todo!("exponent"),
                        Equal           => self.eq(b, a),
                        NotEqual        => self.neq(b, a),
                        // Negation because `geq` and `leq` writes result to the second address
                        LessOrEqual     => self.geq(b, a),
                        GreaterOrEqual  => self.leq(b, a),
                        Less            => self.gt(b, a),
                        Greater         => self.lt(b, a),
                        //
                        Invalid         => panic!("invalid operator"),
                    }
                    index -= 1;
                },
                StrLit(_) => todo!("string literals in expressions"),
                _ => eprintln!("Error: unexpected token in expression node"),
            }
        }

        if index != 1 {
            eprintln!("Error: too many numeric values in expression");
        }

        self.movval(stack, result);
        self.dealloc(stack);
    }

    fn process_block(&mut self, node: &Node) {
        use Token::*;
        match &node.token {
            If => self.process_if(node),
            While => todo!(),
            For => todo!(),
            ElseIf => eprintln!("Error: if-statement must come before else if-statement"),
            Else => eprintln!("Error: if-statement must come before else-statement"),
            _ => eprintln!("Error: invalid token for beginning of conditional block"),
        }
    }

    fn process_if(&mut self, node: &Node) {
        let flag = self.malloc(2);
        let else_flag = flag + 1;
        self.set(else_flag, 1);
        if let Some(expr) = node.children.first() {
            self.process_expr_node(expr, flag);
        } else { eprintln!("Error: no conditional expression after if-statement") }
        self.mov(flag);

        // zeroes `flag` and `else_flag` if expression is true
        self.out.push_str("[[-]>-<");
        
        // processes rest of nodes
        for child in node.children.iter().skip(1) {
            self.process_node(child);
        }

        self.out.push(']');
    }

    fn process_print(&mut self, node: &Node) {
        use Token::*;
        let child_node = node.children.first().unwrap();
        match &child_node.token {
            NumLit(v) => {
                
            },
            StrLit(s) => {
                let temp = self.calloc(1);
                let mut last_c = 0_u8;
                for c in s.chars() {
                    let c = c as u8;
                    if c >= last_c {
                        self.addconst(c - last_c, temp);
                    } else {
                        self.subconst(last_c - c, temp);
                    }
                    self.out.push('.');
                    last_c = c;
                }
                self.dealloc(temp);
            },
            Ident(_) => todo!(),
            Expr => {
                let result = self.malloc(1);
                self.process_expr_node(child_node, result);
                self.mov(result);
                self.out.push('.');
                self.dealloc(result);
            },
            _ => unimplemented!(),
        }

        // let result = self.malloc(1);
        // self.parse_expr_node(node.children.first().unwrap(), result);
        // self.mov(result);
        // self.out.push('.');
        // self.dealloc(result);
    }

    // Internal functions
    pub fn new() -> Self {
        Self {
            ptr:    0,
            env:    HashMap::new(),
            allocd: HashMap::new(),
            out:    String::new(),
        }
    }

    /// Alloc's memory for `size` contigous cells.
    fn malloc(&mut self, size: usize) -> usize {
        let mut adr = 0;
        let mut memlen = 0;
        while adr < Self::ARRAY_SIZE {
            if self.allocd.contains_key(&adr) {
                adr += self.allocd.get(&adr).unwrap();
                memlen = 0;
            } else {
                adr += 1;
                memlen += 1;
            }
            if memlen == size {
                adr = adr - size;
                self.allocd.insert(adr, size);
                return adr;
            }
        }
        panic!("out of memory");
    }

    /// As malloc, but zeroes all alloc'd cells.
    fn calloc(&mut self, size: usize) -> usize {
        let adr = self.malloc(size);
        for i in adr..adr + size {
            self.set(i, 0);
        }
        adr
    }

    /// Dealloc's the alloc'd memory at `adr`.
    /// Panics if `adr` is not allocated.
    fn dealloc(&mut self, adr: usize) {
        if let None = self.allocd.remove(&adr) {
            panic!("tried to deallocate non-alloc'd memory");
        }
    }

    /// Assigns the value `val` to the variable name `name` 
    /// and adds it to `env` if not already present.
    fn assign(&mut self, name: &str) -> usize {
        if let Some(temp) = self.env.get(name) {
            *temp
        } else {
            let temp = self.malloc(1);
            self.env.insert(name.to_string(), temp);
            temp
        }
    }

    /// Dealloc's a variable and removes it from `env`.
    fn free(&mut self, name: &str) {
        if let Some(adr) = self.env.remove(name) {
            self.dealloc(adr);
        } else {
            panic!("tried to free non alloc'd variable");
        }
    }

    /// Moves the pointer to `adr`.
    fn mov(&mut self, adr: usize) {
        let dir = if self.ptr < adr { '>' } else { '<' };
        let steps = (self.ptr as i32 - adr as i32).abs() as usize;
        for _ in 0..steps {
            self.out.push(dir);
        }
        self.ptr = adr;
    }


    fn movval(&mut self, src: usize, dest: usize) {
        self.mov(dest);
        self.out.push_str("[-]");
        self.mov(src);
        self.out.push('[');
        self.out.push('-');
        self.mov(dest);
        self.out.push('+');
        self.mov(src);
        self.out.push(']');
    }

    /// Sets the value at `dest` to `val`.
    fn set(&mut self, dest: usize, val: num) {
        self.mov(dest);
        self.out.push_str("[-]");
        for _ in 0..val {
            self.out.push('+');
        }
    }

    /// Copies the value at `src` to the value at `dest`.
    fn cpy(&mut self, src: usize, dest: usize) {
        let temp = self.calloc(1);

        self.set(dest, 0);
        self.mov(src);
        self.out.push_str("[-");
        self.mov(dest);
        self.out.push('+');
        self.mov(temp);
        self.out.push('+');
        self.mov(src);
        self.out.push(']');
        self.movval(temp, src);

        self.dealloc(temp);
    }

    /// Computes the sum of `lhs` and `rhs` and writes to `rhs`.
    fn add(&mut self, lhs: usize, rhs: usize) {
        let temp = self.malloc(1);
        self.cpy(lhs, temp);
        self.consuming_add(temp, rhs);
        self.dealloc(temp);
    }

    fn addconst(&mut self, c: num, adr: usize) {
        self.mov(adr);
        for _ in 0..c {
            self.out.push('+');
        }
    }

    /// Computes the sum of `lhs` and `rhs` and writes to `rhs`.
    /// 
    /// Consumes both operands, e.g. they are both unusable after this operation.
    fn consuming_add(&mut self, lhs: usize, rhs: usize) {
        self.mov(lhs);
        self.out.push_str("[-");
        self.mov(rhs);
        self.out.push('+');
        self.mov(lhs);
        self.out.push(']');
    }

    /// Computes the difference between `lhs` and `rhs` and writes to `rhs`
    /// (`rhs = rhs - lhs`).
    fn sub(&mut self, lhs: usize, rhs: usize) {
        let temp = self.malloc(1);
        self.cpy(lhs, temp);
        self.mov(temp);
        self.consuming_sub(temp, rhs);
        self.dealloc(temp);
    }

    fn subconst(&mut self, c: num, adr: usize) {
        self.mov(adr);
        for _ in 0..c {
            self.out.push('-');
        }
    }

    /// Computes the difference between `lhs` and `rhs` and writes to `rhs`
    /// (`rhs = rhs - lhs`). 
    /// 
    /// Consumes both operands, e.g. they are both 
    /// unusable after this operation.
    fn consuming_sub(&mut self, lhs: usize, rhs: usize) {
        self.mov(lhs);
        self.out.push_str("[-");
        self.mov(rhs);
        self.out.push('-');
        self.mov(lhs);
        self.out.push(']');
    }    

    /// Computes the product of `lhs` and `rhs` and writes to `rhs`.
    fn mul(&mut self, lhs: usize, rhs: usize) {
        let lhs_copy = self.malloc(1);
        self.cpy(lhs, lhs_copy);
        self.consuming_mul(lhs_copy, rhs);
        self.dealloc(lhs_copy);
    }

    /// Computes the product of `lhs` and `rhs` and writes to `rhs`.
    /// 
    /// Consumes both operands, e.g. they are both unusable after this operation.
    fn consuming_mul(&mut self, lhs: usize, rhs: usize) {
        let temp = self.malloc(1);

        self.movval(rhs, temp);
        self.mov(lhs);
        self.out.push_str("[-[-");  
        self.add(temp, rhs);
        self.mov(lhs);
        self.out.push(']');
        self.consuming_add(temp, rhs);
        self.mov(lhs);
        self.out.push(']');
        
        self.dealloc(temp);
    }

    /// Tests if `lhs` and `rhs` are equal.
    /// Will set a cell to 0 or 1 and end with the pointer on that cell.
    /// 
    /// NOTE! This cell will not be alloc'd, so use it immediately.
    fn eq(&mut self, lhs: usize, rhs: usize) {
        let result = self.calloc(1);

        // writes `lhs - rhs` into `temp`,
        self.cpy(lhs, result);    
        self.consuming_sub(rhs, result);
        self.mov(result);

        // sets result to 0 if `temp` != 0, and sets `temp` to zero
        self.out.push_str("[[-]");
        self.mov(rhs);
        self.out.push('-');
        self.mov(result);
        self.out.push(']');
        self.mov(rhs);
        self.out.push('+');
        self.dealloc(result);
    }

    /// Tests if `lhs` and `rhs` are not equal.
    /// Will set a cell to 0 or 1 and end with the pointer on that cell.
    /// 
    /// NOTE! This cell will not be alloc'd, so use it immediately.
    fn neq(&mut self, lhs: usize, rhs: usize) {
        self.eq(lhs, rhs);                  // tests if `lhs` and `rhs` are equal
        self.out.push_str("[[-]-]+");       // negates the result, assumes wrapping
    }

    /// Tests if `lhs` is greater or equal to `rhs`.
    /// Will set a cell to 0 or 1 and end with the pointer on that cell.
    /// 
    /// NOTE! This cell will not be alloc'd, so use it immediately.
    fn geq(&mut self, lhs: usize, rhs: usize) {
        let result = self.malloc(6);        // initial layout: [0 1 0 a b 0]
        self.set(result + 1, 1);
        let a = result + 3;
        let b = result + 4;

        self.cpy(lhs, a);
        self.cpy(rhs, b);
        self.mov(a);

        self.out.push_str("+>+<");          // to handle the cases `a=0` and `b=0`
        self.out.push_str("[->-[>]<<]<");   // ends on `result + 2` if `a>=b`, else `result + 3`
        self.out.push_str("[-<+>]");        // sets `result` to 1 if `a>=b`
        self.out.push_str("<[-<]");         // moves to `result` (second block is in case of `a<b`)
        self.ptr = result;                  // set `ptr` manually
        self.movval(result, rhs);           // writes `result` to `rhs`
        self.dealloc(result);               // at end array is [result 0 0 X X 0]
    }


    /// Tests if `lhs` is greater than `rhs`.
    /// Will set `rhs` to 0 or 1.
    fn gt(&mut self, lhs: usize, rhs: usize) {
        self.leq(lhs, rhs);                 // tests if `lhs` is less or equal to `rhs`
        self.out.push_str("[[-]-]+");       // negates the result, assumes wrapping
    }

    /// Tests if `lhs` is less or equal to `rhs`.
    /// Will set a cell to 0 or 1 and end with the pointer on that cell
    /// 
    /// NOTE! This cell will not be alloc'd, so use it immediately.
    fn leq(&mut self, lhs: usize, rhs: usize) {
        self.geq(rhs, lhs);
    }

    /// Tests if `lhs` is less than `rhs`.
    /// Will set a cell to 0 or 1 and end with the pointer on that cell.
    /// 
    /// NOTE! This cell will not be alloc'd, so use it immediately.
    fn lt(&mut self, lhs: usize, rhs: usize) {
        self.geq(lhs, rhs);                 // tests if `lhs` is greater or equal to `rhs`
        self.out.push_str("[[-]-]+");       // negates the result, assumes wrapping        
    }

    // fn parse_set(&mut self, lhs: &str, rhs: &str) {
    //     let dest = if lhs.parse::<usize>().is_ok() {
    //         panic!("lhs in set can't be a constant value");
    //     } else if let Some(adr) = self.env.get(lhs) {
    //         *adr
    //     } else if lhs.chars().next().unwrap().is_alphabetic() {
    //         let adr = self.alloc(1);
    //         self.env.insert(lhs.to_string(), adr);
    //         adr
    //     } else {
    //         panic!("invalid variable name for dest \
    //             (must start with letter)");
    //     };
    //     if let Ok(v) = rhs.parse::<num>() {
    //         self.set(dest, v);
    //     } else {
    //         panic!("rhs in set must be a constant value");
    //     }
        
    // }

    // fn parse_cpy(&mut self, lhs: &str, rhs: &str) {
    //     let dest = if rhs.parse::<num>().is_ok() {
    //         panic!("cannot input number as dest for mov");
    //     } else if let Some(adr) = self.env.get(rhs) {
    //         *adr
    //     } else {
    //         let adr = self.alloc(1);
    //         self.env.insert(rhs.to_string(), adr);
    //         adr
    //     };
        
    //     if let Ok(value) = lhs.parse::<num>() {
    //         self.set(dest, value);
    //     } else if let Some(adr) = self.env.get(lhs) {
    //         let src = *adr;
    //         if src == dest {
    //             panic!("can't copy a memory location to itself");
    //         }
    //         self.cpy(src, dest);
    //     } else { panic!("src parameter is undefined"); };   
    // }

    // fn parse_add(&mut self, lhs: &str, rhs: &str) {
    //     let rhs = if rhs.parse::<num>().is_ok() {
    //         panic!("cannot input number as dest for add");
    //     } else if let Some(adr) = self.env.get(rhs) {
    //         *adr
    //     } else { panic!("dest operand must be set first"); };
        
    //     if let Ok(value) = lhs.parse::<num>() {
    //         self.movptr(rhs);
    //         for _ in 0..value {
    //             self.out.push('+');
    //         }
    //     } else if let Some(adr) = self.env.get(lhs) {
    //         let lhs = *adr;
    //         self.add(lhs, rhs);
    //     } else { panic!("src parameter is undefined"); }
    // }

    // fn parse_sub(&mut self, lhs: &str, rhs: &str) {
    //     let rhs = if rhs.parse::<num>().is_ok() {
    //         panic!("cannot input number as dest for sub");
    //     } else if let Some(adr) = self.env.get(rhs) {
    //         *adr
    //     } else { panic!("dest operand must be set first"); };
        
    //     if let Ok(value) = lhs.parse::<num>() {
    //         self.movptr(rhs);
    //         for _ in 0..value {
    //             self.out.push('-');
    //         }
    //     } else if let Some(adr) = self.env.get(lhs) {
    //         let lhs = *adr;
    //         if lhs == rhs {
    //             self.set(rhs, 0);
    //         } else {
    //             self.sub(lhs, rhs);
    //         }
    //     } else { panic!("src parameter is undefined"); }
    // }

    // fn parse_mul(&mut self, lhs: &str, rhs: &str) {
    //     let rhs = if rhs.parse::<num>().is_ok() {
    //         panic!("cannot input number as dest for mul");
    //     } else if let Some(adr) = self.env.get(rhs) {
    //         *adr
    //     } else { panic!("dest operand must be set first"); };
        
    //     if let Ok(value) = lhs.parse::<num>() {
    //         if value == 0 {
    //             self.set(rhs, 0);
    //         } else {
    //             let temp = self.alloc(1);
    //             self.cpy(rhs, temp);
    //             for _ in 0..value - 1 {
    //                 self.add(temp, rhs);
    //             }
    //         }
    //     } else if let Some(adr) = self.env.get(lhs) {
    //         let lhs = *adr;
    //         self.mul(lhs, rhs);
    //     } else { panic!("src parameter is undefined"); }
    // }

    // fn parse_putd(&mut self, adr: usize) {

    // }

    // fn parse_puts(&mut self, arg: &str) {
    //     if arg.starts_with('\'') {
    //         let temp = self.alloc(1);
    //         for c in arg[1..].chars() {
    //             self.set(temp, c as u8);
    //             self.movptr(temp);
    //             self.out.push('.');
    //         }
    //         self.free(temp);
    //     } else if let Some(adr) = self.env.get(arg) {
    //         let adr = *adr;
    //         self.movptr(adr);
    //         self.out.push('.');
    //     } else {
    //         panic!("variable not set");
    //     }
    // }

    // /// Tests if `lhs` is equal to `rhs`. Ends with pointer on the result.
    // fn parse_teq(&mut self, lhs: &str, rhs: &str) {
    //     let lhs = if let Some(adr) = self.env.get(lhs) {
    //         *adr
    //     } else {
    //         panic!("lhs is not defined");
    //     };

        
    // }
}
 