use std::collections::HashMap;
use std::io::Write;
use std::fs::OpenOptions;

pub(crate) use u8 as num;

use crate::token::Node;
use crate::token::Token;

pub fn to_brainfuck(root: Node, output: &str) -> Result<(), std::io::Error> {

    const CODE_WIDTH: usize = 80;

    let mut compiler = Compiler::new();   
    compiler.process_node(&root);

    let out_capacity = compiler.out.len() + compiler.out.len() / CODE_WIDTH + 1;
    let mut out = String::with_capacity(out_capacity);

    for (i, ch) in compiler.out.chars().enumerate() {
        if i != 0 && i % CODE_WIDTH == 0 {
            out.push('\n');
        }
        out.push(ch);
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;
    
    write!(file, "{}", out)?;
    Ok(())
}

struct Compiler {
    ptr:    usize,
    env:    HashMap<String, usize>,
    allocd: HashMap<usize, usize>,
    out:    String,
}

impl Compiler {

    const ARRAY_SIZE: usize = 30_000;

    // Compile functions

    fn process_node(&mut self, node: &Node) {
        use Token::*;
        for n in &node.children {
            match n {
                Node { token: Branch,  .. } => self.process_branch(n),
                Node { token: Assign,  .. } => self.process_assign(n),
                Node { token: Print,   .. } => self.process_print(n),
                Node { token: GetChar, .. } => self.process_getchar(n),
                _ => unimplemented!("{:?}", n.token),
            }
        }
    }

    fn process_branch(&mut self, node: &Node) {
        let expr = &node.children[0];
        let if_body = &node.children[1];
        let else_body = &node.children[2];

        // `if_flag` and `else_flag` decides which bodies to run
        let if_flag = self.malloc(2);
        let else_flag = if_flag + 1;
        self.set(else_flag, 1);

        // write expression result into `if_flag`
        self.process_expr_node(expr, if_flag);

        // if `if_flag` is non zero then set `if_flag` and `else_flag` to zero
        self.mov(if_flag);
        self.out.push_str("[[-]>-<");
        self.process_node(if_body);
        self.mov(if_flag);
        self.out.push(']');

        // if `else_flag` is non zero then run else body
        self.mov(else_flag);
        self.out.push_str("[-");
        self.process_node(else_body);
        self.mov(else_flag);
        self.out.push(']');

        self.dealloc(if_flag);
    }

    fn process_assign(&mut self, node: &Node) {
        let adr = if let Token::Ident(name) = &node.children[0].token {
            self.assign_byte(name)
        } else { panic!("no identifier in assign block") };
        self.process_expr_node(&node.children[1], adr);
    }

    /// Parses and evaluates an expression to compute a value.
    /// Then writes that value to `result`, which is assumed to be zeroed.
    fn process_expr_node(&mut self, node: &Node, result: usize) {
        use Token::*;
        if node.token != Expr {
            panic!("expression node does not have the type Expr");
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

    fn process_getchar(&mut self, node: &Node) {
        let adr = if let Token::Ident(name) = &node.children[0].token {
            self.assign_byte(name)
        } else { panic!("no identifier in getchar node") };
        self.mov(adr);
        self.out.push(',');
    }

    fn process_print(&mut self, node: &Node) {
        use Token::*;
        let child_node = node.children.first().unwrap();
        match &child_node.token {
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
    fn assign_byte(&mut self, name: &str) -> usize {
        if let Some(temp) = self.env.get(name) {
            *temp
        } else {
            let temp = self.calloc(1);
            self.env.insert(name.to_string(), temp);
            temp
        }
    }

    fn assign_array(&mut self, name: &str, size: usize) {
        
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
    /// Consumes both operands, e.g. they are both unusable after this operation.
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

    /// Writes a 1 to `adr` if it's zero, and 0 otherwise. Assumes wrapping.
    fn not(&mut self, adr: usize) {
        let temp = self.calloc(1);

        // Sets `temp` to 1 if `adr` is zero, and 0 otherwise. Also zeroes `adr`.
        self.mov(adr);
        self.out.push_str("[[-]");
        self.mov(temp);
        self.out.push('-');
        self.mov(adr);
        self.out.push(']');
        self.mov(temp);
        self.out.push('+');

        // Moves value from `temp` to `adr`.
        self.out.push_str("[-");
        self.mov(adr);
        self.out.push('+');
        self.mov(temp);
        self.out.push(']');

        self.dealloc(temp);
    }

    /// Tests if `lhs` and `rhs` are equal.
    /// Will set `rhs` to 0 or 1.
    fn eq(&mut self, lhs: usize, rhs: usize) {
        let temp = self.calloc(1);

        // writes `lhs - rhs` into `temp`,
        self.cpy(lhs, temp);    
        self.consuming_sub(temp, rhs);
        self.not(rhs);

        self.dealloc(temp);
    }

    /// Tests if `lhs` and `rhs` are not equal.
    /// Will set `rhs` to 0 or 1.
    fn neq(&mut self, lhs: usize, rhs: usize) {
        self.eq(lhs, rhs);          // tests if `lhs` and `rhs` are equal
        self.not(rhs);              // negates the result
    }

    /// Tests if `lhs` is greater or equal to `rhs`.
    /// Will set `rhs` to either 1 or 0.
    fn geq(&mut self, lhs: usize, rhs: usize) {
        //                      v
        // initial layout: [0 1 0 a b 0]
        let result = self.malloc(6);
        let a = result + 3;
        let b = result + 4;

        self.set(result, 0);
        self.set(result + 1, 1);
        self.set(result + 2, 0);
        self.cpy(lhs, a);
        self.cpy(rhs, b);
        self.set(result + 5, 0);

        self.mov(a);
        self.out.push_str("+>+<");          // to handle the cases `a=0` and `b=0`
        self.out.push_str("[->-[>]<<]<");   // ends on `result + 1` if `a>=b`, else `result + 2`
        self.out.push_str("[<+>>]<<");      // sets `result` to 1 if `a>=b` and moves to result
        self.ptr = result;                  // set `ptr` manually
        self.movval(result, rhs);           // writes `result` to `rhs`
        self.dealloc(result);               // at end array is [result 1 0 X X 0]
    }


    /// Tests if `lhs` is greater than `rhs`.
    /// Will set `rhs` to 0 or 1.
    fn gt(&mut self, lhs: usize, rhs: usize) {
        self.leq(lhs, rhs);         // tests if `lhs` is less or equal to `rhs`
        self.not(rhs);              // negates the result
    }

    /// Tests if `lhs` is less or equal to `rhs`.
    /// Will set `rhs` to 0 or 1.
    fn leq(&mut self, lhs: usize, rhs: usize) {
        let temp = self.malloc(1);
        self.cpy(lhs, temp);
        self.geq(rhs, temp);
        self.movval(temp, rhs);
    }

    /// Tests if `lhs` is less than `rhs`.
    /// Will set a cell to 0 or 1 and end with the pointer on that cell.
    /// 
    /// NOTE! This cell will not be alloc'd, so use it immediately.
    fn lt(&mut self, lhs: usize, rhs: usize) {
        self.geq(lhs, rhs);         // tests if `lhs` is greater or equal to `rhs`
        self.not(rhs);              // negates the result
    }
}
 
