#ifndef brang_table_h
#define brang_table_h

#include <stdbool.h>

#include "bf.h"

#define BF_PTR int

typedef struct {
    char   *key;
    BF_PTR  value;
    size_t  hash;
} table_e;

typedef struct {
    int len;
    int cap;
    table_e *entries;
} table_t;

static size_t strhash(const char *str);
void inittable(table_t *table);
void freetable(table_t *table);

table_e *find(table_e *entries, int cap, const char *key);
bool insert(table_t *table, const char* key, int value);
bool retrieve(table_t *table, const char *key, BF_PTR* value);
bool remove(table_t *table, const char *key);
void addall(table_t *src, table_t *dst);


#endif