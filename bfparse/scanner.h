#ifndef brang_scanner_h
#define brang_scanner_h

#include <stdbool.h>
#include <string.h>
#include <ctype.h>

typedef enum _tokentype {
    STOP,       // EOF
    // control blocks
    ELSE,       // else
    BOOL_FALSE, // false
    FUNC_DEF,   // fn
    FOR,        // for
    IF,         // if
    RETURN,     // return
    BOOL_TRUE,  // true
    VAR_DEF,    // var
    WHILE,      // while
    // function definition
    // constant expressions
    NUM_LIT,    // /[0-9]+/
    STR_LIT,    // /".*"/
    // memory addresses
    IDENT,      // /[A-Za-z][A-Za-z0-9]*/
    // MEM_ADDRESS,
    // binary operators
    ASSIGN,     // =
    ADD,        // +
    ADD_ASSIGN, // +=
    SUB,        // -
    SUB_ASSIGN, // -=
    MUL,        // *
    MUL_ASSIGN, // *=
    DIV,        // /
    DIV_ASSIGN, // /=
    MOD,        // %
    MOD_ASSIGN, // &=
    // logical operators
    NOT,        // !
    BIT_AND,    // &
    AND,        // &&
    BIT_OR,     // |
    OR,         // ||
    XOR,        // ^
    EQ,         // ==
    NEQ,        // !=
    LEQ,        // <=
    GEQ,        // >=
    LESS,       // <
    GREATER,    // >
    // block delimiters
    LBRACKET,   // {
    RBRACKET,   // }
    LPARENT,    // (
    RPARENT,    // )
    // misc
    SEMIC,      // ;
    COMMA,      // ,
    DOT,        // .
    COMMENT,    // /#.*\n/
    // WHITESPACE, // /\s+/
    ERR,
    NUMBER_OF_TOKEN_TYPES,
} tokentype;

typedef struct _token_t {
    tokentype   type;
    const char *start;
    size_t      len;
    size_t      line;
} token_t;

typedef struct _scanner_t {
    const char *start;
    const char *current;
    size_t      line;
} scanner_t;

void initscanner(const char *code);
token_t scantoken();

static token_t maketoken(tokentype);
static token_t errtoken(const char*);

static bool match(char);

static token_t strtoken();
static token_t numtoken();

static tokentype checkkw(int offset, const char *kw, tokentype type);

static tokentype identtype();
static token_t identtoken();

#endif
