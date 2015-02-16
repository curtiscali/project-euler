#!/usr/bin/python3

def combo(n, k):
    combinations = 1
    for i in range(k):
        combinations *= ((n - i) / (i + 1))
    return int(combinations)

if __name__ == '__main__':
    print(combo(40, 20))
