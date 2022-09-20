#include "darray.h"

void *_resize_array(void* ptr, size_t osize, size_t nsize) {
    if (osize != 0 && nsize == 0) {
        free(ptr);
        return NULL;
    }

    void *result = realloc(ptr, nsize);
    if (result == NULL)
        exit(1);

    return result;
}
