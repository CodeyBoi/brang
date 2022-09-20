#ifndef brang_bf_h
#define brang_bf_h

#include <stdio.h>
#include <stdint.h>

#include "chunk.h"

#define NUMTYPE uint8_t
#define OUTPUT_WIDTH (50)

typedef struct {
    int     ptr;    // the BF pointer head
    int     sp;     // points to the first free spot on the stack
    size_t  written;
    FILE   *f;      
} bfassem_t;

void initassembler(FILE *f);
void pushnum(NUMTYPE val);
void loadvar(int adr);
void add();
void sub();
void mul();

static void writec(char c);
static void writestr(const char *str);
static int bfalloc(int size);
static void bffree(int size);
static int pop();
static void mov(int adr);
static void setval(NUMTYPE val);
static void cpy(int src, int dst);

#endif