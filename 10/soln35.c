#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define TWO_MILLION 2000000

bool *primes_below(int, int*);

int main() {
    int size;
    bool *primes = primes_below(TWO_MILLION, &size);

    unsigned long long sum = 0;
    for(int i = 0; i < size; i++) {
        if(primes[i]) sum += (i + 2);
    }
    
    printf("%lld\n", sum);

    free(primes);

    return 0;
}

bool *primes_below(int n, int* arr_size) {
    *arr_size = n - 2;
    bool *primes = (bool *)malloc(sizeof(bool) * (*arr_size));
    int root = (int) sqrt(n);

    for(int i = 0; i < *arr_size; i++) {
        primes[i] = true;
    }

    for(int i = 0; i < root; i++) {
        if(primes[i]) {
            for(int j = (i + 2) * (i + 2); j < n; j += (i + 2)) {
                primes[j - 2] = false;
            }
        }
    }

    return primes;
}
