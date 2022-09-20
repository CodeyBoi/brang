#include "scanner.h"

static token_t maketoken(tokentype);
static token_t errtoken(const char*);
static bool match(char);
static token_t strtoken();
static token_t numtoken();
static tokentype checkkw(int offset, const char *kw, tokentype type);
static tokentype identtype();
static token_t identtoken();

#define isEOF()             (*sc.current == '\0')
#define advance()           (*sc.current++)
#define peek()              (*sc.current)
#define peekahead(offset)   (isEOF() ? '\0' : sc.current[(offset)])
#define choose(c, a, b)     (maketoken(match((c)) ? (a) : (b)))

static scanner_t sc;

void initscanner(const char *code) {
    sc.start = code;
    sc.current = sc.start;
    sc.line = 1;
}

static token_t maketoken(tokentype t) {
    token_t token;
    token.type = t;
    token.start = sc.start;
    token.len = (size_t)(sc.current - sc.start);
    token.line = sc.line;
    return token;
}

static token_t errtoken(const char *msg) {
    token_t token;
    token.type = ERR;
    token.start = msg;
    token.len = strlen(msg);
    token.line = sc.line;
    return token;
}

/**
 * @brief Checks if next char is C, and consumes it if it is.
 */
static bool match(char c) {
    if (peek() != c || isEOF())
        return false;
    sc.current++;
    return true;
}

static token_t strtoken() {
    char c;
    while ((c = advance()) != '"' && !isEOF())
        sc.line += (c == '\n');

    if (isEOF())
        return errtoken("Unterminated string");

    return maketoken(STR_LIT);
}

static token_t numtoken() {
    // Consume characters until we find a non-digit
    while (isdigit(peek()))
        advance();

    // If we have the form X.X where X are digits we have a 
    // fractional number
    if (peek() == '.' && isdigit(peekahead(1))) {
        advance();
        // Keep consuming chars after the dot
        while (isdigit(peek())) 
            advance();
    }
    return maketoken(NUM_LIT);
}

static tokentype checkkw(int offset, const char *kw, tokentype type) {
    size_t len = strlen(kw);
    if (sc.current - sc.start == len + offset
        && memcmp(sc.start + offset, kw, len) == 0)
    {
        return type;
    }
    return IDENT;
}

static tokentype identtype() {
    switch (sc.start[0]) {
        case 'e': return checkkw(1, "lse", ELSE);
        case 'f':
            if (sc.current - sc.start > 1) {
                switch (sc.start[1]) {
                    case 'a': return checkkw(2, "lse", BOOL_FALSE);
                    case 'n': return checkkw(2, "", FUNC_DEF);
                    case 'o': return checkkw(2, "r", FOR);
                }
            }
            break;
        case 'i': return checkkw(1, "f", IF);
        case 'r': return checkkw(1, "eturn", RETURN);
        case 't': return checkkw(1, "rue", BOOL_TRUE);
        case 'v': return checkkw(1, "ar", VAR_DEF);
        case 'w': return checkkw(1, "hile", WHILE);
    }
    return IDENT;
}

static token_t identtoken() {
    while (isalnum(peek()) || peek() == '_')
        advance();
    return maketoken(identtype());
}

token_t scantoken() {
    char c;


    // Skip whitespace
    while (isspace(c = advance())) {
        // Increment linecount if newline appears
        sc.line += (c == '\n');}

    // Skip comments
    if (c == '/' && match('/')) {
        while (peek() != '\n' && !isEOF())
            advance();
        return scantoken();
    }

    sc.start = sc.current - 1;

    if (c == '\0')
        return maketoken(STOP);

    switch (c) {
        // One char tokens
        case '(': return maketoken(LPARENT);
        case ')': return maketoken(RPARENT);
        case '{': return maketoken(LBRACKET);
        case '}': return maketoken(RBRACKET);
        case ';': return maketoken(SEMIC);
        case ',': return maketoken(COMMA);
        case '.': return maketoken(DOT);
        case '^': return maketoken(XOR);

        // One or two char tokens
        case '+': return choose('=', ADD_ASSIGN, ADD);
        case '-': return choose('=', SUB_ASSIGN, SUB);
        case '*': return choose('=', MUL_ASSIGN, MUL);
        case '/': return choose('=', DIV_ASSIGN, DIV);
        case '%': return choose('=', MOD_ASSIGN, MOD);
        
        case '<': return choose('=', LEQ, LESS);
        case '>': return choose('=', GEQ, GREATER);

        case '=': return choose('=', EQ, ASSIGN);
        case '!': return choose('=', NOT, NEQ);
        case '&': return choose('&', AND, BIT_AND);
        case '|': return choose('|', OR, BIT_OR);
    }

    if (c == '"')                       return strtoken();
    else if (isdigit(c))                return numtoken();
    else if (isalpha(c) || c == '_')    return identtoken();

    return errtoken("Unexpected character");
}
