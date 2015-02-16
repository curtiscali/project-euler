def sum_quadratic(n):
    quadratic_sum = (n*(n + 1)*(2*n + 1)) / 6
    return int(quadratic_sum)

def sum_linear(n):
    linear_sum = (n*(n + 1)) / 2
    return int(linear_sum)

def main():
    NUM_ITERATIONS = 500000000

    quadratic_sum = 16 * sum_quadratic(NUM_ITERATIONS)
    linear_sum = 4 * sum_linear(NUM_ITERATIONS)

    print(quadratic_sum + linear_sum + 2001)

if __name__ == '__main__':
    main()
