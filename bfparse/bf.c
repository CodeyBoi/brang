#include "bf.h"

bfassem_t bfa;

void initassembler(FILE *f)
{
    bfa.ptr = 0;
    bfa.sp = 0;
    bfa.written = 0;
    bfa.f = f;
}

static void writec(char c)
{
    putc(c, bfa.f);
    if (++bfa.written % OUTPUT_WIDTH == 0)
        putc('\n', bfa.f);
}

static void writestr(const char *str)
{
    while (*str != '\0')
        writec(*str++);
}

/**
 * @brief Allocates SIZE slots of memory on stack.
 * @return The address of the first memory slot allocated.
 */
static int bfalloc(int size)
{
    int ptr = bfa.sp;
    for (int i = size * sizeof(NUMTYPE) - 1; i >= 0; i--) {
        mov(bfa.sp + i);
        setval(0);
    }
    // mov(bfa.sp);
    bfa.sp += size * sizeof(NUMTYPE);
    return ptr;
}

static void bffree(int size)
{
    bfa.sp -= size * sizeof(NUMTYPE);
}

void pushnum(NUMTYPE val)
{
    mov(bfalloc(1));
    setval(val);
}

/**
 * @brief Pushes the value of the variable at ADR onto the stack.
 */
void loadvar(int adr)
{
    cpy(adr, bfalloc(1));
}

/**
 * @brief Returns address to the value at the top of the stack and 
 * deallocates its associated memory.
 */
static int pop()
{
    bffree(1);
    return bfa.sp;
}

/**
 * @brief Move BF pointer head to ADR.
 */
static void mov(int adr)
{
    if (adr > bfa.ptr) {
        for (int i = 0; i < adr - bfa.ptr; i++)
            writec('>');
    } else {
        for (int i = 0; i < bfa.ptr - adr; i++)
            writec('<');
    }
    bfa.ptr = adr;
}

/**
 * @brief Set value at pointer head to VAL.
 */
static void setval(NUMTYPE val)
{
    writestr("[-]");
    for (int i = 0; i < val; i++)
        writec('+');
}

/**
 * @brief Copies a value from PTR to the first free slot.
 * Returns the address that was written to.
 */
static void cpy(int src, int dst)
{
    int tmp = bfalloc(1);

    // move value from src to dst and tmp
    mov(src);
    writestr("[-");
    mov(dst);
    writec('+');
    mov(tmp);
    writec('+');
    mov(src);
    writec(']');

    // move value back from tmp to src
    mov(tmp);
    writestr("[-");
    mov(src);
    writec('+');
    mov(tmp);
    writec(']');

    bffree(1);
}

void add()
{
    mov(pop());
    writestr("[-<+>]");
}

void sub()
{
    mov(pop());
    writestr("[-<->]");
}

void mul()
{
    int lhs = bfa.sp - 2;
    int rhs = bfa.sp - 1;
    int tmp = bfalloc(1);
    cpy(rhs, tmp);
    mov(lhs);
    writestr("[-[-");

    // add tmp to rhs
    int tmp2 = bfalloc(1);
    cpy(tmp, tmp2);
    mov(tmp2);
    writestr("[-");
    mov(rhs);
    writec('+');
    mov(tmp2);
    writec(']');
    bffree(1);

    mov(lhs);
    writec(']');
    add();
    mov(lhs);
    writec(']');

    bffree(1);
    pop();
}