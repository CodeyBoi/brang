#include <stdio.h>

#include "vm.h"
#include "chunk.h"
#include "debug.h"

vm_t vm;

void initvm() {
    resetstack();
}

static void resetstack() {
  vm.sp = vm.stack;
}

void freevm() {

}

vm_result interpret(chunk_t *chunk) {
    vm.chunk = chunk;
    vm.ip = vm.chunk->data;
    return run();
}

void push(constval value) {
    *vm.sp = value;
    vm.sp++;
}

constval pop() {
    vm.sp--;
    return *vm.sp;
}

static vm_result run() {
#define NEXT_BYTE() (*vm.ip++)
#define NEXT_CONSTANT() (vm.chunk->constants.vals[NEXT_BYTE()])
#define BIN_OP(op) \
    do { \
        constval b = pop(); \
        constval a = pop(); \
        push(a op b); \
    } while (0);
    

    for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
        printf("          ");
        for (constval *slot = vm.stack; slot < vm.sp; slot++) {
            printf("[ ");
            printvalue(*slot);
            printf(" ]");
        }
        printf("\n");
        disassem_instruction(vm.chunk, (int)(vm.ip - vm.chunk->data));
#endif
        uint8_t inst;
        switch (inst = NEXT_BYTE())
        {
        case OP_RETURN:
            printvalue(pop());
            printf("\n");
            return VM_OK;        
        
        case OP_CONSTANT: {
            constval constant = NEXT_CONSTANT();
            push(constant);
            break;
        }

        case OP_ADD: BIN_OP(+); break;

        case OP_SUB: BIN_OP(-); break;

        case OP_MUL: BIN_OP(*); break;

        case OP_DIV: BIN_OP(/); break;

        case OP_NEGATE:
            push(-pop());
            break;
        
        default:
            break;
        }
    }

#undef NEXT_BYTE
#undef NEXT_CONSTANT
#undef BIN_OP
}