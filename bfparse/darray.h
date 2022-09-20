#ifndef darray_h
#define darray_h

#include <stdlib.h>

#define ARRAY_GROW_FACTOR (2)

void *_resize_array(void*, size_t osize, size_t nsize);

#define GROW_CAP(cap) \
    ((cap) < 8 ? 8 : (cap) * ARRAY_GROW_FACTOR)

#define GROW_ARRAY(type, ptr, osize, nsize) \
    (type*)_resize_array(ptr, osize * sizeof(type), \
        nsize * sizeof(type))

#define FREE_ARRAY(type, ptr, osize) \
    _resize_array(ptr, (osize) * sizeof(type), 0)

#define ALLOC_ARRAY(type, n) \
    GROW_ARRAY(type, NULL, 0, n)

#endif
