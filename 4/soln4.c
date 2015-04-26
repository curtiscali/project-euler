#include <stdbool.h>
#include <stdio.h>

int reversed(int);
bool is_palindrome(int n) {return n == reversed(n);}

int main() {
    int largest_product = 0;
    for(int i = 500; i < 1000; i++) {
        for(int j = 500; j < 1000; j++) {
            if(i * j > largest_product && is_palindrome(i * j)) {
                largest_product = i * j;
            }
        }
    }

    printf("%d\n", largest_product);
}

int reversed(int n) {
    int rev = 0;
    while(n > 0) {
        rev *= 10;
        rev += n % 10;
        n /= 10;
    }

    return rev;
}
