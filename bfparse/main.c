#include <stdio.h>

#include "chunk.h"
#include "vm.h"
#include "debug.h"
#include "scanner.h"

int main(int argc, char *argv[]) {
    token_t t;
    char *buf;
    FILE *f;
    size_t len;

    if (argc < 2) {
        printf("Usage: brang <file>\n");
        exit(1);
    }

    f = fopen(argv[1], "rb");
    fseek(f, 0, SEEK_END);
    len = ftell(f);
    rewind(f);
    buf = malloc(len + 1);
    buf[len] = 0;
    fread(buf, sizeof *buf, len, f);
    initscanner(buf);

    t = scantoken();
    while (t.type != STOP) {
        printtoken(t);
        t = scantoken();
    }
    printtoken(t);

    free(buf);
}

int main2(int argc, char *argv[]) {
    chunk_t chunk;

    initvm();

    initchunk(&chunk);

    int constant = addconstant(&chunk, 37);
    writechunk(&chunk, OP_CONSTANT, 1);
    writechunk(&chunk, constant, 1);
    writechunk(&chunk, OP_NEGATE, 1);

    writechunk(&chunk, OP_RETURN, 2);
    disassem_chunk(&chunk, "test chunk");
    interpret(&chunk);

    freevm();
    freechunk(&chunk);
}