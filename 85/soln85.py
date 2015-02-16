#!/usr/bin/python3

def num_rectangles_in_grid(m, n):
    return (m*(m + 1) * n*(n + 1)) / 4

def main():
    m,n,min_diff = 0,0,2000000
    for x in range(1, 1000):
        for y in range(x + 1, 1000):
            if num_rectangles_in_grid(x, y) < 2000000:
                diff = 2000000 - num_rectangles_in_grid(x, y)
                if diff < min_diff:
                    min_diff = diff
                    m,n = x,y
    print(m*n)

if __name__ == '__main__':
    main()
