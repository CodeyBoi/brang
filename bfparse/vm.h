#ifndef brang_vm_h
#define brang_vm_h

#include "chunk.h"
#include "constval.h"

#define STACK_MAX (256)

typedef struct {
    chunk_t  *chunk;
    uint8_t  *ip;
    constval  stack[STACK_MAX];
    constval *sp;
} vm_t;

typedef enum {
    VM_OK,
    VM_COMPILE_ERROR,
    VM_RUNTIME_ERROR
} vm_result;

void initvm();
void freevm();
vm_result interpret(chunk_t*);

static void resetstack();
void push(constval value);
constval pop();

static vm_result run();

#endif
