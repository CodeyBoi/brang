#ifndef brang_chunk_h
#define brang_chunk_h

#include <stdint.h>
#include <stdlib.h>

#include "constval.h"

#define MAX_VAR_COUNT (1 << 24)

typedef enum {
    OP_CONSTANT,
    OP_CONSTANT_LONG,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_NEGATE,
    OP_PRINT,
    OP_RETURN
} opcode;

typedef struct {
    size_t      len;
    size_t      cap;
    uint8_t    *data;
    int        *lines;
    constarray  constants;
} chunk_t;

void initchunk(chunk_t*);
void freechunk(chunk_t*);
void writechunk(chunk_t*, uint8_t byte, int line);
void writeconstant(chunk_t*, constval, int line);

int addconstant(chunk_t*, constval);

#endif
