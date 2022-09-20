#include <stdio.h>

#include "constval.h"
#include "darray.h"

void initconstarray(constarray *ims) {
    ims->len = 0;
    ims->cap = 0;
    ims->vals = NULL;
}

void freeconstarray(constarray *ims) {
    FREE_ARRAY(constval, ims->vals, ims->cap);
    initconstarray(ims);
}

void writeconstarray(constarray *ims, constval value) {
    if (ims->len == ims->cap) {
        size_t ocap = ims->cap;
        ims->cap = GROW_CAP(ims->cap);
        ims->vals = GROW_ARRAY(constval, ims->vals, ocap, ims->cap);
    }
    ims->vals[ims->len++] = value;
}

void printvalue(constval value) {
    printf("%ld", value);
}