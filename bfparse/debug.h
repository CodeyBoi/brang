#ifndef brang_debug_h
#define brang_debug_h

#define DEBUG_TRACE_EXECUTION

#include "chunk.h"
#include "scanner.h"

void disassem_chunk(chunk_t*, const char *name);
int disassem_instruction(chunk_t*, int offset);

void printtoken(token_t);

static int simple_inst(const char* name, int offset);
static int const_inst(const char* name, chunk_t *chunk, int offset);
static int long_inst(const char *name, chunk_t *chunk, int offset);

#endif
