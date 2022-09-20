#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define VEC_INIT_CAP (64)

typedef struct vec {
    void   *ptr;
    size_t  len;
    size_t  cap;
    size_t  memsize;
} vec;

vec *_make_vec(size_t memsize) {
    vec *v = malloc(sizeof *v);
    v->ptr      = malloc(VEC_INIT_CAP * memsize);
    v->len      = 0;
    v->cap      = VEC_INIT_CAP;
    v->memsize  = memsize;
    return v;
}

void freevec(vec *v) {
    free(v->ptr);
    free(v);
}

#define newvec(type) _make_vec(sizeof(type))

void push(vec *v, void *elem) {
    if (v->len >= v->cap) {
        v->cap = v->cap << 1;
        void *tmp = realloc(v->ptr, v->cap * v->memsize);
        if (tmp == NULL) {
            printf("error: could not reallocate when pushing to vec\n");
            exit(1);
        }
        v->ptr = tmp;
    }
    memcpy(v->ptr + v->len * v->memsize, elem, v->memsize);
    v->len++;
}

void pop(vec *v, void *elem) {
    v->len--;
    if (elem != NULL)
        memcpy(elem, v->ptr + v->len * v->memsize, v->memsize);
}

#include <sys/time.h>
#define LOOPS (1000000000)

void vectest() {
    vec *v = newvec(int);
    struct timeval start, stop;
    double secs = 0;

    for (int j = 0; j < 1; j++) {
    // VEC PUSH
    gettimeofday(&start, NULL);

    for (size_t i = 0; i < LOOPS; i++)
        push(v, &i);

    gettimeofday(&stop, NULL);
    secs = (double)(stop.tv_usec - start.tv_usec) / 1000000 + (double)(stop.tv_sec - start.tv_sec);
    printf("VEC PUSH\niterations:\t%d\ntime taken:\t%f\n", LOOPS, secs);

    // VEC POP
    gettimeofday(&start, NULL);

    for (size_t i = 0; i < LOOPS; i++)
        pop(v, NULL);

    gettimeofday(&stop, NULL);
    secs = (double)(stop.tv_usec - start.tv_usec) / 1000000 + (double)(stop.tv_sec - start.tv_sec);
    printf("VEC POP\niterations:\t%d\ntime taken:\t%f\n", LOOPS, secs);
    }
    // int stack[LOOPS];
    // // STACK PUSH
    // gettimeofday(&start, NULL);

    // for (size_t i = 0; i < LOOPS; i++)
    //     stack[i] = i;

    // gettimeofday(&stop, NULL);
    // secs = (double)(stop.tv_usec - start.tv_usec) / 1000000 + (double)(stop.tv_sec - start.tv_sec);
    // printf("STACK PUSH\niterations:\t%d\ntime taken:\t%f\n", LOOPS, secs);

    // // STACK POP
    // gettimeofday(&start, NULL);

    // for (size_t i = 0; i < LOOPS; i++)
    //     stack[i] = 0;

    // gettimeofday(&stop, NULL);
    // secs = (double)(stop.tv_usec - start.tv_usec) / 1000000 + (double)(stop.tv_sec - start.tv_sec);
    // printf("STACK POP\niterations:\t%d\ntime taken:\t%f\n", LOOPS, secs);

    freevec(v);
}

int main() {
    vectest();
}