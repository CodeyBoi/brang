#ifndef brang_immed_h
#define brang_immed_h

#include <stdint.h>
#include <stdlib.h>

typedef uint64_t constval;

typedef struct {
    size_t      len;
    size_t      cap;
    constval   *vals;
} constarray;

void initconstarray(constarray*);
void freeconstarray(constarray*);
void writeconstarray(constarray*, constval value);

void printvalue(constval);

#endif
