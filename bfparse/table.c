#include <string.h>

#include "table.h"
#include "darray.h"
#include "scanner.h"

#define TABLE_MAX_LOAD  (0.75)
#define NIL_VAL         (-1)
#define TOMBSTONE_VAL   (-2)

/**
 * @brief A simple hash function for null-terminated strings.
 */
static size_t strhash(const char *str)
{
    size_t  hash = 5381;
    int     c;

    while (c = *str++)
        hash = ((hash << 5) + hash) + c;

    return hash;
}

void inittable(table_t *table)
{
    table->len = 0;
    table->cap = 0;
    table->entries = NULL;
}

void freetable(table_t *table)
{
    FREE_ARRAY(table_e, table->entries, table->cap);
    inittable(table);
}

/**
 * @brief Finds an entry in the table using the null-terminated string KEY.
 * @return The entry, or an empty entry if it doesn't exist.
 */
table_e *find(table_e *entries, int cap, const char *key)
{
    size_t hash = strhash(key);
    size_t index = hash % cap;
    for (;;) {
        table_e *entry = &entries[index];
        table_e *tombstone = NULL;

        // Return the entry if it's empty, or if it's the one we're looking for
        if (entry->key == NULL) {
            // This entry is either empty or is a tombstone (a deleted entry)
            if (entry->value == NIL_VAL) {
                // This entry is empty
                // Return tombstone if we found one earlier, otherwise return empty entry
                entry = tombstone != NULL ? tombstone : entry;
                entry->hash = hash;
                return entry;
            } else if (entry->value == TOMBSTONE_VAL) {
                // This entry is a tombstone, mark it for later
                if (tombstone == NULL)
                    tombstone = entry;
            } else {
                fprintf(stderr, "Error: invalid entry value %d\n", entry->value);
            }
        } else if (strcmp(entry->key, key) == 0) {
            return entry;
        }

        index = (index + 1) % cap;
    }
}

/**
 * @brief Changes TABLES capacity to CAP. 
 */
static void resizetable(table_t *table, int cap)
{
    // Allocate new table and initialize values
    table_e *entries = ALLOC_ARRAY(table_e, cap);
    for (table_e *e = entries; e < entries + cap; e++) {
        e->key = NULL;
        e->value = NIL_VAL;
        e->hash = 0;
    }

    // Copy old entries into new table
    for (int i = 0; i < table->cap; i++) {
        table_e *old_e = &table->entries[i];
        if (old_e->key != NULL) {
            table_e *dst = find(entries, cap, old_e->key);
            dst->key = old_e->key;
            dst->value = old_e->value;
            table->len++;
        }
    }

    // Free old table
    FREE_ARRAY(table_e, table->entries, table->cap);

    // Update table
    table->entries = entries;
    table->cap = cap;
    table->len = 0;
}

/**
 * @brief Inserts an entry into the table.
 * @return True if the entry was inserted, false if it already existed.
 */
bool insert(table_t *table, const char* key, int value)
{
    if (table->len + 1 > table->cap * TABLE_MAX_LOAD)
        resizetable(table, GROW_CAP(table->cap));

    table_e *entry = find(table->entries, table->cap, key);
    bool isnew = entry->key == NULL && entry->value == NIL_VAL;
    if (isnew)
        table->len++;

    entry->key = key;
    entry->value = value;

    return isnew;
}

/**
 * @brief Retrieves the value associated with KEY and stores it in VALUE if it exists.
 * @return True if the value was found, false if it didn't exist.
 */
bool retrieve(table_t *table, const char *key, BF_PTR* value)
{
    if (table->len == 0)
        return false;
    
    table_e *entry = find(table->entries, table->cap, key);
    if (entry->key == NULL)
        return false;

    *value = entry->value;
    return true;
}

/**
 * @brief Removes the entry associated with KEY.
 * @return True if the entry was removed, false if it didn't exist.
 * @note If the entry was removed, its value is set to TOMBSTONE_VAL.
 */
bool remove(table_t *table, const char *key)
{
    if (table->len == 0)
        return false;

    table_e *entry = find(table->entries, table->cap, key);
    if (entry->key == NULL)
        return false;

    entry->key = NULL;
    entry->value = TOMBSTONE_VAL;
    return true;
}

/**
 * @brief Adds all entries from SRC to DST.
 * If a duplicate key is found, the value is overwritten.
 */
void addall(table_t *src, table_t *dst)
{
    for (table_e *e = src->entries; e < src->entries + src->cap; e++) {
        if (e->key != NULL)
            insert(dst, e->key, e->value);
    }
}

/**
 * @brief Null-terminates a string and returns it.
 * 
 * @param buf The buffer to store the string in.
 * @param str The string to be null-terminated.
 * @param len The length of the string.
 */
char *null_terminate(char *buf, char *str, size_t len)
{
    strncpy(buf, str, len);
    buf[len] = '\0';
    return buf;
}
