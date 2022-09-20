#include "parser.h"

parser_t co;
chunk_t *parsing_chunk;

static chunk_t *current_chunk()
{
    return parsing_chunk;
}

static void initparser()
{
    co.haderror = false;
}

static char *readcode(const char *path)
{
    FILE   *f;
    char   *code;
    size_t  len;

    f = fopen(path, "r");
    if (!f) {
        printf("Error: Couldn't read from file \"%s\"\n", path);
        exit(1);
    }
    fseek(f, 0, SEEK_END);
    len = ftell(f);
    rewind(f);
    code = malloc(len + 1);
    code[len] = 0;
    fread(code, sizeof *code, len, f);
    fclose(f);
    return code;
}

static void emitbyte(uint8_t byte)
{
    writechunk(current_chunk(), byte, co.prev.line);
}

static void emitbytes(uint8_t byte1, uint8_t byte2)
{
    emitbyte(byte1);
    emitbyte(byte2);
}

static void end_parser()
{
    emitreturn();
}

static void numlit()
{
    int value = str2int(&co.prev);
    emitconstant(value);
}

static void emitconstant(constval value) {
    int index = addconstant(current_chunk(), value);
    if (index <= UINT8_MAX) {
        emitbytes(OP_CONSTANT, index & 0xFF);
    } else if (index < MAX_VAR_COUNT) {
        emitbytes(OP_CONSTANT_LONG, index >> 16);
        emitbytes(index >> 8 & 0xFF, index & 0xFF);
    } else {
        error("Too many constants in one chunk.");
    }
}

static void emitreturn()
{
    emitbyte(OP_RETURN);
}

static void advance()
{
    co.prev = co.current;

    for (;;) {
        co.current = scantoken();
        if (co.current.type != ERR)
            break;

        errhere(co.current.start);
    }
}

/**
 * @brief Checks if next token is of type TYPE, and consumes it if it is.
 */
static bool match(tokentype type) {
    if (co.current.type != type)
        return false;

    advance();
    return true;
}

/**
 * @brief Checks if next token is of type TYPE, and consumes it if it is.
 * Otherwise throws error.
 * 
 * @param msg the error message to print if error occurs
 */
static void expect(tokentype type, const char *msg)
{
    if (co.current.type == type)
        advance();
    else 
        errhere(msg);
}

static void errhere(const char *msg)
{
    errorat(&co.current, msg);
}

static void error(const char *msg)
{
    errorat(&co.prev, msg);
}

static void errorat(token_t *token, const char *msg)
{
    fprintf(stderr, "[line %ld] Error", token->line);

    if (token->type == STOP) {
        fprintf(stderr, " at end");
    } else if (token->type == ERR) {
        // ignore
    } else {
        fprintf(stderr, " at '%.*s'", (int)token->len, token->start);
    }

    fprintf(stderr, ": %s\n", msg);
    co.haderror = true;
}

static int str2int(token_t *t)
{
    int i = 0;
    for (int j = 0; j < t->len; j++)
        i = 10 * i + t->start[j] - '0';
    return i;
}

static void expression()
{
    bool expr_ended = false;
    while (!expr_ended) {
        switch (co.prev.type)
        {
        case NUM_LIT:
            
            break;

        case IDENT:

        
        default:
            error("Unexpected token in expression");
            break;
        }

        advance();
    }
}

bool parse(const char *codepath, chunk_t *chunk)
{
    char *code;

    code = readcode(codepath);    
    initscanner(code);
    initparser();
    parsing_chunk = chunk;

    advance();
    while (co.current.type != STOP) {
        printtoken(co.current);
        advance();
        switch (co.prev.type)
        {
        case ERR:
            error("Unexpected token");
            break;

        case IDENT:
            expect(ASSIGN, "Expected '=' after identifier");
            expression();
            break;

        default:
            break;
        }
    }

    free(code);
    return !co.haderror;
}
