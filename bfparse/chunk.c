#include "chunk.h"
#include "darray.h"

#define MAX_VAR_COUNT (1 << 24)

void initchunk(chunk_t *chk) {
    chk->len = 0;
    chk->cap = 0;
    chk->data = NULL;
    chk->lines = NULL;
    initconstarray(&chk->constants);
}

void freechunk(chunk_t *chk) {
    FREE_ARRAY(uint8_t, chk->data, chk->cap);
    FREE_ARRAY(int, chk->lines, chk->cap);
    freeconstarray(&chk->constants);
    initchunk(chk);
}

/**
 * @brief Writes a byte to CHK.
 */
void writechunk(chunk_t *chk, uint8_t byte, int line) {
    if (chk->len == chk->cap) {
        size_t ocap = chk->cap;
        chk->cap = GROW_CAP(chk->cap);
        chk->data = GROW_ARRAY(uint8_t, chk->data, ocap, chk->cap);
        chk->lines = GROW_ARRAY(int, chk->lines, ocap, chk->cap);
    }
    chk->data[chk->len] = byte;
    chk->lines[chk->len] = line;
    chk->len++;
}

/**
 * @brief Adds a constant of type CONSTVAL to CHK. 
 * Returns index of its position in memory.
 */
int addconstant(chunk_t *chk, constval value) {
    writeconstarray(&chk->constants, value);
    return chk->constants.len - 1;
}
