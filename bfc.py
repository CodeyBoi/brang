import sys

def main():
    print(r'#include <stdio.h>')
    print(r'int main() {')
    print(r'char  a[30000] = {0};') 
    print(r'char* p = a;')
    for line in sys.stdin:
        for c in line:
            if c == '>':
                print(r'p++;')
            elif c == '<':
                print(r'p--;')
            elif c == '+':
                print(r'(*p)++;')
            elif c == '-':
                print(r'(*p)--;')
            elif c == '.':
                print(r'putchar(*p);')
            elif c == ',':
                print(r'*p = getchar();')
            elif c == '[':
                print(r'while (*p) {')
            elif c == ']':
                print(r'}')
    
    print(r'}')

if __name__ == '__main__':
    main()