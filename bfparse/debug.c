#include <stdio.h>

#include "debug.h"

#define TOKENNAME(type) _TOKENNAMES[type]

const char ESC_CHARS[][2] = {
    'a', '\a',
    'b', '\b',
    'f', '\f',
    'n', '\n',
    'r', '\r',
    'v', '\v',
    '\\', '\\',
    '\"', '\"',
    '\'', '\'',
    '\0', '\0',
};

const char *_TOKENNAMES[] = {
    "STOP",       // EOF

    "ELSE",       // else
    "BOOL_FALSE", // false
    "FUNC_DEF",   // fn
    "FOR",        // for
    "IF",         // if
    "RETURN",     // return
    "BOOL_TRUE",  // true
    "VAR_DEF",    // var
    "WHILE",      // while
    "NUM_LIT",    // /[0-9]+/
    "STR_LIT",    // /".*"/
    "IDENT",      // /[A-Za-z][A-Za-z0-9]*/
    // MEM_ADDRESS,
    "ASSIGN",     // =
    "ADD",        // +
    "ADD_ASSIGN", // +=
    "SUB",        // -
    "SUB_ASSIGN", // -=
    "MUL",        // *
    "MUL_ASSIGN", // *=
    "DIV",        // /
    "DIV_ASSIGN", // /=
    "MOD",        // %
    "MOD_ASSIGN", // %=
    "NOT",        // !
    "BIT_AND",    // &
    "AND",        // &&
    "BIT_OR",     // |
    "OR",         // ||
    "XOR",        // ^
    "EQ",         // ==
    "NEQ",        // !=
    "LEQ",        // <=
    "GEQ",        // >=
    "LESS",       // <
    "GREATER",    // >
    "LBRACKET",   // {
    "RBRACKET",   // }
    "LPARENT",    // (
    "RPARENT",    // )
    "SEMIC",      // ;
    "COMMA",      // ,
    "DOT",        // .
    "COMMENT",    // /#.*\n/
    // WHITESPACE, // /\s+/
    "ERR",
    "NUMBER_OF_TOKEN_TYPES",
};

void printtoken(token_t t) {
    printf("%4ld %-10s %.*s\n", t.line, TOKENNAME(t.type), (int)t.len, t.start);
}

/**
 * @brief Prints all instructions in CHUNK.
 */
void disassem_chunk(chunk_t *chunk, const char *name) {
    printf("== %s ==\n", name);
    printf("%4s %4s %s\n", "offs", "line", "instruction");

    int offset = 0;
    while (offset < chunk->len)
        offset = disassem_instruction(chunk, offset);
}

/**
 * @brief Prints the instruction in CHUNK at offset OFFSET.
 * Returns the offset of the next instruction in CHUNK.
 */
int disassem_instruction(chunk_t *chunk, int offset) {
    printf("%04d ", offset);

    if (offset > 0 && chunk->lines[offset] == chunk->lines[offset - 1]) {
        printf("   | ");
    } else {
        printf("%4d ", chunk->lines[offset]);
    }

    uint8_t inst = chunk->data[offset];
    switch (inst)
    {
    case OP_RETURN:
        return simple_inst("OP_RETURN", offset);
    
    case OP_CONSTANT:
        return const_inst("OP_CONSTANT", chunk, offset);

    case OP_CONSTANT_LONG:
        return long_inst("OP_CONSTANT_LONG", chunk, offset);
    
    case OP_ADD:
        return simple_inst("OP_ADD", offset);

    case OP_SUB:
        return simple_inst("OP_SUB", offset);

    case OP_MUL:
        return simple_inst("OP_MUL", offset);

    case OP_DIV:
        return simple_inst("OP_DIV", offset);

    case OP_NEGATE:
        return simple_inst("OP_NEGATE", offset);

    default:
        printf("Unknown opcode %d\n", inst);
        return offset + 1;
    }
}

static int simple_inst(const char *name, int offset) {
    printf("%s\n", name);
    return offset + 1;
}

static int const_inst(const char *name, chunk_t *chunk, int offset) {
    uint8_t index = chunk->data[offset + 1];
    constval constant = chunk->constants.vals[index];

    printf("%-16s %4d '", name, index);
    printvalue(constant);
    printf("'\n");
    return offset + 2;
}

static int long_inst(const char *name, chunk_t *chunk, int offset) {
    uint32_t index = 0;
    index += chunk->data[offset + 1] << 16;
    index += chunk->data[offset + 2] << 8;
    index += chunk->data[offset + 3];

    constval constant = chunk->constants.vals[index];

    printf("%-16s %4d '", name, index);
    printvalue(constant);
    printf("'\n");
}

