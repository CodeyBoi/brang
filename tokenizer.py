import re
import sys
from typing import Pattern
from bffuncs import BrainFuck

def main():
    program = ' '.join(line for line in sys.stdin)

    for type, id in tokenize(program):
        print(f'{type + ":": <12}{id}')

    # bf = BrainFuck()
    # bf.puts('THIS IS A BIG ASS TEST\n')
    # bf.assign('a', 35)  # a = 35
    # bf.assign('b', 33)  # b = 35
    # bf.puts('a=')       # print(f"{a=}\n{b=}")
    # bf.putv('a')
    # bf.puts('\n')
    # bf.puts('b=')
    # bf.putv('b')
    # bf.puts('\n')
    # bf.addvars('a', 'b', 'a') # a = a + b
    # bf.puts('a=')
    # bf.putv('a')
    # bf.puts('\n')
    # bf.puts('b=')
    # bf.putv('b')
    # bf.puts('\n')


PATTERN_STRINGS = [
    # Keywords
    ('FUNSIG',      r'fun\s'),
    ('VARSIG',      r'var\s'),
    ('COND',        r'(else if|else|if)\s'),
    ('LOOP',        r'(for|while)\s'),
    # Literals
    ('NUMLIT',      r'\d+'),
    ('STRLIT',      r'(["\'])[^\1]*\1'),
    # Identifier
    ('IDENT',       r'[a-zA-Z_]\w*'),
    # Syntax tokens
    ('UNOP',        r'!'),
    ('BIOP',        r'(==|!=|<=|>=|[<>+\-*/=])'),
    ('BRACK',       r'[{}]'),
    ('SEMIC',       r';'),
    ('COMNT',       r'#.*'),
]


def tokenize(program):
    ptr = 0
    tokens = []
    patterns = []
    for type, pattern in PATTERN_STRINGS:
        patterns.append((type, re.compile(r'\s*' + pattern)))

    token, size = next_token(program[ptr:], patterns)
    while token[0] not in ['EOF', 'ERROR']:
        ptr += size
        tokens.append(token)
        token, size = next_token(program[ptr:], patterns)
    tokens.append(token)
    return tokens


def next_token(program, patterns):
    for type, pattern in patterns:
        print(pattern)
        match = pattern.match(program)
        if match:
            value = match.group().lstrip().rstrip()
            if type == 'NUM':
                value = int(value)
            elif type == 'STR':
                value = value[1:-1]
            return ((type, value), match.span()[1])
    if program.isspace():
        return (('EOF', ''), 0)

    return (('ERR', 'Syntax error'), 0)

def parse(tokens):
    parser = Parser(tokens)
    parser.parse()

class Parser:

    def __init__(self, tokens):
        self.idx = 0
        self.tokens = tokens

    def parse(self):
        # def peek(n):
        #     return tokens[idx + n]
        
        # def eat(n):
        #     idx += 1
        #     return tokens[idx - 1]
        
        while self.idx < len(self.tokens):
            type, value = self.tokens[self.idx]
            if type == 'DCLR':
                if value.startswith('fun'):
                    pass
                    # TODO: func definition
                elif value.startswith('var'):
                    self.parseAssignment()
                    

if __name__ == '__main__':
    main()