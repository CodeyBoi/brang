#ifndef brang_compiler_h
#define brang_compiler_h

#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "debug.h"
#include "scanner.h"
#include "bf.h"

#define BUF_SIZE (4096)

const uint8_t op_preds[] = {
    [DOT] = 1,
    [NOT] = 2,
    [MUL] = 3,
    [DIV] = 3,
    [MOD] = 3,
    [ADD] = 4,
    [SUB] = 4,
    [LESS] = 6,
    [GREATER] = 6,
    [LEQ] = 6,
    [GEQ] = 6,
    [EQ] = 7,
    [NEQ] = 7,
    [BIT_AND] = 8,
    [XOR] = 9,
    [BIT_OR] = 10,
    [AND] = 11,
    [OR] = 12,
    [ASSIGN] = 14,
    [ADD_ASSIGN] = 14,
    [SUB_ASSIGN] = 14,
    [MUL_ASSIGN] = 14,
    [DIV_ASSIGN] = 14,
    [MOD_ASSIGN] = 14,
    [COMMA] = 15,
};

typedef struct {
    token_t     current;
    token_t     prev;
    bool        haderror;
} parser_t;

bool parse(const char *codepath, chunk_t *chunk);

static void emitbyte(uint8_t byte);
static void emitconstant(constval constant);
static void emitreturn();

static void expression();
static void statement();
static void declaration();
static void block();

static void initparser();
static void advance();
static void errhere(const char *msg);
static void errorat(token_t*, const char *msg);
static void error(const char *msg);

static int str2int(token_t *t);

#endif
