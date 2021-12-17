void mov(int** ptr, int);
void movto(int** ptr, int);
void setval(int);

int main() {

    int* ptr;
    int* stackptr;

}

/**
 * @brief Moves the pointer relatively to its current position 'k' steps, 
 * negative values correspond with steps to the left.
 * 
 * @param ptr a pointer to the pointer
 * @param k the number of steps to move
 */
void mov(int** ptr, int k) {
    int i;
    for (i = 0; i < k; i++)
        printf('>');
    for (i = 0; i < -k; i++)
        printf('<');
    *ptr += k;
}       

/**
 * @brief Moves the pointer to the address k.
 */
void movto(int** ptr, int k) {
    for (; *ptr < k; ++*ptr)
        printf('>');
    for (; *ptr > k; --*ptr)
        printf('<');
}