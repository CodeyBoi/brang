use std::{collections::HashMap, io::{BufReader, BufRead, Write}, fs::{File, OpenOptions}};

pub(crate) use u8 as num;

pub fn to_bf(input: &str, output: &str) -> Result<(), std::io::Error> {
    let mut parser = Parser::new();
    let assem = BufReader::new(File::open(input)?);
    for line in assem.lines() {
        let line = line.unwrap();
        // self.out.push_str(&format!("{:<25}", line));
        let tokens = tokenize(line);
        match tokens[0].as_str() {
            "set" =>    parser.parse_set(&tokens[1], &tokens[2]),
            "cpy" =>    parser.parse_cpy(&tokens[1], &tokens[2]),
            "add" =>    parser.parse_add(&tokens[1], &tokens[2]),
            "sub" =>    parser.parse_sub(&tokens[1], &tokens[2]),
            "mul" =>    parser.parse_mul(&tokens[1], &tokens[2]),
            "teq" =>    parser.parse_teq(&tokens[1], &tokens[2]),
            "putd" =>   todo!(),
            "puts" =>   parser.parse_puts(&tokens[1]),
            _ => (),
        }
        parser.out.push('\n');
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;
    
    write!(file, "{}", parser.out)?;
    Ok(())
}

fn tokenize(s: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\'' {
            token.push('\'');
            let mut string_c = chars.next().unwrap();
            while string_c != '\'' {
                if string_c == '\\' && chars.peek() == Some(&'n') {
                    token.push('\n');
                    chars.next();
                } else {
                    token.push(string_c);
                }
                string_c = chars.next().unwrap();
            }
            tokens.push(token);
            token = String::new();
        } else if !c.is_whitespace() {
            token.push(c);
        } else if !token.is_empty() {
            tokens.push(token);
            token = String::new();
        }
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
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
struct Parser {
    ptr:    usize,
    env:    HashMap<String, usize>,
    allocd: HashMap<usize, usize>,
    out:    String,
}

impl Parser {

    const ARRAY_SIZE: usize =           30_000;

    pub fn new() -> Self {
        Self {
            ptr:    0,
            env:    HashMap::new(),
            allocd: HashMap::new(),
            out:    String::new(),
        }
    }

    fn alloc(&mut self, size: usize) -> usize {
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

    fn free(&mut self, adr: usize) {
        assert!(self.allocd.contains_key(&adr));
        self.allocd.remove(&adr);
    }

    fn movptr(&mut self, adr: usize) {
        let dir = if self.ptr < adr { '>' } else { '<' };
        let steps = (self.ptr as i32 - adr as i32).abs() as usize;
        for _ in 0..steps {
            self.out.push(dir);
        }
        self.ptr = adr;
    }

    fn set(&mut self, dest: usize, val: num) {
        self.movptr(dest);
        self.out.push_str("[-]");
        for _ in 0..val {
            self.out.push('+');
        }
    }

    fn cpy(&mut self, src: usize, dest: usize) {
        let temp = self.alloc(1);
        self.set(dest, 0);
        self.movptr(src);
        self.out.push_str("[-");
        self.movptr(dest);
        self.out.push('+');
        self.movptr(temp);
        self.out.push('+');
        self.movptr(src);
        self.out.push(']');

        self.movptr(temp);
        self.out.push_str("[-");
        self.movptr(src);
        self.out.push('+');
        self.movptr(temp);
        self.out.push(']');
        self.free(temp);
    }

    fn add(&mut self, lhs: usize, rhs: usize) {
        let temp = self.alloc(1);
        self.cpy(lhs, temp);
        self.movptr(temp);
        self.out.push_str("[-");
        self.movptr(rhs);
        self.out.push('+');
        self.movptr(temp);
        self.out.push(']');
        self.free(temp);
    }

    fn sub(&mut self, lhs: usize, rhs: usize) {
        let temp = self.alloc(1);
        self.cpy(lhs, temp);
        self.movptr(temp);
        self.out.push_str("[-");
        self.movptr(rhs);
        self.out.push('-');
        self.movptr(temp);
        self.out.push(']');
        self.free(temp);
    }

    fn mul(&mut self, lhs: usize, rhs: usize) {
        let l_temp = self.alloc(1);
        let r_temp = self.alloc(1);
        self.cpy(lhs, l_temp);
        self.cpy(rhs, r_temp);
        self.movptr(l_temp);
        self.out.push_str("-[-");
        self.add(r_temp, rhs);
        self.movptr(l_temp);
        self.out.push(']');
        self.free(l_temp);
        self.free(r_temp);
    }

    fn modulo(&mut self, lhs: usize, rhs: usize) {

    }

    fn parse_set(&mut self, lhs: &str, rhs: &str) {
        let dest = if lhs.parse::<usize>().is_ok() {
            panic!("lhs in set can't be a constant value");
        } else if let Some(adr) = self.env.get(lhs) {
            *adr
        } else if lhs.chars().next().unwrap().is_alphabetic() {
            let adr = self.alloc(1);
            self.env.insert(lhs.to_string(), adr);
            adr
        } else {
            panic!("invalid variable name for dest \
                (must start with letter)");
        };
        if let Ok(v) = rhs.parse::<num>() {
            self.set(dest, v);
        } else {
            panic!("rhs in set must be a constant value");
        }
        
    }

    fn parse_cpy(&mut self, lhs: &str, rhs: &str) {
        let dest = if rhs.parse::<num>().is_ok() {
            panic!("cannot input number as dest for mov");
        } else if let Some(adr) = self.env.get(rhs) {
            *adr
        } else {
            let adr = self.alloc(1);
            self.env.insert(rhs.to_string(), adr);
            adr
        };
        
        if let Ok(value) = lhs.parse::<num>() {
            self.set(dest, value);
        } else if let Some(adr) = self.env.get(lhs) {
            let src = *adr;
            if src == dest {
                panic!("can't copy a memory location to itself");
            }
            self.cpy(src, dest);
        } else { panic!("src parameter is undefined"); };   
    }

    fn parse_add(&mut self, lhs: &str, rhs: &str) {
        let rhs = if rhs.parse::<num>().is_ok() {
            panic!("cannot input number as dest for add");
        } else if let Some(adr) = self.env.get(rhs) {
            *adr
        } else { panic!("dest operand must be set first"); };
        
        if let Ok(value) = lhs.parse::<num>() {
            self.movptr(rhs);
            for _ in 0..value {
                self.out.push('+');
            }
        } else if let Some(adr) = self.env.get(lhs) {
            let lhs = *adr;
            self.add(lhs, rhs);
        } else { panic!("src parameter is undefined"); }
    }

    fn parse_sub(&mut self, lhs: &str, rhs: &str) {
        let rhs = if rhs.parse::<num>().is_ok() {
            panic!("cannot input number as dest for sub");
        } else if let Some(adr) = self.env.get(rhs) {
            *adr
        } else { panic!("dest operand must be set first"); };
        
        if let Ok(value) = lhs.parse::<num>() {
            self.movptr(rhs);
            for _ in 0..value {
                self.out.push('-');
            }
        } else if let Some(adr) = self.env.get(lhs) {
            let lhs = *adr;
            if lhs == rhs {
                self.set(rhs, 0);
            } else {
                self.sub(lhs, rhs);
            }
        } else { panic!("src parameter is undefined"); }
    }

    fn parse_mul(&mut self, lhs: &str, rhs: &str) {
        let rhs = if rhs.parse::<num>().is_ok() {
            panic!("cannot input number as dest for mul");
        } else if let Some(adr) = self.env.get(rhs) {
            *adr
        } else { panic!("dest operand must be set first"); };
        
        if let Ok(value) = lhs.parse::<num>() {
            if value == 0 {
                self.set(rhs, 0);
            } else {
                let temp = self.alloc(1);
                self.cpy(rhs, temp);
                for _ in 0..value - 1 {
                    self.add(temp, rhs);
                }
            }
        } else if let Some(adr) = self.env.get(lhs) {
            let lhs = *adr;
            self.mul(lhs, rhs);
        } else { panic!("src parameter is undefined"); }
    }

    fn parse_putd(&mut self, adr: usize) {

    }

    fn parse_puts(&mut self, arg: &str) {
        if arg.starts_with('\'') {
            let temp = self.alloc(1);
            for c in arg[1..].chars() {
                self.set(temp, c as u8);
                self.movptr(temp);
                self.out.push('.');
            }
            self.free(temp);
        } else if let Some(adr) = self.env.get(arg) {
            let adr = *adr;
            self.movptr(adr);
            self.out.push('.');
        } else {
            panic!("variable not set");
        }
    }

    /// Tests if `lhs` is equal to `rhs`. Ends with pointer on the result.
    fn parse_teq(&mut self, lhs: &str, rhs: &str) {
        let lhs = if let Some(adr) = self.env.get(lhs) {
            *adr
        } else {
            panic!("lhs is not defined");
        };

        
    }
}
 